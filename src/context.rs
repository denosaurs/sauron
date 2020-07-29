use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::cp::tok::Tok;
use crate::diagnostic::{
  Diagnostic, DiagnosticLevel, Location, MessageDiagnostic,
};

#[derive(Clone)]
pub struct Context {
  pub diagnostics: Arc<Mutex<Vec<Diagnostic>>>,
  flags: Arc<Mutex<HashMap<String, bool>>>,
  tokens: Arc<Mutex<HashMap<PathBuf, Vec<Tok>>>>,
  scope: String,
}

impl Context {
  pub fn add(
    &self,
    level: DiagnosticLevel,
    code: &str,
    message: &str,
    path: &PathBuf,
    line: Option<usize>,
    col: Option<usize>,
  ) {
    let diagnostic = MessageDiagnostic {
      level,
      location: Location {
        path: path.to_owned(),
        line,
        col,
      },
      scope: self.scope.clone(),
      code: code.to_string(),
      message: message.to_string(),
    };
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(Diagnostic::Message(diagnostic));
  }

  pub fn add_diagnostic(&self, diagnostic: Diagnostic) {
    let mut diagnostics = self.diagnostics.lock().unwrap();
    diagnostics.push(diagnostic);
  }

  pub fn set_flag(&self, key: &str, value: bool) {
    let mut flags = self.flags.lock().unwrap();
    flags.insert(format!("{}:{}", self.scope, key), value);
  }

  pub fn get_flag(&self, key: &str) -> bool {
    let flags = self.flags.lock().unwrap();
    flags
      .get(format!("{}:{}", self.scope, key).as_str())
      .cloned()
      .unwrap_or(false)
  }

  pub fn add_tokens(&self, path_buf: PathBuf, tokens: Vec<Tok>) {
    self.tokens.lock().unwrap().insert(path_buf, tokens);
  }

  pub fn get_tokens(&self) -> HashMap<PathBuf, Vec<Tok>> {
    self.tokens.lock().unwrap().clone()
  }

  pub fn scope(&self, scope: &str) -> Self {
    Context {
      diagnostics: self.diagnostics.clone(),
      flags: self.flags.clone(),
      tokens: self.tokens.clone(),
      scope: scope.to_string(),
    }
  }
}

impl Default for Context {
  fn default() -> Self {
    Context {
      diagnostics: Arc::new(Mutex::new(vec![])),
      flags: Arc::new(Mutex::new(HashMap::new())),
      tokens: Arc::new(Mutex::new(HashMap::new())),
      scope: "General".to_string(),
    }
  }
}
