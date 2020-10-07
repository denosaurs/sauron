use std::fmt;
use std::path::PathBuf;

use colored::*;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum DiagnosticLevel {
  Required,
  Recommended,
}

impl fmt::Display for DiagnosticLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "{}",
      match &self {
        DiagnosticLevel::Required => "required".red().bold(),
        DiagnosticLevel::Recommended => "recommended".yellow().bold(),
      }
    ))
  }
}

pub trait Diagnostic: Serialize + fmt::Display {
  fn level(&self) -> DiagnosticLevel;
  fn location(&self) -> Location;
  fn short_message(&self) -> &str;
  fn code(&self) -> &str;
  fn scope(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize)]
pub struct FileLocation {
  pub path: PathBuf,
  pub line: Option<usize>,
  pub col: Option<usize>,
}

#[derive(Clone, Serialize)]
pub enum Location {
  File(FileLocation),
  Files(Vec<FileLocation>),
  Directory(PathBuf),
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Location::File(file) => f.write_fmt(format_args!("{}", file)),
      Location::Files(files) => f.write_fmt(format_args!("{:?}", files)),
      Location::Directory(dir) => {
        f.write_fmt(format_args!("{}", dir.display()))
      }
    }
  }
}

impl fmt::Display for FileLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut fmt = self.path.display().to_string();
    if let Some(line) = self.line {
      if let Some(col) = self.col {
        fmt = format!("{}:{}:{}", fmt, line, col + 1);
      } else {
        fmt = format!("{}:{}", fmt, line);
      }
    }
    write!(f, "{}", fmt)
  }
}

impl Into<FileLocation> for swc_common::Loc {
  fn into(self) -> FileLocation {
    use swc_common::FileName::*;

    let path = match &self.file.name {
      Real(path_buf) => path_buf.to_owned(),
      Custom(str) => PathBuf::from(str),
      _ => panic!("invalid filename"),
    };

    FileLocation {
      path,
      line: Some(self.line),
      col: Some(self.col.0),
    }
  }
}
