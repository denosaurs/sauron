use std::ffi::OsStr;
use std::{path::PathBuf, sync::Arc};

use regex::Regex;

use sauron_core::{diagnostic::DiagnosticLevel, rule::Rule};

use crate::StructureContext;

use super::StructureRule;

pub struct SnakeCase;

impl Rule<StructureContext> for SnakeCase {
  fn check_file(
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
        ctx.add(
          DiagnosticLevel::Recommended,
          "snake-case",
          "All source files must use snake case",
          path,
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
