use core::fmt;

#[derive(Debug)]
pub struct NessieError;

impl fmt::Display for NessieError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "NessieError is here!")
  }
}
