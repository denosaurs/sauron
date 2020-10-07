use std::{path::PathBuf, sync::Arc};

pub trait Rule<T> {
  fn new() -> Box<Self>
    where
      Self: Sized;
  fn code(&self) -> &'static str;
  fn docs(&self) -> &'static str;
  fn check_path(&self, _ctx: Arc<T>, _path: &PathBuf, _root: bool) {}
  fn check_file(
    &self,
    _ctx: Arc<T>,
    _path: &PathBuf,
    _data: String,
    _root: bool,
  ) {}
  fn check_context(&self, _ctx: Arc<T>, _path: &PathBuf) {}
}
