use core::str;
use std::{io, string};

pub struct OKAnswer;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum MessageError {
    IO(io::Error),
    Terminal(TerminalError),
    Parsing(ParsingError),
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum LoadError {
    MessageFailed(MessageError),
    FileNotFound,
    IO(io::Error),
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

#[derive(derive_more::Display, derive_more::Error, Debug)]
pub enum AnswerReadingError {
    IO(io::Error),
    NonUtf8(string::FromUtf8Error),
}

impl From<string::FromUtf8Error> for AnswerReadingError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::NonUtf8(value)
    }
}
impl From<io::Error> for AnswerReadingError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

#[derive(derive_more::Display, derive_more::Error, Debug, derive_more::From)]
pub enum ParsingError {
    FailedReading(AnswerReadingError),
    NoPrefixFound,
    NoSuffixFound,
    NoDelimiterFound,
    InvalidlyformedTerminalAnswer,
}
