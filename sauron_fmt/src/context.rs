use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

use sauron_core::{context::Context, diagnostic::FileLocation, rule::Rule};

use crate::diagnostic::FmtDiagnostic;

pub struct FmtContext {
  diagnostics: Arc<Mutex<Vec<FmtDiagnostic>>>,
}

impl FmtContext {
  pub fn add_diagnostic(&self, rule: &impl Rule<FmtContext>, path: &PathBuf) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(FmtDiagnostic {
      file: FileLocation {
        path: path.to_owned(),
        col: None,
        line: None,
      },
      code: rule.code().to_string(),
      docs: rule.docs().to_string(),
    });
  }
}

impl Context<FmtDiagnostic> for FmtContext {
  fn new(_: ()) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<FmtDiagnostic>>> {
    &self.diagnostics
  }
}
