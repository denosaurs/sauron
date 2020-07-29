use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use deno_lint::linter::LinterBuilder;
use deno_lint::rules::LintRule;

use crate::check::Check;
use crate::context::Context;
use crate::files::MediaType;
use crate::swc_ecma_parser::Syntax;
use std::any::Any;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct ErrBox(Box<dyn AnyError>);
// The Send and Sync traits are required because deno is multithreaded and we
// need to be able to handle errors across threads.
pub trait AnyError: Any + Error + Send + Sync + 'static {}
impl<T> AnyError for T where T: Any + Error + Send + Sync + Sized + 'static {}

pub struct Linter;

mod rules;
mod syntax;

fn create_linter(
  syntax: Syntax,
  rules: Vec<Box<dyn LintRule>>,
) -> deno_lint::linter::Linter {
  LinterBuilder::default()
    .ignore_file_directives(vec!["deno-lint-ignore-file"])
    .ignore_diagnostic_directives(vec![
      "deno-lint-ignore",
      "eslint-disable-next-line",
    ])
    .lint_unused_ignore_directives(true)
    // TODO(bartlomieju): switch to true
    .lint_unknown_rules(false)
    .syntax(syntax)
    .rules(rules)
    .build()
}

fn lint_file(file_path: PathBuf) -> Vec<deno_lint::diagnostic::LintDiagnostic> {
  let file_name = file_path.to_string_lossy().to_string();
  let source_code = fs::read_to_string(&file_path).unwrap();
  let syntax =
    syntax::get_syntax_for_media_type(MediaType::from(file_path.as_path()));

  // let lint_rules = rules::get_deno_rules();
  let mut linter = create_linter(syntax, vec![]);

  let file_diagnostics = linter.lint(file_name, source_code).unwrap();

  file_diagnostics
}

fn is_supported(path: &Path) -> bool {
  let lowercase_ext = path
    .extension()
    .and_then(|e| e.to_str())
    .map(|e| e.to_lowercase());
  if let Some(ext) = lowercase_ext {
    ext == "ts" || ext == "tsx" || ext == "js" || ext == "jsx" || ext == "mjs"
  } else {
    false
  }
}

impl Check for Linter {
  fn check_file(&self, ctx: Context, path: &PathBuf, _root: bool) {
    if path.is_file() && is_supported(path.as_path()) {
      lint_file(path.to_owned());
    }
  }
  //   match MediaType::from(path.as_path()) {
  //     MediaType::Unknown => return,
  //     MediaType::Wasm => return,
  //     MediaType::Json => return,
  //     _ => (),
  //   };
  //
  //   let source_code =
  //     std::fs::read_to_string(path).expect("Failed to read file");
  //
  //   let rules = rules::get_deno_rules();
  //   let syntax = syntax::get_syntax_for_file(path.as_path());
  //   println!("{:?}", syntax);
  //
  //   let mut linter = LinterBuilder::default()
  //       .ignore_file_directives(vec!["deno-lint-ignore-file"])
  //       .ignore_diagnostic_directives(vec![
  //         "deno-lint-ignore",
  //         "eslint-disable-next-line",
  //       ])
  //       .lint_unused_ignore_directives(true)
  //       .lint_unknown_rules(false)
  //       .syntax(syntax)
  //       .rules(rules)
  //       .build();
  //
  //   println!("{}", path.display());
  //
  //   let file_diagnostics = linter
  //     .lint(String::from(path.to_str().unwrap()), source_code);
  //
  //   if let Ok(file_diagnostics) = file_diagnostics {
  //     if !file_diagnostics.is_empty() {
  //       for d in file_diagnostics.iter() {
  //         ctx.add_diagnostic(d.clone().into())
  //       }
  //     }
  //   }
  // }
}

impl Default for Linter {
  fn default() -> Self {
    Linter {}
  }
}
