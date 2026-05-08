use std::io::{self, Write};

use crate::visuals_tui::{ansi_codes, utils::Rawmodder};

pub struct DisplayScreen(());

impl DisplayScreen {
    pub fn enable() -> io::Result<DisplayScreen> {
        let guard = Rawmodder::enable()?;

        let mut out = io::stdout().lock();

        out.save_screen()?;
        out.save_cursor()?;
        out.erase_screen()?;
        out.move_cursor_home()?;

        //out.write_all(b"\x1B[20;20H")?;

        drop(guard);
        Ok(DisplayScreen(()))
    }
}

impl Drop for DisplayScreen {
    fn drop(&mut self) {
        let guard = Rawmodder::enable();

        let mut out = io::stdout().lock();
        _ = out.restore_screen();
        _ = out.restore_cursor();
        _ = out.erase_screen_to_end();

        drop(guard);
    }
}

#[allow(unused)]
pub trait OutUtils {
    fn erase_screen(&mut self) -> io::Result<()>;
    fn erase_screen_to_end(&mut self) -> io::Result<()>;
    fn move_cursor_home(&mut self) -> io::Result<()>;
    fn save_screen(&mut self) -> io::Result<()>;
    fn restore_screen(&mut self) -> io::Result<()>;
    fn save_cursor(&mut self) -> io::Result<()>;
    fn restore_cursor(&mut self) -> io::Result<()>;
    fn move_cursor_to(&mut self, x: u32, y: u32) -> io::Result<()>;
}

impl<T: Write> OutUtils for T {
    fn erase_screen(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::ERASE_SCREEN)
    }

    fn save_screen(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::SAVE_SCREEN)
    }
    fn restore_screen(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::RESTORE_SCREEN)
    }

    fn move_cursor_home(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::MOVE_CURSOR_HOME)
    }

    fn save_cursor(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::SAVE_CURSOR)
    }

    fn restore_cursor(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::RESTORE_CURSOR)
    }

    fn erase_screen_to_end(&mut self) -> io::Result<()> {
        self.write_all(ansi_codes::ERASE_SCREEN_TO_END)
    }

    fn move_cursor_to(&mut self, x: u32, y: u32) -> io::Result<()> {
        self.write_all(
            &[
                ansi_codes::START_MOVE_CURSOR_TO,
                &y.to_string().into_bytes(),
                ansi_codes::DELIMITER,
                &x.to_string().into_bytes(),
                ansi_codes::END_MOVE_CURSOR_TO,
            ]
            .concat(),
        )
    }
}
