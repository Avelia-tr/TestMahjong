use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum LoadError {
    IOError(io::Error),
    DisplayError,
    NotFoundError,
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
