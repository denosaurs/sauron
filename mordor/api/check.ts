import { ServerRequest } from "https://deno.land/std@0.74.0/http/server.ts";
import { check } from "../../sauron_wasm/mod.ts";

const decoder = new TextDecoder();

export default async (req: ServerRequest) => {
  try {
    const raw = await Deno.readAll(req.body);
    const src = decoder.decode(raw);
    const json = JSON.parse(src);

    try {
      const diagnostics = check(json);

      await req.respond({
        body: JSON.stringify(diagnostics),
        status: 200,
        headers: new Headers({
          "Content-Type": "application/json",
        }),
      });
    } catch {
      await req.respond({
        body: "could not check file tree",
        status: 500,
      });
    }
  } catch {
    await req.respond({
      body: "could not read/decode/parse body",
      status: 400,
    });
  }
};
