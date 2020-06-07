use std::path::Path;
use std::ffi::OsStr;

use regex::Regex;

use crate::structure::diagnostic::DiagnosticLevel;
use crate::structure::diagnostic::StructureDiagnostic;
use crate::structure::rules::StructureRule;

pub struct SnakeCase;

impl StructureRule for SnakeCase {
  fn new() -> Box<Self> {
    Box::new(SnakeCase)
  }

  fn check_file(&self, path: &Path, _root: bool) -> Option<StructureDiagnostic> {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => (),
      Some("js") => (),
      _ => return None
    };

    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
      let re = Regex::new(r"^[a-z0-9_]+\..*$").unwrap();

      if !re.is_match(file_name) {
        return Some(StructureDiagnostic {
          level: DiagnosticLevel::Recommended,
          path: path.to_str().unwrap().to_string(),
          code: "SnakeCase".to_string(),
          message: "All source files must use snake case".to_string()
        });
      }
    }

    None
  }
}
