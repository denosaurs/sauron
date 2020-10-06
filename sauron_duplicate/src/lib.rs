use std::path::{Path, PathBuf};
use std::sync::Arc;

use swc_common::input::SourceFileInput;
use swc_common::SourceMap;
use swc_ecmascript::parser::lexer::Lexer;
use swc_ecmascript::parser::token::TokenAndSpan;
use swc_ecmascript::parser::JscTarget::Es2019;
use swc_ecmascript::parser::Syntax;

pub use context::DuplicateContext;
use sauron_core::{
  diagnostic::FileLocation, error::SauronCoreError, files::MediaType,
  rule::Rule, syntax,
};

use crate::rabin_karp::RabinKarp;
use crate::tok::Tok;

mod context;
mod diagnostic;
mod ditto;
mod frame;
mod rabin_karp;
mod tok;

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

pub struct Duplicate {
  source_map: Arc<SourceMap>,
}

impl Rule<DuplicateContext> for Duplicate {
  fn check_file(
    &self,
    ctx: Arc<DuplicateContext>,
    path: &PathBuf,
    data: String,
    _root: bool,
  ) {
    if is_supported(path) {
      if let Ok(tokens) = self.parse_file(path, data) {
        let tokens: Vec<Tok> = tokens.iter().map(Tok::from).collect();
        ctx.add_tokens(path.to_owned(), tokens);
      }
    }
  }

  fn check_context(&self, ctx: Arc<DuplicateContext>, _path: &PathBuf) {
    let dupes =
      RabinKarp::find_duplicates(ctx.get_tokens(), ctx.config.min_tokens);
    for dupe in &dupes {
      let left: FileLocation =
        self.source_map.lookup_char_pos(dupe.left.start.lo).into();
      let right: FileLocation =
        self.source_map.lookup_char_pos(dupe.right.start.lo).into();
      ctx.add_diagnostic(left, right)
    }
  }

  fn new() -> Box<Self> {
    Box::new(Duplicate::default())
  }

  fn code(&self) -> &'static str {
    "no-copy-paste"
  }
}

impl Default for Duplicate {
  fn default() -> Self {
    Self {
      source_map: Arc::new(SourceMap::default()),
    }
  }
}

impl Duplicate {
  pub fn parse_file(
    &self,
    path: &Path,
    data: String,
  ) -> Result<Vec<TokenAndSpan>, SauronCoreError> {
    let media = MediaType::from(path);
    self.parse(path, data, syntax::get_syntax_for_media_type(media))
  }

  pub fn parse(
    &self,
    path: &Path,
    data: String,
    syntax: Syntax,
  ) -> Result<Vec<TokenAndSpan>, SauronCoreError> {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
      let fm = self
        .source_map
        .new_source_file(path.to_owned().into(), data);
      let lexer = Lexer::new(syntax, Es2019, SourceFileInput::from(&*fm), None);
      let tokens: Vec<TokenAndSpan> = lexer.collect();
      Ok(tokens)
    })
  }
}
