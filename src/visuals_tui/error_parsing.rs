use std::io::{self, Read, StdinLock, stdin};

use crate::visuals_tui::{ansi_codes, utils::Rawmodder};

macro_rules! expect_char {
    ($char:expr, $reader:expr) => {
        if !consume_char($char, &mut $reader)? {
            // TODO unexpected character
            return Ok(());
        }
    };
}

pub(super) fn parse_error(_raw_mod: &Rawmodder) -> io::Result<()> {
    let mut answer_channel = stdin().lock();

    expect_char!(ansi_codes::ESC, answer_channel);
    expect_char!(b'_', answer_channel);
    expect_char!(b'G', answer_channel);

    // i= digits ?

    expect_char!(b';', answer_channel);

    todo!()
}

fn consume_char(char: u8, answer_channel: &mut StdinLock) -> io::Result<bool> {
    let mut buf = [0u8];

    answer_channel.read_exact(&mut buf)?;

    Ok(buf[0] == char)
}
