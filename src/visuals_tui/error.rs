use std::error::Error;
use std::io;

#[derive(Debug)]
enum LoadError {
    IOError(io::Error),
    DisplayError,
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
