import * as wasm from "./wasm.js";

export interface Entry {
  path: string;
  data: string | Entry[];
}

await wasm.init(wasm.source);
wasm.init_panic_hook();

export function check(file_tree: Entry & { data: Entry[] }) {
  const res = wasm.sauron_check(file_tree);
  if (res !== undefined) {
    return JSON.parse(res);
  }
}
