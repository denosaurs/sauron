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

await requires("rustup", "rustc", "cargo", "wasm-pack");

if (!(await Deno.stat("Cargo.toml")).isFile) {
  console.error(`the build script should be executed in the "${name}" root`);
}

await run(
  "building using wasm-pack",
  ["wasm-pack", "build", "--target", "web", "--release"],
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
                export const source = lz4.decompress(Uint8Array.from(atob("${encoded}"), c => c.charCodeAt(0)));
                ${await Deno.readTextFile(`pkg/${name}.js`)}`
  .replace("async function init", "export async function init")
  .replace("export default init;", "");

console.log("minifying js");
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
