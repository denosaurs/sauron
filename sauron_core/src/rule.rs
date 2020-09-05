use std::{path::PathBuf, sync::Arc};

/// Represents a SauronRule
pub trait Rule<T> {
  fn check_path(&self, _ctx: Arc<T>, _path: &PathBuf, _root: bool) {}
  fn check_file(&self, _ctx: Arc<T>, _path: &PathBuf, _data: String, _root: bool) {}
  fn check_context(&self, _ctx: Arc<T>, _path: &PathBuf) {}
}
