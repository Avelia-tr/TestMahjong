use std::io;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum LoadError {
    IO(io::Error),
    Display,
    NotFound,
}
