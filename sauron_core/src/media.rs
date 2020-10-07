use std::path::Path;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MediaType {
  JavaScript = 0,
  JSX = 1,
  TypeScript = 2,
  TSX = 3,
  Json = 4,
  Wasm = 5,
  Unknown = 6,
}

impl From<&Path> for MediaType {
  fn from(path: &Path) -> Self {
    match path.extension() {
      None => MediaType::Unknown,
      Some(os_str) => match os_str.to_str() {
        Some("ts") => MediaType::TypeScript,
        Some("tsx") => MediaType::TSX,
        Some("js") => MediaType::JavaScript,
        Some("jsx") => MediaType::JSX,
        Some("mjs") => MediaType::JavaScript,
        Some("cjs") => MediaType::JavaScript,
        Some("json") => MediaType::Json,
        Some("wasm") => MediaType::Wasm,
        _ => MediaType::Unknown,
      },
    }
  }
}
