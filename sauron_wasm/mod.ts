import * as wasm from "./wasm.js";

interface Entry {
  path: string;
  data: string | Entry[];
}

await wasm.init(wasm.source);
wasm.init_panic_hook();

export function sauronCheck(file_tree: Entry & { data: Entry[] }) {
  const ret = JSON.parse(wasm.sauron_check(file_tree));
  return ret;
}

console.log(
  sauronCheck(
    {
      path: "root",
      data: [{ path: "root/README.md", data: "# Readme content" }],
    },
  ),
);
