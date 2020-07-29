use std::ffi::OsStr;
use std::path::PathBuf;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct HasMod;

impl Check for HasMod {
  fn check_file(&self, ctx: Context, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("mod.ts") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_mod", true);
    }
  }

  fn check_context(&self, ctx: Context, root_dir: &PathBuf) {
    if !ctx.get_flag("has_mod") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "has-mod",
        "The module should have a `mod.ts` file in the root directory",
        root_dir,
        None,
        None,
      );
    }
  }
}

impl StructureRule for HasMod {
  fn new() -> Box<Self> {
    Box::new(HasMod)
  }
}
