use std::path::{Path, PathBuf};
use std::{ffi::OsStr, sync::Arc};

use crate::swc_common::input::SourceFileInput;
use crate::swc_common::SourceMap;
use crate::swc_ecma_parser::lexer::Lexer;
use crate::swc_ecma_parser::token::TokenAndSpan;
use crate::swc_ecma_parser::JscTarget::Es2019;
use crate::swc_ecma_parser::Syntax;

use crate::check::Check;
use crate::context::Context;
use crate::cp::rabin_karp::RabinKarp;
use crate::cp::tok::Tok;
use crate::diagnostic::{DiagnosticLevel, Location};
use crate::{error::SauronError, files::MediaType, syntax};

mod duplicate;
mod frame;
mod rabin_karp;

pub mod tok;

pub struct CopyPaste {
  source_map: Arc<SourceMap>,
}

impl Check for CopyPaste {
  fn check_file(&self, ctx: Context, path: &PathBuf, _root: bool) {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => (),
      Some("js") => (),
      _ => return,
    };

    if let Ok(tokens) = self.parse_file(path) {
      let tokens: Vec<Tok> = tokens.iter().map(Tok::from).collect();
      ctx.add_tokens(path.to_owned(), tokens);
    }
  }

  fn check_context(&self, ctx: Context, _path: &PathBuf) {
    let dupes = RabinKarp::find_duplicates(ctx.get_tokens());
    for dupe in &dupes {
      let left_loc: Location =
        self.source_map.lookup_char_pos(dupe.left.start.lo).into();
      let right_loc: Location =
        self.source_map.lookup_char_pos(dupe.right.start.lo).into();
      ctx.add(
        DiagnosticLevel::Recommended,
        "no-duplicates",
        right_loc.to_string().as_str(),
        &left_loc.path,
        left_loc.line,
        left_loc.col,
      )
    }
  }
}

impl Default for CopyPaste {
  fn default() -> Self {
    Self {
      source_map: Arc::new(SourceMap::default()),
    }
  }
}

impl CopyPaste {
  pub fn parse_file(
    &self,
    path: &Path,
  ) -> Result<Vec<TokenAndSpan>, SauronError> {
    let media = MediaType::from(path);
    self.parse(path, syntax::get_syntax_for_media_type(media))
  }

  pub fn parse(
    &self,
    path: &Path,
    syntax: Syntax,
  ) -> Result<Vec<TokenAndSpan>, SauronError> {
    crate::swc_common::GLOBALS.set(&crate::swc_common::Globals::new(), || {
      let fm = self.source_map.load_file(path).map_err(|_e| SauronError)?;

      let lexer = Lexer::new(syntax, Es2019, SourceFileInput::from(&*fm), None);

      let tokens: Vec<TokenAndSpan> = lexer.collect();
      Ok(tokens)
    })
  }
}
