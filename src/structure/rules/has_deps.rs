use std::ffi::OsStr;
use std::path::PathBuf;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct HasDeps;

impl Check for HasDeps {
  fn check_file(&self, ctx: Context, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("deps.ts") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_deps", true);
    }
  }

  fn check_context(&self, ctx: Context, root_dir: &PathBuf) {
    if !ctx.get_flag("has_deps") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "HasDeps",
        "The module should have a deps.ts file in the root directory",
        root_dir,
        None,
        None
      );
    }
  }
}

impl StructureRule for HasDeps {
  fn new() -> Box<Self> {
    Box::new(HasDeps)
  }
}
