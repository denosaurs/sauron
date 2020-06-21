use core::fmt;
use std::path::PathBuf;
use std::fmt::Formatter;

use deno_lint::diagnostic::LintDiagnostic;
use serde::Serialize;

mod message;
pub use message::MessageDiagnostic;

#[derive(Clone, Debug, Serialize)]
pub enum DiagnosticLevel {
  Required,
  Recommended,
}

#[derive(Clone, Serialize)]
pub enum Diagnostic {
  Message(MessageDiagnostic),
}

#[derive(Clone, Serialize)]
pub struct Location {
  pub path: PathBuf,
  pub line: Option<usize>,
  pub col: Option<usize>,
}

pub trait Colored {
  fn colored(&self) -> String;
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut fmt = self.path.display().to_string();
    if let Some(line) = self.line {
      if let Some(col) = self.col {
        fmt = format!("{}:{}:{}", fmt, line, col + 1);
      } else {
        fmt = format!("{}:{}", fmt, line);
      }
    }
    write!(f, "{}", fmt)
  }
}

impl From<LintDiagnostic> for Diagnostic {
  fn from(diag: LintDiagnostic) -> Self {
    let path = PathBuf::from(diag.location.filename.clone());
    Diagnostic::Message(MessageDiagnostic {
      level: DiagnosticLevel::Recommended,
      location: Location {
        path,
        line: Some(diag.location.line),
        col: Some(diag.location.col),
      },
      scope: "lint".to_string(),
      code: diag.code,
      message: diag.message,
    })
  }
}

impl Into<Location> for swc_common::Loc {
  fn into(self) -> Location {
    use swc_common::FileName::*;

    let path = match &self.file.name {
      Real(path_buf) => path_buf.to_owned(),
      Custom(str) => PathBuf::from(str),
      _ => panic!("invalid filename"),
    };

    Location {
      path,
      line: Some(self.line),
      // Using self.col instead of self.col_display
      // because it leads to out-of-bounds columns if file
      // contains non-narrow chars (like tabs).
      // See: https://github.com/denoland/deno_lint/issues/139
      col: Some(self.col.0),
    }
  }
}
