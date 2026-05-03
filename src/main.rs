use std::io::{BufRead, Write, stdin, stdout};

use crate::visuals_tui::utils::Rawmodder;

#[allow(unused)]
mod game;

#[allow(dead_code)]
mod visuals_tui;

fn main() {
    let guard = Rawmodder::enable();

    stdout()
        .write_all(b"\x1B_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1B\\")
        .expect("no io error");

    stdout().flush().expect("valid flush");

    let mut buf = vec![];
    stdin()
        .lock()
        .read_until(b'\x5c', &mut buf)
        .expect("no io error");

    drop(guard);

    let answer = String::from_utf8(buf)
        .expect("valid utf8")
        .replace("\x1B", "␛");

    println!("answer : {}", answer);
}
