use std::sync::{Arc, Mutex};

use sauron_core::context::Context;

use crate::diagnostic::LintDiagnostic;

// TODO(divy-work): Remove this macro when all_rules is used
#[allow(dead_code)]

pub struct LintConfig {
  all_rules: bool,
}

impl Default for LintConfig {
  fn default() -> Self {
    Self { all_rules: false }
  }
}

pub struct LintContext {
  diagnostics: Arc<Mutex<Vec<LintDiagnostic>>>,
}

impl LintContext {
  pub fn add_diagnostic(&self, diagnostic: LintDiagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
  }
}

impl Context<LintDiagnostic> for LintContext {
  fn new(_: ()) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<LintDiagnostic>>> {
    &self.diagnostics
  }
}
