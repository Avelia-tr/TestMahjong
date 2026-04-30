use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub(crate) const ESC: &[u8] = b"\x1B";

pub struct Rawmodder;

impl Rawmodder {
    pub fn enable() -> io::Result<Rawmodder> {
        enable_raw_mode()?;
        Ok(Rawmodder)
    }
}

impl Drop for Rawmodder {
    fn drop(&mut self) {
        _ = disable_raw_mode();
    }
}
