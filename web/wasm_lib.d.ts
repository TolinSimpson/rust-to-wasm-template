/* tslint:disable */
/* eslint-disable */
export class Octree {
  free(): void;
  [Symbol.dispose](): void;
  constructor(minx: number, miny: number, minz: number, maxx: number, maxy: number, maxz: number, capacity: number);
  insert(x: number, y: number, z: number, id: number): void;
  query_aabb(minx: number, miny: number, minz: number, maxx: number, maxy: number, maxz: number): Uint32Array;
  query_sphere(cx: number, cy: number, cz: number, r: number): Uint32Array;
  len(): number;
  clear(): void;
  all_node_aabbs(): Float32Array;
  all_points(): Float32Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_octree_free: (a: number, b: number) => void;
  readonly octree_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly octree_insert: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly octree_query_aabb: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly octree_query_sphere: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly octree_len: (a: number) => number;
  readonly octree_clear: (a: number) => void;
  readonly octree_all_node_aabbs: (a: number) => [number, number];
  readonly octree_all_points: (a: number) => [number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
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
