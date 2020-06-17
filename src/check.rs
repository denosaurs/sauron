use std::path::PathBuf;

use crate::context::Context;

pub trait Check {
  fn check_file(&self, _ctx: Context, _path: &PathBuf, _root: bool) {}
  fn check_context(&self, _ctx: Context, _path: &PathBuf) {}
}
