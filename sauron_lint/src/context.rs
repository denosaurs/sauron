use std::sync::{Arc, Mutex};

use sauron_core::context::Context;

use crate::diagnostic::LintDiagnostic;

pub enum RuleSet {
  Recommended,
  All,
}

pub struct LintConfig {
  pub rules: RuleSet,
}

impl Default for LintConfig {
  fn default() -> Self {
    Self {
      rules: RuleSet::Recommended,
    }
  }
}

pub struct LintContext {
  diagnostics: Arc<Mutex<Vec<LintDiagnostic>>>,

  pub config: LintConfig,
}

impl LintContext {
  pub fn add_diagnostic(&self, diagnostic: LintDiagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
  }
}

impl Context<LintDiagnostic, LintConfig> for LintContext {
  fn new(config: LintConfig) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
      config,
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<LintDiagnostic>>> {
    &self.diagnostics
  }
}
