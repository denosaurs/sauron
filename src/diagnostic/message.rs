use serde::Serialize;

use crate::colors;
use crate::diagnostic::{Colored, DiagnosticLevel, Location};

#[derive(Clone, Serialize)]
pub struct MessageDiagnostic {
  pub level: DiagnosticLevel,
  pub location: Location,
  pub scope: String,
  pub message: String,
  pub code: String,
}

impl Colored for MessageDiagnostic {
  fn colored(&self) -> String {
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
    let pretty_location =
      format!("  {} {}", colors::blue("-->".to_string()), self.location);
    let lines = vec![pretty_error, pretty_location];
    lines.join("\n")
  }
}
