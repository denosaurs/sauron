use std::path::PathBuf;

use swc_common::Span;

use crate::frame::Frame;

#[derive(Clone, Debug)]
pub struct Position {
  pub start: Span,
  pub end: Span,
}

impl From<&Frame> for Position {
  fn from(frame: &Frame) -> Self {
    Self {
      start: frame.start.span,
      end: frame.end.span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Ditto {
  pub left: Position,
  pub left_path: PathBuf,
  pub right: Position,
  pub right_path: PathBuf,
}

impl Ditto {
  pub fn new(
    left: &Frame,
    left_path: &PathBuf,
    right: &Frame,
    right_path: &PathBuf,
  ) -> Self {
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
