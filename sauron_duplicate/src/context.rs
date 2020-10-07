use std::{
  collections::HashMap,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use sauron_core::{context::Context, diagnostic::FileLocation, rule::Rule};

use crate::{diagnostic::DuplicateDiagnostic, tok::Tok};

pub struct DuplicateConfig {
  pub min_tokens: usize,
}

impl Default for DuplicateConfig {
  fn default() -> Self {
    Self { min_tokens: 50 }
  }
}

pub struct DuplicateContext {
  diagnostics: Arc<Mutex<Vec<DuplicateDiagnostic>>>,
  tokens: Arc<Mutex<HashMap<PathBuf, Vec<Tok>>>>,

  pub config: DuplicateConfig,
}

impl DuplicateContext {
  pub fn add_diagnostic(
    &self,
    rule: &impl Rule<DuplicateContext>,
    left: FileLocation,
    right: FileLocation,
  ) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(DuplicateDiagnostic {
      left,
      right,
      code: rule.code().to_string(),
      docs: rule.docs().to_string(),
    });
  }

  pub fn add_tokens(&self, path_buf: PathBuf, tokens: Vec<Tok>) {
    self.tokens.lock().unwrap().insert(path_buf, tokens);
  }

  pub fn get_tokens(&self) -> HashMap<PathBuf, Vec<Tok>> {
    self.tokens.lock().unwrap().clone()
  }
}

impl Context<DuplicateDiagnostic, DuplicateConfig> for DuplicateContext {
  fn new(config: DuplicateConfig) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
      tokens: Arc::new(Mutex::new(HashMap::new())),
      config,
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<DuplicateDiagnostic>>> {
    &self.diagnostics
  }
}
