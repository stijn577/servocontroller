use alloc::string::String;
use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic Error: {0}")]
    _Generic(String),
}
