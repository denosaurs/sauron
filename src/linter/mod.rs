use std::ffi::OsStr;
use std::path::PathBuf;

use deno_lint::linter::Linter as DenoLinter;
use deno_lint::rules::get_all_rules;
use deno_lint::swc_util::get_default_ts_config;

use crate::check::Check;
use crate::context::Context;
use crate::diagnostic::Diagnostic;

pub struct Linter;

impl Check for Linter {
  fn check_file(&self, ctx: Context, path: &PathBuf, _root: bool) {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => (),
      Some("js") => (),
      _ => return,
    };

    let mut linter = DenoLinter::default();
    let rules = get_all_rules();
    let syntax = get_default_ts_config();

    let source_code =
      std::fs::read_to_string(path).expect("Failed to read file");

    let file_diagnostics = linter
      .lint(
        String::from(path.to_str().unwrap()),
        source_code,
        syntax,
        rules,
      )
      .expect("Failed to lint");

    if !file_diagnostics.is_empty() {
      for d in file_diagnostics.iter() {
        ctx.add_diagnostic(Diagnostic::from(d.clone()))
      }
    }
  }
}

impl Default for Linter {
  fn default() -> Self {
    Linter {}
  }
}
