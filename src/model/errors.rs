//use std::
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Unable to load model at {0}")]
    UnableToOpenModel(String),
}
