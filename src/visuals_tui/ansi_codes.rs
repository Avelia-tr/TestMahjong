pub const ESC: u8 = b'\x1B';

pub const DELIMITER: &[u8] = b";";

pub const ERASE_SCREEN: &[u8] = b"\x1B[2J";
pub const SAVE_SCREEN: &[u8] = b"\x1B[?47h";
pub const RESTORE_SCREEN: &[u8] = b"\x1B[?47l";

pub const SAVE_CURSOR: &[u8] = b"\x1B[s";
pub const RESTORE_CURSOR: &[u8] = b"\x1B[u";

pub const MOVE_CURSOR_HOME: &[u8] = b"\x1B[H";

pub const START_MOVE_CURSOR_TO: &[u8] = b"\x1B[";
pub const END_MOVE_CURSOR_TO: &[u8] = b"H";

pub const ERASE_SCREEN_TO_END: &[u8] = b"\x1B[J";

pub const HIDE_CURSOR: &[u8] = b"\x1B[?25l";
pub const SHOW_CURSOR: &[u8] = b"\x1B[?25h";
