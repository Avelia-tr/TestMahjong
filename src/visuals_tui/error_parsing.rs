use core::str;
use std::{
    error::Error,
    io::{self, BufRead},
};

use crate::visuals_tui::{error::TerminalError, utils::Rawmodder};

pub(super) fn parse_error_kitty(
    _raw_mod: &Rawmodder,
) -> Result<Result<OkAnswer, TerminalError>, Box<dyn Error>> {
    let answer = fetch_answer().unwrap();

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

    match answer.split_once(":") {
        Some(x) => x.into(),
        None => todo!(),
    }

    todo!()
}

fn fetch_answer() -> io::Result<String> {
    let mut answer_channel = io::stdin().lock();
    let mut buf = vec![];
    answer_channel.read_until(b'\x5c', &mut buf)?;

    String::from_utf8(buf).map_err(|x| todo!())
}
