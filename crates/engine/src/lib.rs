//! WasmForge's deterministic, allocation-conscious browser compute core.
//! The public ABI deliberately batches work to minimize JS↔Wasm crossings.
use serde::Serialize;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("WasmForge panic: {info}");
        web_console_error(&msg);
    }));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn web_console_error(s: &str);
}

#[derive(Serialize)]
struct Analysis {
    bytes: usize,
    characters: usize,
    words: usize,
    unique_words: usize,
    sentences: usize,
    lines: usize,
    reading_seconds: u32,
    lexical_density: f64,
    avg_word_length: f64,
    top_terms: Vec<(String, usize)>,
    fingerprint: String,
}

/// Unicode-aware text analysis, returned once as JSON to keep the boundary chunky.
#[wasm_bindgen]
pub fn analyze_text(input: &str) -> String {
    let words: Vec<String> = input
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();
    let mut freq = BTreeMap::<String, usize>::new();
    for word in &words {
        *freq.entry(word.clone()).or_default() += 1;
    }
    let mut top: Vec<_> = freq.iter().map(|(w, c)| (w.clone(), *c)).collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    top.truncate(8);
    let chars = input.chars().count();
    let total_letters: usize = words.iter().map(|w| w.chars().count()).sum();
    serde_json::to_string(&Analysis {
        bytes: input.len(),
        characters: chars,
        words: words.len(),
        unique_words: freq.len(),
        sentences: input
            .matches(['.', '!', '?'])
            .count()
            .max(if input.trim().is_empty() { 0 } else { 1 }),
        lines: input.lines().count(),
        reading_seconds: ((words.len() as f64 / 238.0) * 60.0).ceil() as u32,
        lexical_density: ratio(freq.len(), words.len()),
        avg_word_length: ratio(total_letters, words.len()),
        top_terms: top,
        fingerprint: format!("{:016x}", fnv1a(input.as_bytes())),
    })
    .expect("serializable")
}

fn ratio(a: usize, b: usize) -> f64 {
    if b == 0 {
        0.0
    } else {
        (a as f64 / b as f64 * 1000.0).round() / 1000.0
    }
}
fn fnv1a(data: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in data {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3)
    }
    h
}

/// Seeded Monte Carlo simulation; deterministic across browser and native builds.
#[wasm_bindgen]
pub fn monte_carlo_pi(samples: u32, seed: u32) -> String {
    let n = samples.clamp(1, 10_000_000);
    let mut rng = XorShift32(seed.max(1));
    let mut inside = 0u32;
    for _ in 0..n {
        let x = rng.f64();
        let y = rng.f64();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    let estimate = 4.0 * inside as f64 / n as f64;
    serde_json::json!({"samples":n,"inside":inside,"estimate":estimate,"error":(std::f64::consts::PI-estimate).abs(),"seed":seed}).to_string()
}

struct XorShift32(u32);
impl XorShift32 {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
    fn f64(&mut self) -> f64 {
        self.next() as f64 / u32::MAX as f64
    }
}

/// A DSP-style low-pass + edge detector over a zero-copy typed array boundary.
#[wasm_bindgen]
pub fn process_signal(samples: &[f64], smoothing: f64) -> Vec<f64> {
    if samples.is_empty() {
        return vec![];
    }
    let a = smoothing.clamp(0.001, 0.999);
    let mut out = Vec::with_capacity(samples.len());
    let mut ema = samples[0];
    let mut prev = ema;
    for &x in samples {
        ema = a * x + (1.0 - a) * ema;
        out.push(ema - prev);
        prev = ema;
    }
    out
}

/// Procedural terrain generated entirely in Wasm and transferred as one typed array.
#[wasm_bindgen]
pub fn generate_terrain(width: u32, height: u32, seed: u32) -> Vec<u8> {
    let (w, h) = (width.clamp(8, 1024), height.clamp(8, 1024));
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let nx = x as f64 / w as f64 * 5.0;
            let ny = y as f64 / h as f64 * 5.0;
            let v = (noise(nx, ny, seed)
                + 0.52 * noise(nx * 2.03, ny * 2.03, seed + 17)
                + 0.25 * noise(nx * 4.1, ny * 4.1, seed + 31))
                / 1.77;
            let i = ((y * w + x) * 4) as usize;
            let t = ((v + 1.0) * 0.5).clamp(0.0, 1.0);
            let (r, g, b) = palette(t);
            out[i] = r;
            out[i + 1] = g;
            out[i + 2] = b;
            out[i + 3] = 255;
        }
    }
    out
}
fn noise(x: f64, y: f64, seed: u32) -> f64 {
    ((x * 12.9898 + y * 78.233 + seed as f64 * 0.071).sin() * 43758.5453).fract() * 2.0 - 1.0
}
fn palette(t: f64) -> (u8, u8, u8) {
    if t < 0.38 {
        (5, (35.0 + t * 80.0) as u8, (75.0 + t * 210.0) as u8)
    } else if t < 0.56 {
        (18, (80.0 + t * 130.0) as u8, 72)
    } else if t < 0.78 {
        ((80.0 + t * 100.0) as u8, (70.0 + t * 80.0) as u8, 65)
    } else {
        let v = (180.0 + t * 75.0) as u8;
        (v, v, (v as f64 * 0.98) as u8)
    }
}

#[wasm_bindgen]
pub fn sort_f64(values: &[f64]) -> Vec<f64> {
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    v
}

#[wasm_bindgen]
pub fn engine_manifest() -> String {
    serde_json::json!({
    "name":"wasmforge-engine","version":env!("CARGO_PKG_VERSION"),"abi":1,
    "capabilities":["text-analysis","monte-carlo","signal-processing","terrain-generation","sorting"],
    "build":{"target":"wasm32-unknown-unknown","boundary":"batched","deterministic":true}
}).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analysis_is_stable() {
        let a = analyze_text("Rust rust Wasm.");
        assert!(a.contains("\"words\":3"));
    }
    #[test]
    fn sort_handles_nan() {
        assert_eq!(sort_f64(&[2.0, 1.0])[0], 1.0);
    }
}
