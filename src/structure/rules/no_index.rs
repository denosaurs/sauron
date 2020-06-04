use std::path::Path;
use std::ffi::OsStr;

use super::StructureDiagnostic;
use super::StructureRule;

pub struct NoIndex;

impl StructureRule for NoIndex {
  fn new() -> Box<Self> {
    Box::new(NoIndex)
  }

  fn check_file(&self, path: &Path, root: bool) -> Option<StructureDiagnostic> {
    if !root {
      return None
    }

    match path.file_name().and_then(OsStr::to_str) {
      Some("index.js") => Some(StructureDiagnostic {
        path: path.to_str().unwrap().to_string(),
        code: "NoIndex".to_string(),
        message: "No index.js file allowed in root directory".to_string()
      }),
      Some("index.ts") => Some(StructureDiagnostic {
        path: path.to_str().unwrap().to_string(),
        code: "NoIndex".to_string(),
        message: "No index.ts file allowed in root directory".to_string()
      }),
      _ => None
    }
  }
}
