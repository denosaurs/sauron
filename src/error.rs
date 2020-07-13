use core::fmt;

#[derive(Debug)]
pub struct SauronError;

impl fmt::Display for SauronError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "SauronError is here!")
  }
}
