use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SauronCoreError {
  #[error(transparent)]
  IOError(#[from] io::Error),
  #[error("unknown data store error")]
  Unknown,
}
