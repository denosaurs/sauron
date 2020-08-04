use std::sync::{Arc, Mutex};

use sauron_core::context::Context;

use crate::diagnostic::LinterDiagnostic;

// TODO(divy-work): Remove this macro when all_rules is used
#[allow(dead_code)]
pub struct LinterConfig {
  all_rules: bool,
}

impl Default for LinterConfig {
  fn default() -> Self {
    Self { all_rules: false }
  }
}

pub struct LinterContext {
  diagnostics: Arc<Mutex<Vec<LinterDiagnostic>>>,
}

impl LinterContext {
  pub fn add_diagnostic(&self, diagnostic: LinterDiagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
  }
}

impl Context<LinterDiagnostic> for LinterContext {
  fn new(_: ()) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<LinterDiagnostic>>> {
    &self.diagnostics
  }
}
