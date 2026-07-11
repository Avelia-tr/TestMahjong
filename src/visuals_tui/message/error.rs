use core::str;
use std::{io, string};

use crate::visuals_tui::message::{EncodeMessage, Message};

pub struct OKAnswer;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("message : {message} failed because : {kind}")]
pub struct MessageError {
    message: String,
    kind: MessageErrorKind,
}

pub trait ToMessageError<T> {
    fn to_message_error(self, message: &Message) -> Result<T, MessageError>;
    fn to_error_no_msg(self) -> Result<T, MessageError>;
}

impl<T, T2: Into<MessageErrorKind>> ToMessageError<T> for Result<T, T2> {
    fn to_message_error(self, message: &Message) -> Result<T, MessageError> {
        self.map_err(|kind| MessageError {
            message: String::from_utf8(message.encode())
                .expect("message.encode should be valid utf-8")
                .replace('\x1b', "␛"),
            kind: kind.into(),
        })
    }

    fn to_error_no_msg(self) -> Result<T, MessageError> {
        self.map_err(|kind| MessageError {
            message: String::new(),
            kind: kind.into(),
        })
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum MessageErrorKind {
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
