use crate::visuals_tui::window_size::get_window_size;

#[allow(unused)]
mod game;

#[allow(dead_code)]
mod visuals_tui;

fn main() {
    let a = get_window_size().expect("nothing weird to happen");
    println!("answer :{}|", String::from_utf8(a).expect("valid shit"));
}
