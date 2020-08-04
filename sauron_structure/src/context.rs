use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use sauron_core::context::Context;
use sauron_core::diagnostic::{DiagnosticLevel, Location};

use crate::diagnostic::StructureDiagnostic;

// TODO(divy-work): Remove this macro when all_rules is used
#[allow(dead_code)]
pub struct StructureConfig {
  all_rules: bool,
}

impl Default for StructureConfig {
  fn default() -> Self {
    Self { all_rules: false }
  }
}

pub struct StructureContext {
  diagnostics: Arc<Mutex<Vec<StructureDiagnostic>>>,
  flags: Arc<Mutex<HashMap<String, bool>>>,
}

impl StructureContext {
  pub fn add_diagnostic(&self, diagnostic: StructureDiagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
  }
  pub fn add(
    &self,
    level: DiagnosticLevel,
    code: &str,
    message: &str,
    path: &std::path::PathBuf,
  ) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(StructureDiagnostic {
      level,
      code: code.to_string(),
      message: message.to_string(),
      location: Location::Directory(path.to_owned()),
    });
  }

  pub fn set_flag(&self, key: &str, value: bool) {
    let mut flags = self.flags.lock().unwrap();
    flags.insert(key.to_string(), value);
  }

  pub fn get_flag(&self, key: &str) -> bool {
    let flags = self.flags.lock().unwrap();
    flags.get(key).cloned().unwrap_or(false)
  }
}

impl Context<StructureDiagnostic> for StructureContext {
  fn new(_: ()) -> Arc<Self> {
    Arc::new(Self {
      diagnostics: Arc::new(Mutex::new(vec![])),
      flags: Arc::new(Mutex::new(HashMap::new())),
    })
  }
  fn diagnostics(&self) -> &Arc<Mutex<Vec<StructureDiagnostic>>> {
    &self.diagnostics
  }
}
