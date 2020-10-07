use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use sauron_core::{context::Context, rule::Rule};
use sauron_core::diagnostic::{DiagnosticLevel, Location};

use crate::diagnostic::StructureDiagnostic;

pub struct StructureContext {
  diagnostics: Arc<Mutex<Vec<StructureDiagnostic>>>,
  flags: Arc<Mutex<HashMap<String, bool>>>,
}

impl StructureContext {
  pub fn add_diagnostic(
    &self,
    rule: &impl Rule<StructureContext>,
    level: DiagnosticLevel,
    message: &str,
    path: &std::path::PathBuf,
  ) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(StructureDiagnostic {
      level,
      code: rule.code().to_string(),
      docs: rule.docs().to_string(),
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
