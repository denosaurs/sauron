use std::path::Path;

use crate::structure::diagnostic::StructureDiagnostic;

mod snake_case;
mod no_index;

pub trait StructureRule {
  fn new() -> Box<Self>
  where
    Self: Sized;
  fn check_file(&self, path: &Path, root: bool) -> Option<StructureDiagnostic>;
}

pub fn get_all_rules() -> Vec<Box<dyn StructureRule>> {
  vec![
    snake_case::SnakeCase::new(), 
    no_index::NoIndex::new()
  ]
}
