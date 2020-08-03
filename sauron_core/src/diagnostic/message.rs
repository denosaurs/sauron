use colored::*;
use serde::Serialize;

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
        DiagnosticLevel::Required => "required".red().bold(),
        DiagnosticLevel::Recommended => "recommended".yellow().bold(),
      },
      format!("{}:{}", self.scope.clone(), self.code.clone()).white(),
      self.message,
    );
    let pretty_location = format!("  {} {}", "-->".white(), self.location);
    let lines = vec![pretty_error, pretty_location];
    lines.join("\n")
  }
}
