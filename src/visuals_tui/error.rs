use std::io;

use crate::visuals_tui::image_display_message::{ImageId, ImageNumber};

pub struct OKAnswer(pub ImageId, pub Option<ImageNumber>);

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum LoadError {
    IO(io::Error),
    Display(),
    NotFound,
    UnRecognizedError,
}

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum TerminalError {
    #[error(ignore)]
    NoEntity(String),
    #[error(ignore)]
    InvalidArgument(String),
    #[error(ignore)]
    BadFile(String),
    #[error(ignore)]
    NoData(String),
    #[error(ignore)]
    FileTooLarge(String),
    #[display("found unknown code {code} : {message}")]
    Unknown { code: String, message: String },
}

impl From<(&str, &str)> for TerminalError {
    fn from((error_code, answer_str): (&str, &str)) -> Self {
        let answer = answer_str.to_owned();

        match error_code {
            "ENOENT" => Self::NoEntity(answer),
            "EINVAL" => Self::InvalidArgument(answer),
            "EBADF" => Self::BadFile(answer),
            "ENODATA" => Self::NoData(answer),
            "EFBIG" => Self::FileTooLarge(answer),
            x => Self::Unknown {
                code: x.to_owned(),
                message: answer,
            },
        }
    }
}
