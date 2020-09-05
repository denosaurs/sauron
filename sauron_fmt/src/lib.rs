use std::path::{Path, PathBuf};
use std::sync::Arc;

use dprint_plugin_typescript as dprint;

use sauron_core::diagnostic::FileLocation;
use sauron_core::rule::Rule;

mod context;
pub use context::FmtContext;

mod diagnostic;
use diagnostic::FmtDiagnostic;

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

fn get_config() -> dprint::configuration::Configuration {
  use dprint::configuration::*;
  ConfigurationBuilder::new().deno().build()
}

pub struct Formatter {
  formatter: dprint::Formatter,
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
      let r = self.formatter.format_text(path, &data);

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
}

impl Default for Formatter {
  fn default() -> Self {
    Formatter {
      formatter: dprint::Formatter::new(get_config()),
    }
  }
}
