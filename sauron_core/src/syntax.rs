use swc_ecmascript::parser::{EsConfig, Syntax, TsConfig};

use crate::media::MediaType;

fn get_default_es_config() -> EsConfig {
  let mut config = EsConfig::default();
  config.num_sep = true;
  config.class_private_props = true;
  config.class_private_methods = true;
  config.class_props = true;
  config.export_default_from = true;
  config.export_namespace_from = true;
  config.dynamic_import = true;
  config.nullish_coalescing = true;
  config.optional_chaining = true;
  config.import_meta = true;
  config.top_level_await = true;
  config
}

fn get_default_ts_config() -> TsConfig {
  let mut ts_config = TsConfig::default();
  ts_config.dynamic_import = true;
  ts_config.decorators = true;
  ts_config
}

pub fn get_syntax_for_media_type(media_type: MediaType) -> Syntax {
  match media_type {
    MediaType::JavaScript => Syntax::Es(get_default_es_config()),
    MediaType::JSX => {
      let mut config = get_default_es_config();
      config.jsx = true;
      Syntax::Es(config)
    }
    MediaType::TypeScript => Syntax::Typescript(get_default_ts_config()),
    MediaType::TSX => {
      let mut config = get_default_ts_config();
      config.tsx = true;
      Syntax::Typescript(config)
    }
    _ => Syntax::Es(get_default_es_config()),
  }
}
