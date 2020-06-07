use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum DiagnosticLevel {
  Required,
  Recommended,
  Optional
}

#[derive(Clone, Debug, Serialize)]
pub struct StructureDiagnostic {
  pub level: DiagnosticLevel,
  pub path: String, // Should be Path but did not get it to work
  pub message: String,
  pub code: String,
}

impl StructureDiagnostic {
  pub fn to_pretty_string(&self) -> String {
    let pretty_error =format!("{:?} ({}) {}", self.level, self.code.to_string(), self.message);

    let pretty_location = format!(" --> {}", self.path).to_string();

    let lines = vec![
      pretty_error,
      pretty_location,
    ];

    lines.join("\n")
  }
}
