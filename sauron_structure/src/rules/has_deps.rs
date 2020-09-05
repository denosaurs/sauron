use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

use super::StructureRule;

pub struct HasDeps;

impl Rule<StructureContext> for HasDeps {
  fn check_path(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("deps.ts") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_deps", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_deps") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "has-deps",
        "The module should have a `deps.ts` file in the root directory",
        root_dir,
      );
    }
  }
}

impl StructureRule for HasDeps {
  fn new() -> Box<Self> {
    Box::new(HasDeps)
  }
}
