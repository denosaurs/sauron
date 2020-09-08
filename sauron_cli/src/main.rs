use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::{app_from_crate, Arg};
use colored::*;
use walkdir::WalkDir;

use sauron_core::{context::Context, rule::Rule};
use sauron_duplicate::{Duplicate, DuplicateContext};
use sauron_fmt::{FmtContext, Formatter};
use sauron_lint::{LintContext, Linter};
use sauron_structure::{rules, StructureContext};

fn main() {
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

  let formatter_rule = Formatter::default();
  let formatter_ctx = FmtContext::default();

  let linter_rule = Linter::default();
  let linter_ctx = LintContext::default();

  let duplicate_rule = Duplicate::default();
  let duplicate_ctx = DuplicateContext::default();

  let walk = WalkDir::new(&root_dir);
  for entry in walk {
    if let Ok(entry) = entry {
      let path = &entry.into_path();
      let root = path.parent().unwrap() == root_dir.as_path();

      for structure_rule in &structure_rules {
        structure_rule.check_path(structure_ctx.clone(), path, root);
      }

      if path.is_file() {
        match fs::read_to_string(path) {
          Ok(data) => {
            duplicate_rule.check_file(
              duplicate_ctx.clone(),
              &path,
              data.clone(),
              root,
            );
            formatter_rule.check_file(
              formatter_ctx.clone(),
              &path,
              data.clone(),
              root,
            );
            linter_rule.check_file(linter_ctx.clone(), &path, data, root);
          }
          Err(e) => {
            if e.kind() != std::io::ErrorKind::InvalidData {
              println!(
                "{}: file `{}` could not be read because {}",
                "error".red().bold(),
                path.display(),
                e
              );
              exit(1);
            }
          }
        }
      }
    }
  }

  for structure_rule in &structure_rules {
    structure_rule.check_context(structure_ctx.clone(), &root_dir);
  }

  duplicate_rule.check_context(duplicate_ctx.clone(), &root_dir);

  let duplicate_diagnostics = duplicate_ctx.diagnostics().lock().unwrap();
  let formatter_diagnostics = formatter_ctx.diagnostics().lock().unwrap();
  let linter_diagnostics = linter_ctx.diagnostics().lock().unwrap();
  let structure_diagnostics = structure_ctx.diagnostics().lock().unwrap();

  if !duplicate_diagnostics.is_empty()
    || !formatter_diagnostics.is_empty()
    || !linter_diagnostics.is_empty()
    || !structure_diagnostics.is_empty()
  {
    for diag in duplicate_diagnostics.iter() {
      println!("\n{}", diag);
    }
    for diag in formatter_diagnostics.iter() {
      println!("\n{}", diag);
    }
    for diag in linter_diagnostics.iter() {
      println!("\n{}", diag);
    }
    for diag in structure_diagnostics.iter() {
      println!("\n{}", diag);
    }

    println!();
    println!(
      "{} - found {} problem[s]",
      "results".red().bold(),
      duplicate_diagnostics.len()
        + formatter_diagnostics.len()
        + linter_diagnostics.len()
        + structure_diagnostics.len()
    );
  }
}
