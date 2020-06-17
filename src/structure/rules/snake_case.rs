use std::ffi::OsStr;
use std::path::PathBuf;

use regex::Regex;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::DiagnosticLevel;
use crate::structure::rules::StructureRule;

pub struct SnakeCase;

impl Check for SnakeCase {
  fn check_file(&self, ctx: Context, path: &PathBuf, _root: bool) {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => (),
      Some("js") => (),
      _ => return,
    };

    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
      let re = Regex::new(r"^[a-z0-9_]+\..*$").unwrap();

      if !re.is_match(file_name) {
        ctx.add(
          DiagnosticLevel::Recommended,
          path,
          "SnakeCase",
          "All source files must use snake case",
        );
      }
    }
  }
}

impl StructureRule for SnakeCase {
  fn new() -> Box<Self> {
    Box::new(SnakeCase)
  }
}
