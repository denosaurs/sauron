import { encode } from "https://deno.land/std@0.74.0/encoding/base64.ts";
import { compress } from "https://deno.land/x/lz4@v0.1.2/mod.ts";
import { minify } from "https://jspm.dev/terser@5.3.4";

const name = "sauron_wasm";

const encoder = new TextEncoder();

async function requires(...executables: string[]) {
  const where = Deno.build.os === "windows" ? "where" : "which";

  for (const executable of executables) {
    const process = Deno.run({
      cmd: [where, executable],
      stderr: "null",
      stdin: "null",
      stdout: "null",
    });

    if (!(await process.status()).success) {
      console.error(`Could not find required build tool ${executable}`);
    }
  }
}

async function run(msg: string, cmd: string[]) {
  console.log(msg);

  const process = Deno.run({
    cmd,
    stderr: "inherit",
    stdin: "null",
    stdout: "null",
  });

  if (!(await process.status()).success) {
    console.error(`${msg} failed`);
  }
}

await requires("rustup", "rustc", "cargo");

if (!(await Deno.stat("Cargo.toml")).isFile) {
  console.error(`the build script should be executed in the "${name}" root`);
}

await run(
  "building using cargo wasi",
  ["cargo", "wasi", "build", "--release"],
);

await run(
  "generating bindings using wasm-bindgen",
  [
    "wasm-bindgen",
    `target/wasm32-wasi/release/${name}.wasm`,
    "--out-dir",
    "pkg",
    "--target",
    "deno",
  ],
);

const wasm = await Deno.readFile(`pkg/${name}_bg.wasm`);
const compressed = compress(wasm);
console.log(
  `compressed wasm using lz4, size reduction: ${wasm.length -
    compressed.length} bytes`,
);
const encoded = encode(compressed);
console.log(
  `encoded wasm using base64, size increase: ${encoded.length -
    compressed.length} bytes`,
);

console.log("inlining wasm in js");
const source = `import * as lz4 from "https://deno.land/x/lz4@v0.1.2/mod.ts";
                import Context from "https://deno.land/std@0.74.0/wasi/snapshot_preview1.ts";
                const source = lz4.decompress(Uint8Array.from(atob("${encoded}"), c => c.charCodeAt(0)));

                const context = new Context({});
                const imports = {
                  __wbindgen_placeholder__: {},
                  wasi_snapshot_preview1: context.exports
                };
                
                ${
                  (await Deno.readTextFile(`pkg/${name}.js`))
                  .replace(/import \* as import\d from 'wasi_snapshot_preview1'/g, "")
                  .replace(/const imports = {(?:\n|.)*};/, "")
                  .replace("const file = new URL(import.meta.url).pathname;", "")
                  .replace("const wasmFile = file.substring(0, file.lastIndexOf(Deno.build.os === 'windows' ? '\\\\' : '/') + 1) + 'sauron_wasm_bg.wasm';", "")
                  .replace("const wasmModule = new WebAssembly.Module(Deno.readFileSync(wasmFile));", "const wasmModule = new WebAssembly.Module(source);")}
                  
                context.memory = wasm.memory;`;

console.log("minifying wasm");
const output = await minify(source, {
  mangle: { module: true },
  output: {
    preamble: "// deno-lint-ignore-file\n// deno-fmt-ignore-file",
  },
});

if (output.error) {
  console.error(`encountered error when minifying: ${output.error}`);
}

const reduction = new Blob([source]).size -
  new Blob([output.code]).size;
console.log(`minified js, size reduction: ${reduction} bytes`);

console.log(`writing output to file ("wasm.js")`);
await Deno.writeFile("wasm.js", encoder.encode(output.code));

const outputFile = await Deno.stat("wasm.js");
console.log(
  `output file ("wasm.js"), final size is: ${outputFile.size} bytes`,
);
