use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::{app_from_crate, Arg};
use colored::*;
use walkdir::WalkDir;

use sauron_core::{context::Context, rule::Rule};
use sauron_duplicate::{Duplicate, DuplicateContext};
use sauron_linter::{Linter, LinterContext};
use sauron_structure::{rules, StructureContext};

fn main() {
  #[cfg(windows)]
  colors::enable_ansi();

  let matches = app_from_crate!()
    .arg(
      Arg::with_name("directory")
        .about("Root directory of the module")
        .required(true)
        .index(1),
    )
    .get_matches();

  let root_dir = PathBuf::from(matches.value_of("directory").unwrap());

  if !root_dir.exists() {
    println!(
      "{}: directory `{}` does not exist.",
      "error".red().bold(),
      root_dir.display()
    );
    exit(1);
  }

  let root_dir = fs::canonicalize(root_dir).unwrap();

  let structure_rules = rules::get_all_rules();
  let structure_ctx = StructureContext::default();

  let linter_rule = Linter::default();
  let linter_ctx = LinterContext::default();

  let duplicate_rule = Duplicate::default();
  let duplicate_ctx = DuplicateContext::default();

  let walk = WalkDir::new(&root_dir);
  for entry in walk {
    if let Ok(entry) = entry {
      let path = entry.into_path();
      let root = path.parent().unwrap() == root_dir.as_path();

      for structure_rule in &structure_rules {
        structure_rule.check_file(structure_ctx.clone(), &path, root);
      }

      linter_rule.check_file(linter_ctx.clone(), &path, root);
      duplicate_rule.check_file(duplicate_ctx.clone(), &path, root)
    }
  }

  for structure_rule in &structure_rules {
    structure_rule.check_context(structure_ctx.clone(), &root_dir);
  }

  duplicate_rule.check_context(duplicate_ctx.clone(), &root_dir);

  let diagnostics = duplicate_ctx.diagnostics().lock().unwrap();
  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      println!();
      println!("{}", d);
    }
    println!();
    println!(
      "{} - found {} problem[s]",
      "results".red().bold(),
      diagnostics.len()
    );
  }

  let diagnostics = linter_ctx.diagnostics().lock().unwrap();
  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      println!();
      println!("{}", d);
    }
    println!();
    println!(
      "{} - found {} problem[s]",
      "results".red().bold(),
      diagnostics.len()
    );
  }

  let diagnostics = structure_ctx.diagnostics().lock().unwrap();
  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      println!();
      println!("{}", d);
    }
    println!();
    println!(
      "{} - found {} problem[s]",
      "results".red().bold(),
      diagnostics.len()
    );
  }
}
