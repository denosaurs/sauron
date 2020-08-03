use std::{fmt, path::PathBuf};

use colored::*;
use deno_lint::diagnostic::LintDiagnostic as DLintDiagnostic;
use serde::Serialize;

use sauron_core::diagnostic::{
  Diagnostic, DiagnosticLevel, FileLocation, Location,
};

#[derive(Clone, Serialize)]
pub struct LinterDiagnostic {
  pub level: DiagnosticLevel,
  pub location: Location,
  pub message: String,
  pub code: String,
}

impl fmt::Display for LinterDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "{} ({}): {}",
      self.level,
      format!("{}:{}", self.scope(), self.code.clone()).white(),
      self.message,
    ))?;
    f.write_str("\n")?;
    f.write_fmt(format_args!("  {} {}", "-->".white(), self.location))
  }
}

impl Diagnostic for LinterDiagnostic {
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

impl From<DLintDiagnostic> for LinterDiagnostic {
  fn from(diag: DLintDiagnostic) -> Self {
    let path = PathBuf::from(diag.location.filename.clone());
    LinterDiagnostic {
      level: DiagnosticLevel::Recommended,
      location: Location::File(FileLocation {
        path,
        line: Some(diag.location.line),
        col: Some(diag.location.col),
      }),
      code: diag.code,
      message: diag.message,
    }
  }
}
