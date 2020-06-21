use swc_common::SpanData;
use std::path::PathBuf;
use crate::cp::frame::Frame;

#[derive(Clone, Debug)]
pub struct Position {
  pub start: SpanData,
  pub end: SpanData
}

impl From<&Frame> for Position {
  fn from(frame: &Frame) -> Self {
    Self {
      start: frame.start.span,
      end: frame.end.span
    }
  }
}

#[derive(Clone, Debug)]
pub struct Duplicate {
  pub left: Position,
  pub left_path: PathBuf,
  pub right: Position,
  pub right_path: PathBuf,
}

impl Duplicate {
  pub fn new(left: &Frame, left_path: &PathBuf, right: &Frame, right_path: &PathBuf) -> Duplicate {
    Self {
      left: left.into(),
      left_path: left_path.to_owned(),
      right: right.into(),
      right_path: right_path.to_owned(),
    }
  }

  pub fn enlarge(&mut self, left: &Frame, right: &Frame) {
    self.left.end = left.end.span;
    self.right.end = right.end.span;
  }
}
