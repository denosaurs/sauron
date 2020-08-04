use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use super::StructureRule;
use crate::StructureContext;

pub struct HasReadme;

impl Rule<StructureContext> for HasReadme {
  fn check_file(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("README.md") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_readme", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_readme") {
      ctx.add(
        DiagnosticLevel::Required,
        "has-readme",
        "The module should have a `README.md` file in the root directory",
        root_dir,
      );
    }
  }
}

impl StructureRule for HasReadme {
  fn new() -> Box<Self> {
    Box::new(HasReadme)
  }
}
