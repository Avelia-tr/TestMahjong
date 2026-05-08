use crate::visuals_tui::display_example;

#[allow(unused)]
mod game;

#[allow(dead_code)]
mod visuals_tui;

fn main() {
    let meow = display_example::example_moving_image();

    println!("{:?}", meow);
}
