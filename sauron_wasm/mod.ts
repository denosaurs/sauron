import { sauron_check } from "./wasm.js";

export interface Entry {
  path: string;
  data: string | Entry[];
}

export function check(file_tree: Entry & { data: Entry[] }) {
  const res = sauron_check(JSON.stringify(file_tree));
  
  if (res !== undefined) {
    return JSON.parse(res);
  }
}
