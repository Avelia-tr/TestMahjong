use crate::game::{
    rand::{DeterministicRng, Seed, lehmer::LehmerRNG},
    wall_implementation::{
        classic::ClassicWall,
        wall_generation::{ProtoWall, classic_wall_tiles},
    },
};

#[allow(unused)]
mod game;

fn main() {
    //example_moving_image().expect("no io error");

    let mut meow = classic_wall_tiles();
    meow.shuffle(&mut LehmerRNG::new().set_seed(Seed(5)));
    let meow2: String = meow.iter().map(|x| x.identity.to_string()).collect();

    println!("{meow2}");
    println!();
    println!();

    let meowingwall = ClassicWall::new_non_suffling(meow);

    dbg!(meowingwall);
}
