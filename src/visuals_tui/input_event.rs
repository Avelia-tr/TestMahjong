use std::io::{self, BufRead};

use derive_more::Display;

use crate::visuals_tui::{ansi_codes::DELIMITER, kitty_input::ComprehensiveInputArgs};

pub enum KeyInput {
    Character(u8),
    Fkey(u8),
    Arrow(ArrowDir),
    Other(u8),
    SpecialKey(SpecialKeys),
    Empty,
}

pub enum SpecialKeys {
    Home,
    End,
}

pub enum ArrowDir {
    Up,
    Down,
    Left,
    Right,
}

pub enum KeyState {
    Press,
    Hold,
    Release,
}

#[derive(Default, Debug)]
pub struct KeyModifier {
    shift: bool,
    alt: bool,
    ctrl: bool,
    super_key: bool,
    hyper: bool,
    meta: bool,
    caps_lock: bool,
    num_lock: bool,
}

pub struct InputEvent {
    key: KeyInput,
    alternate_key_code: Option<char>,
    state: KeyState,
    modifier: KeyModifier,
    text_as_code_point: Option<String>,
}

#[derive(derive_more::Error, Debug, Display)]
pub enum InputEventError {
    IO(io::Error),
    UnrecognizedHeader,
    UnrecognizedPayload,
    #[display("CouldNotParse : {}", 0)]
    #[error(ignore)]
    CouldNotParse(String),
}

impl InputEvent {
    // CSI unicode-key-code[:alternate-key-codes] ; modifiers[:event-type] [; text-as-codepoints] u
    pub fn parse(buffer: Vec<u8>, args: ComprehensiveInputArgs) -> Result<Self, InputEventError> {
        if !has_csi(&buffer) {
            return Err(InputEventError::UnrecognizedHeader);
        }

        let event_state: KeyState;
        let modifier: KeyModifier;
        let key: KeyInput;
        let alternate_key_code: Option<char>;
        let text_code: Option<String>;

        let mut bits = buffer[2..].split(|x| *x == b';');

        let Some(first_part) = bits.next() else {
            return Err(InputEventError::UnrecognizedPayload);
        };

        const DELIMITER_COLON: u8 = b':';

        let codes = first_part.split(|x| *x == DELIMITER_COLON);

        todo!()
    }
}

fn has_csi(buffer: &[u8]) -> bool {
    buffer[0] == 0x1B && buffer[1] == 0x5b
}
