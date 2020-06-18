use crate::check::Check;

mod has_deps;
mod has_license;
mod has_mod;
mod has_readme;
mod no_index;
mod snake_case;

pub trait StructureRule: Check {
  fn new() -> Box<Self>
  where
    Self: Sized;
}

pub fn get_all_rules() -> Vec<Box<dyn StructureRule>> {
  vec![
    has_deps::HasDeps::new(),
    has_license::HasLicense::new(),
    has_mod::HasMod::new(),
    has_readme::HasReadme::new(),
    snake_case::SnakeCase::new(),
    no_index::NoIndex::new(),
  ]
}
