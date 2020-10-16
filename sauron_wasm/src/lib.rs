use wasm_bindgen::prelude::*;

use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use sauron_core::{context::Context, rule::Rule};
use sauron_duplicate::{Duplicate, DuplicateContext};
use sauron_fmt::{FmtContext, Formatter};
use sauron_lint::{LintContext, Linter};
use sauron_structure::{rules, StructureContext};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Entry {
  File { path: String, data: String },
  Directory { path: String, data: Vec<Entry> },
}

#[wasm_bindgen]
pub fn init_panic_hook() {
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn sauron_check(file_tree: &JsValue) -> Option<String> {
  let file_tree = if let Ok(file_tree) = file_tree.into_serde::<Entry>() {
    file_tree
  } else {
    return None;
  };

  let duplicate = Duplicate::new();
  let formatter = Formatter::new();
  let linter = Linter::new();
  let structure_rules = rules::get_all_rules();
  
  let duplicate_ctx = DuplicateContext::default();
  let formatter_ctx = FmtContext::default();
  let linter_ctx = LintContext::default();
  let structure_ctx = StructureContext::default();

  check_file_tree(
    &Checkers {
      duplicate,
      formatter,
      linter,
      structure_rules,
    },
    &Contexts {
      duplicate_ctx: &duplicate_ctx,
      formatter_ctx: &formatter_ctx,
      linter_ctx: &linter_ctx,
      structure_ctx: &structure_ctx,
    },
    file_tree,
    true,
  );
  
  let duplicate_diagnostics = duplicate_ctx.diagnostics().lock().unwrap();
  let formatter_diagnostics = formatter_ctx.diagnostics().lock().unwrap();
  let linter_diagnostics = linter_ctx.diagnostics().lock().unwrap();
  let structure_diagnostics = structure_ctx.diagnostics().lock().unwrap();

  let mut diagnostics = Vec::new();

  for diag in duplicate_diagnostics.iter() {
    diagnostics.push(serde_json::to_value(diag).unwrap());
  }

  for diag in formatter_diagnostics.iter() {
    diagnostics.push(serde_json::to_value(diag).unwrap());
  }

  for diag in linter_diagnostics.iter() {
    diagnostics.push(serde_json::to_value(diag).unwrap());
  }

  for diag in structure_diagnostics.iter() {
    diagnostics.push(serde_json::to_value(diag).unwrap());
  }
  
  Some(serde_json::Value::Array(diagnostics).to_string())
}

struct Contexts<'a> {
  duplicate_ctx: &'a Arc<DuplicateContext>,
  formatter_ctx: &'a Arc<FmtContext>,
  linter_ctx: &'a Arc<LintContext>,
  structure_ctx: &'a Arc<StructureContext>,
}

struct Checkers {
  duplicate: Box<Duplicate>,
  formatter: Box<Formatter>,
  linter: Box<Linter>,
  structure_rules: Vec<Box<dyn Rule<StructureContext>>>,
}

fn check_file_tree(
  checkers: &Checkers,
  ctxs: &Contexts,
  entry: Entry,
  root: bool,
) {
  if let Entry::Directory { path, data } = entry {
    let path = PathBuf::from(path);

    for structure_rule in &checkers.structure_rules {
      structure_rule.check_path(ctxs.structure_ctx.clone(), &path, root);
    }

    for child in data.into_iter() {
      check_file_tree(checkers, ctxs, child, false);
    }

    if root {
      for structure_rule in &checkers.structure_rules {
        structure_rule.check_context(ctxs.structure_ctx.clone(), &path);
      }
    }
    
    checkers.duplicate.check_context(ctxs.duplicate_ctx.clone(), &path);
  } else if let Entry::File { path, data } = entry {
    let path = PathBuf::from(path);

    checkers.duplicate.check_file(
      ctxs.duplicate_ctx.clone(),
      &path,
      data.clone(),
      root,
    );
    checkers.formatter.check_file(
      ctxs.formatter_ctx.clone(),
      &path,
      data.clone(),
      root,
    );
    checkers.linter.check_file(ctxs.linter_ctx.clone(), &path, data, root);
  }
}
