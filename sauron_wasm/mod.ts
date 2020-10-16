import * as wasm from "./wasm.js";

interface Entry {
  path: string;
  data: string | Entry[];
}

await wasm.init(wasm.source);
wasm.init_panic_hook();

export function sauronCheck(file_tree: Entry & { data: Entry[] }) {
  let res = wasm.sauron_check(file_tree);
  if (res !== undefined) {
    return JSON.parse(res);
  }
}

console.log(
  sauronCheck(
    {
      path: "root",
      data: [{ path: "root/README.md", data: "# Readme content" }],
    },
  ),
);
