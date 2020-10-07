use std::fmt;

use serde::Serialize;

use colored::*;
use sauron_core::diagnostic::{Diagnostic, DiagnosticLevel, Location};

#[derive(Clone, Serialize)]
pub struct StructureDiagnostic {
  pub level: DiagnosticLevel,
  pub location: Location,
  pub message: String,
  pub code: String,
  pub docs: String,
}

impl fmt::Display for StructureDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "{} ({}): {}",
      self.level,
      format!("{}:{}", self.scope(), self.code.clone()).dimmed(),
      self.message,
    ))?;
    f.write_str("\n")?;
    f.write_fmt(format_args!("  {} {}\n", "-->".dimmed(), self.location))?;
    f.write_fmt(format_args!("  {} {}", "~".dimmed(), self.docs.dimmed()))
  }
}

impl Diagnostic for StructureDiagnostic {
  fn level(&self) -> DiagnosticLevel {
    self.level.clone()
  }
  fn location(&self) -> Location {
    self.location.clone()
  }
  fn short_message(&self) -> &str {
    &self.message
  }
  fn code(&self) -> &str {
    &self.code
  }
  fn scope(&self) -> &'static str {
    "structure"
  }
}
