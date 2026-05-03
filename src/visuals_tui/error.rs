use std::{io, string};

use crate::visuals_tui::image_display_message::{ImageId, ImageNumber};

pub struct OKAnswer(pub ImageId, pub Option<ImageNumber>);

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum LoadError {
    IO(io::Error),
    Display,
    NotFound,
    UnRecognizedError,
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("error {kind} in {}",origin.replace("\x1B", "<ESC>"))]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub origin: String,
}

#[derive(Debug, derive_more::Display)]
pub enum ParsingErrorKind {
    #[display("expected {expected_char} at {at} got {got_char}")]
    UnexpectedCharacter {
        expected_char: char,
        got_char: char,
        at: usize,
    },
    UnRecognizedErrorCode,
    IO(io::Error),
    NonValidUtf8Read(string::FromUtf8Error),
    InvalidDigit(u32),
}

impl From<io::Error> for ParsingError {
    fn from(value: io::Error) -> Self {
        Self {
            kind: ParsingErrorKind::IO(value),
            origin: String::new(),
        }
    }
}

impl From<string::FromUtf8Error> for ParsingError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self {
            kind: ParsingErrorKind::NonValidUtf8Read(value),
            origin: String::new(),
        }
    }
}
