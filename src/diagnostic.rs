use deno_lint::diagnostic::LintDiagnostic;
use serde::Serialize;
use std::path::PathBuf;

use crate::colors;
use core::fmt;
use serde::export::Formatter;

#[derive(Clone, Debug, Serialize)]
pub enum DiagnosticLevel {
  Required,
  Recommended,
}

#[derive(Debug, Clone, Serialize)]
pub struct Location {
  pub path: PathBuf,
  pub line: Option<usize>,
  pub col: Option<usize>,
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut fmt = self.path.display().to_string();
    if let Some(line) = self.line {
      if let Some(col) = self.col {
        fmt = format!("{}:{}:{}", fmt, line, col);
      } else {
        fmt = format!("{}:{}", fmt, line);
      }
    }
    write!(f, "{}", fmt)
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
  pub level: DiagnosticLevel,
  pub location: Location,
  pub scope: String,
  pub message: String,
  pub code: String,
}

impl Diagnostic {
  pub fn to_pretty_string(&self) -> String {
    let pretty_error = format!(
      "{} ({}): {}",
      match &self.level {
        DiagnosticLevel::Required =>
          colors::red_bold("required".to_string()).to_string(),
        DiagnosticLevel::Recommended =>
          colors::yellow_bold("recommended".to_string()).to_string(),
      },
      colors::gray(format!("{}:{}", self.scope.clone(), self.code.clone())),
      self.message,
    );

    let pretty_location = format!(
      "  {} {}",
      colors::blue("-->".to_string()),
      self.location
    );

    let lines = vec![pretty_error, pretty_location];

    lines.join("\n")
  }
}

impl From<LintDiagnostic> for Diagnostic {
  fn from(diag: LintDiagnostic) -> Self {
    let path = PathBuf::from(diag.location.filename.clone());
    Diagnostic {
      level: DiagnosticLevel::Recommended,
      location: Location {
        path: path.clone(),
        line: Some(diag.location.line),
        col: Some(diag.location.col),
      },
      scope: "lint".to_string(),
      code: diag.code,
      message: diag.message,
    }
  }
}
