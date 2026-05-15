/* tslint:disable */
/* eslint-disable */

export class HonzoWasm {
    free(): void;
    [Symbol.dispose](): void;
    chunk_count(): number;
    get_chunk(index: number): Uint8Array;
    get_meta(): Uint8Array;
    get_meta_parsed(): any;
    get_toc(): any;
    has_drm(): boolean;
    has_sidx(): boolean;
    layout_mode(): number;
    constructor(buf: Uint8Array, reader_version: number);
}

export function honzo_build(spec: any): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_honzowasm_free: (a: number, b: number) => void;
    readonly honzo_build: (a: any) => [number, number, number, number];
    readonly honzowasm_chunk_count: (a: number) => number;
    readonly honzowasm_get_chunk: (a: number, b: number) => [number, number, number, number];
    readonly honzowasm_get_meta: (a: number) => [number, number, number, number];
    readonly honzowasm_get_meta_parsed: (a: number) => [number, number, number];
    readonly honzowasm_get_toc: (a: number) => [number, number, number];
    readonly honzowasm_has_drm: (a: number) => number;
    readonly honzowasm_has_sidx: (a: number) => number;
    readonly honzowasm_layout_mode: (a: number) => number;
    readonly honzowasm_new: (a: number, b: number, c: number) => [number, number, number];
    readonly rust_zstd_wasm_shim_calloc: (a: number, b: number) => number;
    readonly rust_zstd_wasm_shim_free: (a: number) => void;
    readonly rust_zstd_wasm_shim_malloc: (a: number) => number;
    readonly rust_zstd_wasm_shim_memcmp: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memmove: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memset: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_qsort: (a: number, b: number, c: number, d: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
