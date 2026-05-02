use std::io::{self, BufRead, Write, stdin};

use crate::visuals_tui::utils::Rawmodder;

pub struct WindowSize(u16, u16);

pub fn get_window_size() -> std::io::Result<Vec<u8>> {
    let guard = Rawmodder::enable();

    let mut out = io::stdout();
    let message = b"\x1B[14t";

    out.write_all(message)?;
    out.flush()?;

    let mut answer = stdin().lock();
    let mut s = vec![];
    answer.read_until(b't', &mut s)?;

    drop(guard);

    Ok(s.to_vec())
}
