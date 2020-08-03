use std::sync::{Arc, Mutex};

use crate::diagnostic::Diagnostic;

pub trait Context<T: Diagnostic, C: Default = ()> {
  fn new(config: C) -> Arc<Self>;
  fn default() -> Arc<Self> {
    Self::new(C::default())
  }

  fn diagnostics(&self) -> &Arc<Mutex<Vec<T>>>;
}
