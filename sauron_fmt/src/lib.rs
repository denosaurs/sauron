use std::path::{Path, PathBuf};
use std::sync::Arc;

use dprint::configuration::Configuration;
use dprint::configuration::ConfigurationBuilder;
use dprint_plugin_typescript as dprint;

use sauron_core::diagnostic::FileLocation;
use sauron_core::files::MediaType;
use sauron_core::rule::Rule;

mod context;
pub use context::FmtContext;

mod diagnostic;
use diagnostic::FmtDiagnostic;

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
            let file: FileLocation = FileLocation {
              path: path.to_path_buf(),
              line: None,
              col: None,
            };

            ctx.add_diagnostic(FmtDiagnostic { file });
          }
        }
        Err(e) => {
          println!("{}", e);
        }
      }
    }
  }

  fn new() -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(Formatter::default())
  }

  fn code(&self) -> &'static str {
    "fmt"
  }
}

impl Default for Formatter {
  fn default() -> Self {
    Formatter {
      config: get_config(),
    }
  }
}
