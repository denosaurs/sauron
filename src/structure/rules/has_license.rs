use std::ffi::OsStr;
use std::path::PathBuf;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct HasLicense;

impl Check for HasLicense {
  fn check_file(&self, ctx: Context, path: &PathBuf, root: bool) {
    if !root {
      return;
    }
    if let Some("LICENSE") = path.file_name().and_then(OsStr::to_str) {
      ctx.set_flag("has_license", true);
    }
  }

  fn check_context(&self, ctx: Context, root_dir: &PathBuf) {
    if !ctx.get_flag("has_license") {
      ctx.add(
        DiagnosticLevel::Recommended,
        "HasLicense",
        "The module should have a LICENSE file in the root directory",
        root_dir,
        None,
        None,
      );
    }
  }
}

impl StructureRule for HasLicense {
  fn new() -> Box<Self> {
    Box::new(HasLicense)
  }
}
