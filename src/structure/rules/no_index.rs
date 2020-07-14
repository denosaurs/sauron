use std::ffi::OsStr;
use std::path::PathBuf;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct NoIndex;

impl Check for NoIndex {
  fn check_file(&self, ctx: Context, path: &PathBuf, root: bool) {
    if !root {
      return;
    }

    match path.file_name().and_then(OsStr::to_str) {
      Some("index.js") => {
        ctx.add(
          DiagnosticLevel::Recommended,
          "no-index",
          "No `index.js` file allowed in root directory",
          path,
          None,
          None,
        );
      }
      Some("index.ts") => {
        ctx.add(
          DiagnosticLevel::Recommended,
          "NoIndex",
          "No `index.ts` file allowed in root directory",
          path,
          None,
          None,
        );
      }
      _ => {}
    }
  }
}

impl StructureRule for NoIndex {
  fn new() -> Box<Self> {
    Box::new(NoIndex)
  }
}
