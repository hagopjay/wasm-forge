use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderValue, StatusCode},
    response::Response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use sysinfo::System;
use tokio::sync::RwLock;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    started: Instant,
    jobs: Arc<RwLock<Vec<Job>>>,
}
#[derive(Clone, Serialize)]
struct Job {
    id: Uuid,
    kind: String,
    status: String,
    progress: u8,
    created_ms: u128,
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
    version: &'static str,
    uptime_ms: u128,
    runtime: &'static str,
    wasm_boundary: &'static str,
}
#[derive(Deserialize)]
struct JobRequest {
    kind: String,
    #[serde(default)]
    payload: serde_json::Value,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wasmforge_server=info,tower_http=info".into()),
        )
        .json()
        .init();
    let state = AppState {
        started: Instant::now(),
        jobs: Default::default(),
    };
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".into());
    let index = PathBuf::from(&static_dir).join("index.html");
    let api = Router::new()
        .route("/health", get(health))
        .route("/system", get(system))
        .route("/jobs", get(jobs).post(create_job))
        .route("/echo", post(echo))
        .route("/stream", get(ws_handler));
    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(&static_dir).not_found_service(ServeFile::new(index)))
        .with_state(state)
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .layer(CorsLayer::permissive());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind server");
    info!(%addr,"WasmForge online");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("server");
}
async fn health(State(s): State<AppState>) -> Json<Health> {
    Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_ms: s.started.elapsed().as_millis(),
        runtime: "Rust/Axum on WSL",
        wasm_boundary: "browser compute core",
    })
}
async fn system() -> Json<serde_json::Value> {
    let mut s = System::new_all();
    s.refresh_all();
    Json(
        serde_json::json!({"host":System::host_name().unwrap_or_else(||"WSL".into()),"os":System::long_os_version(),"cpus":s.cpus().len(),"memory_mb":s.total_memory()/1024/1024,"architecture":std::env::consts::ARCH,"rust":true}),
    )
}
async fn jobs(State(s): State<AppState>) -> Json<Vec<Job>> {
    Json(s.jobs.read().await.clone())
}
async fn create_job(
    State(s): State<AppState>,
    Json(req): Json<JobRequest>,
) -> Result<(StatusCode, Json<Job>), (StatusCode, String)> {
    if req.kind.len() > 64 {
        return Err((StatusCode::BAD_REQUEST, "kind too long".into()));
    }
    let _payload_bytes = req.payload.to_string().len();
    let job = Job {
        id: Uuid::new_v4(),
        kind: req.kind,
        status: "queued".into(),
        progress: 0,
        created_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    };
    s.jobs.write().await.push(job.clone());
    let st = s.clone();
    let id = job.id;
    tokio::spawn(async move {
        for p in [12, 31, 57, 81, 100] {
            tokio::time::sleep(Duration::from_millis(280)).await;
            if let Some(j) = st.jobs.write().await.iter_mut().find(|j| j.id == id) {
                j.progress = p;
                j.status = if p == 100 { "complete" } else { "running" }.into();
            }
        }
    });
    Ok((StatusCode::ACCEPTED, Json(job)))
}
async fn echo(Json(v): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(
        serde_json::json!({"echo":v,"handled_by":"native Rust","at_ms":SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}),
    )
}
async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(socket)
}
async fn socket(mut ws: WebSocket) {
    let start = Instant::now();
    let mut tick = tokio::time::interval(Duration::from_millis(900));
    loop {
        tokio::select! {_=tick.tick()=>{let msg=serde_json::json!({"type":"telemetry","uptime":start.elapsed().as_secs_f32(),"load":((start.elapsed().as_millis()%700)as f32/1000.0)+0.12,"at":SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}).to_string();if ws.send(Message::Text(msg.into())).await.is_err(){break}},incoming=ws.recv()=>match incoming{Some(Ok(Message::Text(t)))=>{if ws.send(Message::Text(format!("{{\"type\":\"ack\",\"bytes\":{}}}",t.len()).into())).await.is_err(){break}},Some(Ok(Message::Close(_)))|None=>break,Some(Err(e))=>{warn!(%e,"websocket");break},_=>{}}}
    }
}
async fn shutdown() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("ctrl-c") };
    #[cfg(unix)]
    let term = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! {_=ctrl_c=>{},_=term=>{}}
    info!("graceful shutdown");
}
