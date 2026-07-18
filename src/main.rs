use crate::{
    bevy_client::mahjong_client::ClientPlugins,
    game::{
        container::FourPlayerStorage,
        rand::{DeterministicRng, Seed, lehmer::LehmerRNG},
        wall_implementation::{
            classic::ClassicWall,
            wall_generation::{ProtoWall, classic_wall_tiles},
        },
    },
};
use bevy::prelude::*;

mod bevy_client;
mod game;

fn main() {
    let mut rand = LehmerRNG::new().set_seed(Seed(9));

    let meow = classic_wall_tiles().shuffle(&mut rand);
    let meow2: String = meow.iter().map(|x| x.identity.to_string()).collect();

    println!("{meow2}");
    println!();
    println!();

    let mut meowingwall = ClassicWall::new_non_suffling(meow.shuffle(&mut rand));

    let meowing_container = FourPlayerStorage::new(&mut meowingwall);

    dbg!(meowing_container);

    App::new().add_plugins(ClientPlugins).run();
}
