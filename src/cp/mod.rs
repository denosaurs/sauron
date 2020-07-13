use std::ffi::OsStr;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use deno_lint::swc_util::{get_default_es_config, get_default_ts_config};
use swc_common::SourceMap;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::input::SourceFileInput;

use swc_ecma_parser::{Session, Syntax};
use swc_ecma_parser::JscTarget::Es2019;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::token::TokenAndSpan;

use crate::check::Check;
use crate::context::Context;
use crate::error::SauronError;
use crate::cp::tok::Tok;
use crate::cp::rabin_karp::RabinKarp;
use crate::diagnostic::{DiagnosticLevel, Location};

mod rabin_karp;
mod duplicate;
mod frame;
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
      let tokens: Vec<Tok> = tokens
        .iter()
        .map(Tok::from)
        .collect();
      ctx.add_tokens(path.to_owned(), tokens);
    }
  }

  fn check_context(&self, ctx: Context, _path: &PathBuf) {
    let dupes = RabinKarp::find_duplicates(ctx.get_tokens());
    for dupe in &dupes {
      let left_loc: Location = self.source_map.lookup_char_pos(dupe.left.start.lo).into();
      let right_loc: Location = self.source_map.lookup_char_pos(dupe.right.start.lo).into();
      ctx.add(
        DiagnosticLevel::Recommended,
        "no-duplicates",
        right_loc.to_string().as_str(),
        &left_loc.path,
        left_loc.line,
        left_loc.col
      )
    }
  }
}

impl Default for CopyPaste {
  fn default() -> Self {
    Self {
      source_map: Arc::new(SourceMap::default())
    }
  }
}

impl CopyPaste {
  pub fn parse_file(&self, path: &Path) -> Result<Vec<TokenAndSpan>, SauronError> {
    match path.extension().and_then(OsStr::to_str) {
      Some("ts") => self.parse(path, get_default_ts_config()),
      _ => self.parse(path, get_default_es_config()),
    }
  }

  pub fn parse(&self, path: &Path, syntax: Syntax) -> Result<Vec<TokenAndSpan>, SauronError> {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
      let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false,
                                  Some(self.source_map.clone()));

      let session = Session { handler: &handler };

      let fm = self.source_map
        .load_file(path)
        .map_err(|_e| SauronError)?;

      let lexer = Lexer::new(
        session,
        syntax,
        Es2019,
        SourceFileInput::from(&*fm),
        None,
      );

      let tokens: Vec<TokenAndSpan> = lexer.collect();
      Ok(tokens)
    })
  }

}
