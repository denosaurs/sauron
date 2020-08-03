use std::fmt;

use colored::*;
use serde::Serialize;

use sauron_core::diagnostic::{
  Diagnostic, DiagnosticLevel, FileLocation, Location,
};

#[derive(Clone, Serialize)]
pub struct DuplicateDiagnostic {
  pub right: FileLocation,
  pub left: FileLocation,
}

impl fmt::Display for DuplicateDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "{} ({}): {}\n",
      self.level(),
      format!("{}:{}", self.scope(), self.code()).white(),
      self.short_message(),
    ))?;
    f.write_fmt(format_args!("  {} {}\n", "-->".white(), self.left))?;
    f.write_fmt(format_args!("  {} {}\n", "-->".white(), self.right))
  }
}

impl Diagnostic for DuplicateDiagnostic {
  fn level(&self) -> DiagnosticLevel {
    DiagnosticLevel::Recommended
  }
  fn location(&self) -> Location {
    Location::Files(vec![self.right.clone(), self.left.clone()])
  }
  fn short_message(&self) -> &str {
    "cannot have chunks of duplicate code"
  }
  fn code(&self) -> &str {
    "no-copy-paste"
  }
  fn scope(&self) -> &'static str {
    "duplicate"
  }
}
