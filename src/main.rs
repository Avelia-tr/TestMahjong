use std::{
    io::{self, Read, Write},
    thread::sleep,
    time::Duration,
};

use crate::visuals_tui::{
    display_example::example_moving_image,
    kitty_input::{ComprehensiveInput, ComprehensiveInputArgs},
    utils::Rawmodder,
};

#[allow(unused)]
mod game;

#[allow(dead_code)]
mod visuals_tui;

const MEOW_EXTENSIVE_START: &[u8] = b"\x1B\x5B>8u";
const MEOW_EXTENSIVE_STOP: &[u8] = b"\x1B\x5B>1u";

const ARGS_INPUT: ComprehensiveInputArgs = ComprehensiveInputArgs {
    disambiguate_escape_codes: true,
    event_types: false,
    alternate_keys: false,
    all_keys_as_escape_codes: true,
    associated_text: false,
};

fn main() {
    example_moving_image().unwrap();
    //return;
    //let meow = display_example::example_moving_image();
    //

    //let guard = Rawmodder::enable().unwrap(); let meow = ComprehensiveInput::enable(ARGS_INPUT).unwrap(); for i in 0..5 { println!("{i}:{:?}", meow.read_input(&guard).unwrap()); } drop(meow);
}
