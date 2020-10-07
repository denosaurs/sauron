use std::{path::PathBuf, sync::Arc};
use std::ffi::OsStr;

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

pub struct HasReadme;

impl Rule<StructureContext> for HasReadme {
  fn check_path(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("README.md") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_readme", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_readme") {
      ctx.add_diagnostic(
        self,
        DiagnosticLevel::Required,
        "The module should have a `README.md` file in the root directory",
        root_dir,
      );
    }
  }
  fn new() -> Box<Self> {
    Box::new(HasReadme)
  }

  fn code(&self) -> &'static str {
    "has-readme"
  }

  fn docs(&self) -> &'static str {
    "https://mordor.land/#/structure?id=has_readme"
  }
}
