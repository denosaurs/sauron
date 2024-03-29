use std::{fmt, path::PathBuf};

use colored::*;
use deno_lint::diagnostic::LintDiagnostic as DLintDiagnostic;
use serde::Serialize;

use sauron_core::diagnostic::{
  Diagnostic, DiagnosticLevel, FileLocation, Location,
};

#[derive(Clone, Serialize)]
pub struct LintDiagnostic {
  pub level: DiagnosticLevel,
  pub location: Location,
  pub message: String,
  pub code: String,
  pub docs: String,
}

impl fmt::Display for LintDiagnostic {
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

impl Diagnostic for LintDiagnostic {
  fn level(&self) -> DiagnosticLevel {
    self.level.clone()
  }
  fn location(&self) -> Location {
    self.location.clone()
  }
  fn code(&self) -> &str {
    &self.code
  }
  fn short_message(&self) -> &str {
    &self.message
  }
  fn scope(&self) -> &'static str {
    "linter"
  }
}

impl From<DLintDiagnostic> for LintDiagnostic {
  fn from(diag: DLintDiagnostic) -> Self {
    let path = PathBuf::from(diag.filename.clone());
    LintDiagnostic {
      level: DiagnosticLevel::Recommended,
      location: Location::File(FileLocation {
        path,
        line: Some(diag.range.start.line),
        col: Some(diag.range.start.col),
      }),
      message: diag.message,
      code: diag.code.clone(),
      docs: format!("https://mordor.land/#/linter?id={}", diag.code),
    }
  }
}
