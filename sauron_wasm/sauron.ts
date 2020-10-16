import { walk } from "https://deno.land/std@0.74.0/fs/mod.ts";
import { resolve } from "https://deno.land/std@0.74.0/path/mod.ts";
import { check, Entry } from "./mod.ts";

async function createFileTree(
  path: string,
): Promise<Entry & { data: Entry[] }> {
  const root: Entry & { data: Entry[] } = {
    path,
    data: [],
  };

  for await (const entry of walk(path, { maxDepth: 1 })) {
    if (path !== entry.path) {
      if (entry.isDirectory) {
        root.data.push(await createFileTree(entry.path));
      }
      if (entry.isFile) {
        root.data.push({
          path: entry.path,
          data: await Deno.readTextFile(entry.path),
        });
      }
    }
  }

  return root;
}

const fileTree = await createFileTree(resolve(Deno.args[0] ?? "."));

const diagnostics = check(fileTree);

console.log(JSON.stringify(diagnostics, null, 2));
