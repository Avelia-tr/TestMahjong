use std::io::{BufRead, Read, Write, stdin, stdout};

use crate::visuals_tui::{display_example, utils::Rawmodder};

#[allow(unused)]
mod game;

#[allow(dead_code)]
mod visuals_tui;

fn main() {
    let meow = display_example::example_1();

    println!("{:?}", meow);
}
