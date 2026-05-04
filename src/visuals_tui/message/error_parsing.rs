use std::io::{self, BufRead};

use crate::visuals_tui::{
    message::error::{AnswerReadingError, OKAnswer, ParsingError, TerminalError},
    utils::Rawmodder,
};

pub fn parse_error_kitty(
    _raw_mod: &Rawmodder,
) -> Result<Result<OKAnswer, TerminalError>, ParsingError> {
    let answer = fetch_answer()?;

    let answer_stripped = answer
        .strip_prefix("\x1B_G")
        .ok_or(ParsingError::NoPrefixFound)?
        .strip_suffix("\x1B\x5c")
        .ok_or(ParsingError::NoSuffixFound)?;

    let Some((_parameter, terminal_answer)) = answer_stripped.split_once(";") else {
        return Err(ParsingError::NoDelimiterFound);
    };

    // do shit with param ?

    if terminal_answer == "OK" {
        return Ok(Ok(OKAnswer));
    }

    match terminal_answer.split_once(":") {
        Some(x) => Ok(Err(x.into())),
        None => Err(ParsingError::InvalidlyformedTerminalAnswer),
    }
}

fn fetch_answer() -> Result<String, AnswerReadingError> {
    let mut answer_channel = io::stdin().lock();
    let mut buf = vec![];
    answer_channel.read_until(b'\x5c', &mut buf)?;

    Ok(String::from_utf8(buf)?)
}
