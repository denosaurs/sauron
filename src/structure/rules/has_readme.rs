use std::ffi::OsStr;
use std::path::PathBuf;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct HasReadme;

impl Check for HasReadme {
  fn check_file(&self, ctx: Context, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("README.md") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_readme", true);
    }
  }

  fn check_context(&self, ctx: Context, root_dir: &PathBuf) {
    if !ctx.get_flag("has_readme") {
      ctx.add(
        DiagnosticLevel::Recommended,
        root_dir,
        "HasReadme",
        "The module should have a README.md file in the root directory",
      );
    }
  }
}

impl StructureRule for HasReadme {
  fn new() -> Box<Self> {
    Box::new(HasReadme)
  }
}

