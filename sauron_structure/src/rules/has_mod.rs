use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

pub struct HasMod;

impl Rule<StructureContext> for HasMod {
  fn check_path(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("mod.ts") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_mod", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_mod") {
      ctx.add_diagnostic(
        self,
        DiagnosticLevel::Recommended,
        "The module should have a `mod.ts` file in the root directory",
        root_dir,
      );
    }
  }
  fn new() -> Box<Self> {
    Box::new(HasMod)
  }
  fn code(&self) -> &'static str {
    "has-mod"
  }
  fn docs(&self) -> &'static str {
    "https://mordor.land/#/structure?id=has-mod"
  }
}
