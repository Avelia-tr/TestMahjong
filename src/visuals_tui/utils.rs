use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled};

pub struct Rawmodder(bool);

impl Rawmodder {
    pub fn enable() -> io::Result<Rawmodder> {
        let was_raw = is_raw_mode_enabled()?;

        enable_raw_mode()?;

        Ok(Rawmodder(was_raw))
    }
}

impl Drop for Rawmodder {
    fn drop(&mut self) {
        if self.0 {
            return;
        }
        _ = disable_raw_mode();
    }
}
