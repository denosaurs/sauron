mod check;
mod context;
mod diagnostic;

mod linter;
mod structure;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::context::Context;

use crate::check::Check;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Missing dir name");
    std::process::exit(1);
  }

  let dir_name = &args[1];

  let structure_rules = structure::rules::get_all_rules();
  let linter_rule = linter::Linter::default();

  let context = Context::default();

  for entry in WalkDir::new(dir_name) {
    if let Ok(entry) = entry {
      let path = entry.into_path();
      let root = path.parent().unwrap() == Path::new(dir_name);

      for structure_rule in &structure_rules {
        structure_rule.check_file(context.scope("Structure"), &path, root);
      }

      linter_rule.check_file(context.scope("Linter"), &path, root);
    }
  }

  for structure_rule in &structure_rules {
    structure_rule
      .check_context(context.scope("Structure"), &PathBuf::from(dir_name));
  }

  let diagnostics = context.diagnostics.lock().unwrap();
  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      eprintln!("{}", d.to_pretty_string());
    }
    eprintln!("Found {} structure problems", diagnostics.len());
  }
}
