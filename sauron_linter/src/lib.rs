use std::{fs, sync::Arc};
use std::path::{Path, PathBuf};

use deno_lint::linter::LinterBuilder;
use deno_lint::rules::LintRule;
use swc_ecmascript::parser::Syntax;

pub use context::LinterContext;
use sauron_core::{files::MediaType, rule::Rule, syntax};

pub struct Linter;

mod context;
mod diagnostic;
mod rules;

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
    .lint_unknown_rules(false)
    .syntax(syntax)
    .rules(rules)
    .build()
}

fn lint_file(file_path: PathBuf) -> Vec<deno_lint::diagnostic::LintDiagnostic> {
  let file_name = file_path.to_string_lossy().to_string();
  let source_code = fs::read_to_string(&file_path).unwrap();
  let media_type = MediaType::from(file_path.as_path());
  let syntax = syntax::get_syntax_for_media_type(media_type);

  let lint_rules = rules::get_deno_rules();
  let mut linter = create_linter(syntax, lint_rules);

  linter.lint(file_name, source_code).unwrap()
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

impl Rule<LinterContext> for Linter {
  fn check_file(&self, ctx: Arc<LinterContext>, path: &PathBuf, _root: bool) {
    if path.is_file() && is_supported(path.as_path()) {
      let diagnostics = lint_file(path.to_owned());
      for diagnostic in &diagnostics {
        ctx.add_diagnostic(diagnostic.clone().into())
      }
    }
  }
}

impl Default for Linter {
  fn default() -> Self {
    Linter {}
  }
}
