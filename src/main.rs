mod check;
mod error;
mod context;
mod diagnostic;
mod cp;

mod linter;
mod structure;

mod colors;

use std::path::PathBuf;
use std::process::exit;

use clap::{Arg, app_from_crate, crate_name, crate_version, crate_description, crate_authors};
use walkdir::WalkDir;
use prettytable::{Table, format, row, cell};

use crate::context::Context;

use crate::check::Check;
use crate::diagnostic::{Diagnostic, Colored};

fn main() {
  #[cfg(windows)]
  colors::enable_ansi();

  let matches = app_from_crate!()
    .arg(Arg::with_name("DIRECTORY")
      .help("Sets the root directory of the module")
      .required(true)
      .index(1))
    .get_matches();

  let root_dir = PathBuf::from(matches.value_of("DIRECTORY").unwrap());

  if !root_dir.exists() {
    eprintln!(
      "{}: directory `{}` does not exist.",
      colors::red_bold("error".to_string()),
      root_dir.display()
    );
    exit(1);
  }

  let structure_rules = structure::rules::get_all_rules();
  let linter_rule = linter::Linter::default();
  let cp_rule = cp::CopyPaste::default();

  let context = Context::default();

  let walk = WalkDir::new(&root_dir);
  for entry in walk {
    if let Ok(entry) = entry {
      let path = entry.into_path();
      let root = path.parent().unwrap() == root_dir.as_path();

      for structure_rule in &structure_rules {
        structure_rule.check_file(context.scope("structure"), &path, root);
      }

      linter_rule.check_file(context.scope("lint"), &path, root);
      cp_rule.check_file(context.scope("copypaste"), &path, root)
    }
  }

  for structure_rule in &structure_rules {
    structure_rule.check_context(context.scope("structure"), &root_dir);
  }

  cp_rule.check_context(context.scope("copypaste"), &root_dir);

  let diagnostics = context.diagnostics.lock().unwrap();
  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      eprintln!();
      eprintln!("{}", match d {
        Diagnostic::Message(msg) => msg.colored()
      });
    }
    eprintln!();
    eprintln!(
      "{} - found {} problems:",
      colors::red_bold("results".to_string()),
      diagnostics.len()
    );
    let mut results = Table::new();
    results.set_format(*format::consts::FORMAT_CLEAN);
    for scope in &["structure", "lint", "copypaste"] {
      results.add_row(row![
        Fyb->scope,
        F->diagnostics.iter().filter(|d| match d {
          Diagnostic::Message(msg) => msg.scope.eq(scope)
        }).collect::<Vec<&Diagnostic>>().len()]);
    }
    results.print_tty(true);
  }
}
