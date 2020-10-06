use crate::StructureContext;
use sauron_core::rule::Rule;

mod has_deps;
mod has_license;
mod has_mod;
mod has_readme;
mod no_index;
mod snake_case;

pub fn get_all_rules() -> Vec<Box<dyn Rule<StructureContext>>> {
  vec![
    has_deps::HasDeps::new(),
    has_license::HasLicense::new(),
    has_mod::HasMod::new(),
    has_readme::HasReadme::new(),
    snake_case::SnakeCase::new(),
    no_index::NoIndex::new(),
  ]
}
