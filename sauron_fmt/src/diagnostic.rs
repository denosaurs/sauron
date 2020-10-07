use std::fmt;

use colored::*;
use serde::Serialize;

use sauron_core::diagnostic::{
  Diagnostic, DiagnosticLevel, FileLocation, Location,
};

#[derive(Clone, Serialize)]
pub struct FmtDiagnostic {
  pub file: FileLocation,
  pub code: String,
  pub docs: String,
}

impl fmt::Display for FmtDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "{} ({}): {}\n",
      self.level(),
      format!("{}:{}", self.scope(), self.code()).dimmed(),
      self.short_message(),
    ))?;
    f.write_str("\n")?;
    f.write_fmt(format_args!("  {} {}\n", "-->".dimmed(), self.file))?;
    f.write_fmt(format_args!("  {} {}", "~".dimmed(), self.docs.dimmed()))
  }
}

impl Diagnostic for FmtDiagnostic {
  fn level(&self) -> DiagnosticLevel {
    DiagnosticLevel::Recommended
  }
  fn location(&self) -> Location {
    Location::File(self.file.clone())
  }
  fn short_message(&self) -> &str {
    "unformatted file"
  }
  fn code(&self) -> &str {
    &self.code
  }
  fn scope(&self) -> &'static str {
    "fmt"
  }
}
