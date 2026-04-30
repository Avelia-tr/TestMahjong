use std::io::{self, BufRead, Write};

use crate::visuals_tui::utils::Rawmodder;

pub fn get_window_size() -> std::io::Result<Vec<u8>> {
    let guard = Rawmodder::enable();
    let mut out = io::stdout();
    let message = b"\x1B[14t";

    out.write_all(message)?;

    out.flush()?;

    let mut answer = io::stdin().lock();
    send_window_message()?;

    let mut s = Vec::with_capacity(64);
    answer.read_until(b'\\', &mut s)?;

    drop(guard);
    Ok(s)
}

fn send_window_message() -> io::Result<()> {
    Ok(())
}
