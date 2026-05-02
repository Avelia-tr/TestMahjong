use std::{io, marker::PhantomData};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Rawmodder(());

impl Rawmodder {
    pub fn enable() -> io::Result<Rawmodder> {
        enable_raw_mode()?;
        Ok(Rawmodder(()))
    }
}

impl Drop for Rawmodder {
    fn drop(&mut self) {
        _ = disable_raw_mode();
    }
}
