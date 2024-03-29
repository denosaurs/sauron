use std::path::{Path, PathBuf};
use std::sync::Arc;

use dprint::configuration::Configuration;
use dprint::configuration::ConfigurationBuilder;
use dprint_plugin_typescript as dprint;

pub use context::FmtContext;
use sauron_core::media::MediaType;
use sauron_core::rule::Rule;

mod context;

mod diagnostic;

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

fn get_config() -> Configuration {
  ConfigurationBuilder::new().deno().build()
}

pub struct Formatter {
  config: Configuration,
}

impl Rule<FmtContext> for Formatter {
  fn check_file(
    &self,
    ctx: Arc<FmtContext>,
    path: &PathBuf,
    data: String,
    _root: bool,
  ) {
    if is_supported(path) {
      let r = dprint::format_text(path, &data, &self.config);

      match r {
        Ok(f) => {
          if f != data {
            ctx.add_diagnostic(self, path);
          }
        }
        Err(e) => {
          println!("{}", e);
        }
      }
    }
  }
  fn new() -> Box<Self> {
    Box::new(Formatter::default())
  }
  fn code(&self) -> &'static str {
    "unformatted-file"
  }
  fn docs(&self) -> &'static str {
    "https://mordor.land/#/format"
  }
}

impl Default for Formatter {
  fn default() -> Self {
    Formatter {
      config: get_config(),
    }
  }
}
