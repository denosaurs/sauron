use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

pub struct HasLicense;

impl Rule<StructureContext> for HasLicense {
  fn check_path(&self, ctx: Arc<StructureContext>, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("LICENSE") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_license", true);
    }
  }

  fn check_context(&self, ctx: Arc<StructureContext>, root_dir: &PathBuf) {
    if !ctx.get_flag("has_license") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "has-license",
        "The module should have a `LICENSE` file in the root directory",
        root_dir,
      );
    }
  }

  fn new() -> Box<Self> {
    Box::new(HasLicense)
  }

  fn code(&self) -> &'static str {
    "has-license"
  }
}
