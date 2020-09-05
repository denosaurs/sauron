use std::sync::{Arc, Mutex};

use sauron_core::context::Context;

use crate::diagnostic::FmtDiagnostic;

pub struct FmtContext {
  diagnostics: Arc<Mutex<Vec<FmtDiagnostic>>>,
}

impl FmtContext {
  pub fn add_diagnostic(&self, diagnostic: FmtDiagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
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
