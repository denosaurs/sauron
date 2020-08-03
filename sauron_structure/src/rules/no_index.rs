use std::{path::PathBuf, sync::Arc};
use std::ffi::OsStr;

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

use super::StructureRule;

pub struct NoIndex;

impl Rule<StructureContext> for NoIndex {
  fn check_file(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
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
        );
      }
      Some("index.ts") => {
        ctx.add(
          DiagnosticLevel::Recommended,
          "NoIndex",
          "No `index.ts` file allowed in root directory",
          path,
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
