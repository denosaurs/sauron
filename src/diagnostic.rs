use deno_lint::diagnostic::LintDiagnostic;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub enum DiagnosticLevel {
  // Required,
  Recommended,
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
  pub level: DiagnosticLevel,
  pub path: PathBuf,
  pub scope: String,
  pub message: String,
  pub code: String,
}

impl Diagnostic {
  pub fn to_pretty_string(&self) -> String {
    let pretty_error = format!(
      "{:?} ({}:{}) {}",
      self.level, self.scope, self.code, self.message
    );

    let pretty_location = format!(" --> {}", self.path.display());

    let lines = vec![pretty_error, pretty_location];

    lines.join("\n")
  }
}

impl From<LintDiagnostic> for Diagnostic {
  fn from(diag: LintDiagnostic) -> Self {
    Diagnostic {
      level: DiagnosticLevel::Recommended,
      path: PathBuf::from(diag.location.filename.clone()),
      scope: "Lint".to_string(),
      code: diag.code,
      message: format!(
        "{} ({}:{}:{})",
        diag.message,
        diag.location.filename,
        diag.location.line,
        diag.location.col
      ),
    }
  }
}
