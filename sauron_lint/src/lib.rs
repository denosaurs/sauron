use std::path::{Path, PathBuf};
use std::sync::Arc;

use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use deno_lint::rules::LintRule;
use swc_ecmascript::parser::Syntax;

pub use context::LintContext;
use sauron_core::{media::MediaType, rule::Rule, syntax};

pub struct Linter;

mod context;
mod diagnostic;

fn create_linter(
  syntax: Syntax,
  rules: Vec<Box<dyn LintRule>>,
) -> deno_lint::linter::Linter {
  LinterBuilder::default()
    .lint_unused_ignore_directives(true)
    .lint_unknown_rules(false)
    .syntax(syntax)
    .rules(rules)
    .build()
}

fn is_supported(path: &Path) -> bool {
  match MediaType::from(path) {
    MediaType::JavaScript => true,
    MediaType::JSX => true,
    MediaType::TypeScript => true,
    MediaType::TSX => true,
    MediaType::Json => false,
    MediaType::Wasm => false,
    MediaType::Unknown => false,
  }
}

impl Rule<LintContext> for Linter {
  fn check_file(
    &self,
    ctx: Arc<LintContext>,
    path: &PathBuf,
    data: String,
    _root: bool,
  ) {
    if is_supported(path) {
      let file_name = path.to_string_lossy().to_string();
      let media_type = MediaType::from(path.as_path());
      let syntax = syntax::get_syntax_for_media_type(media_type);

      let lint_rules = match ctx.config.rules {
        context::RuleSet::Recommended => get_recommended_rules(),
        context::RuleSet::All => get_all_rules(),
      };

      let mut linter = create_linter(syntax, lint_rules);

      let diagnostics = linter.lint(file_name, data).unwrap();
      for diagnostic in &diagnostics {
        ctx.add_diagnostic(diagnostic.clone().into())
      }
    }
  }
  fn new() -> Box<Self> {
    Box::new(Linter)
  }
  fn code(&self) -> &'static str {
    "linter"
  }
  fn docs(&self) -> &'static str {
    "https://mordor.land/#/linter"
  }
}

impl Default for Linter {
  fn default() -> Self {
    Linter {}
  }
}
