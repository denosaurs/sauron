use std::{path::PathBuf, sync::Arc};
use std::ffi::OsStr;

use regex::Regex;

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

pub struct SnakeCase;

impl Rule<StructureContext> for SnakeCase {
  fn check_path(
    &self,
    ctx: Arc<StructureContext>,
    path: &PathBuf,
    _root: bool,
  ) {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => (),
      Some("js") => (),
      _ => return,
    };

    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
      let re = Regex::new(r"^[a-z0-9_]+\..*$").unwrap();

      if !re.is_match(file_name) {
        ctx.add_diagnostic(
          self,
          DiagnosticLevel::Recommended,
          "All source files must use snake case",
          path,
        );
      }
    }
  }
  fn new() -> Box<Self> {
    Box::new(SnakeCase)
  }
  fn code(&self) -> &'static str {
    "snake-case"
  }
  fn docs(&self) -> &'static str {
    "https://mordor.land/#/structure?id=snake_case"
  }
}
