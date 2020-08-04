use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

use super::StructureRule;

pub struct HasMod;

impl Rule<StructureContext> for HasMod {
  fn check_file(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("mod.ts") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_mod", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_mod") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "has-mod",
        "The module should have a `mod.ts` file in the root directory",
        root_dir,
      );
    }
  }
}

impl StructureRule for HasMod {
  fn new() -> Box<Self> {
    Box::new(HasMod)
  }
}
