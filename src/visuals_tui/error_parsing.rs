use core::str;
use std::{
    char,
    io::{self, BufRead},
};

use crate::visuals_tui::{
    error::{OKAnswer, ParsingError, ParsingErrorKind},
    image_display_message::{ImageId, ImageNumber},
    utils::Rawmodder,
};

type ParseResult<T> = Result<T, ParsingError>;

pub(super) fn parse_error_kitty(_raw_mod: &Rawmodder) -> ParseResult<OKAnswer> {
    let answer = fetch_answer()?;

    let answer_stripped = answer
        .strip_prefix("\x1B_G")
        .expect("[TODO] handle no prefix found")
        .strip_suffix("\x1B\x5c")
        .expect("[TODO] handle no suffix found");

    let Some((parameter, answer)) = answer.split_once(";") else {
        todo!()
    };

    // do shit with param ?

    if answer == "OK" {
        todo!()
    }

    // parse answer

    todo!()
}
