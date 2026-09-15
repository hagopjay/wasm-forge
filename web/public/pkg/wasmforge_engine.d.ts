/* tslint:disable */
/* eslint-disable */

/**
 * Unicode-aware text analysis, returned once as JSON to keep the boundary chunky.
 */
export function analyze_text(input: string): string;

export function engine_manifest(): string;

/**
 * Procedural terrain generated entirely in Wasm and transferred as one typed array.
 */
export function generate_terrain(width: number, height: number, seed: number): Uint8Array;

/**
 * Seeded Monte Carlo simulation; deterministic across browser and native builds.
 */
export function monte_carlo_pi(samples: number, seed: number): string;

/**
 * A DSP-style low-pass + edge detector over a zero-copy typed array boundary.
 */
export function process_signal(samples: Float64Array, smoothing: number): Float64Array;

export function sort_f64(values: Float64Array): Float64Array;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly analyze_text: (a: number, b: number, c: number) => void;
    readonly engine_manifest: (a: number) => void;
    readonly generate_terrain: (a: number, b: number, c: number, d: number) => void;
    readonly monte_carlo_pi: (a: number, b: number, c: number) => void;
    readonly process_signal: (a: number, b: number, c: number, d: number) => void;
    readonly sort_f64: (a: number, b: number, c: number) => void;
    readonly start: () => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
