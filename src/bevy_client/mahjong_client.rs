use bevy::prelude::*;

use crate::{
    bevy_client::ui::tile_button::{self, TilesPlugins, make_button},
    game::tiles::{DragonTiles, MahjongTile, MahjongTilesIdentity, NumberTile, Wind},
};

pub struct ClientPlugins;

impl Plugin for ClientPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(TilesPlugins)
            .add_systems(Startup, base_scene.spawn());
    }
}

// Main File for the big dependencies : camera, main setups, Major State etc...

fn base_scene() -> impl SceneList {
    bsn_list! {
        Camera2d,
        test_1(),
    }
}

fn test_1() -> impl Scene {
    bsn! {
        Node {
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::Stretch,
            align_items: AlignItems::Center,
            top: percent(50),
            column_gap: px(10),
            padding: UiRect::all(px(20)),
            margin: UiRect::all(px(20)),
            flex_wrap: FlexWrap::Wrap,
        }
        BackgroundColor(Color::Oklcha(Oklcha { lightness: 0.3, chroma: 0.2, hue: 240., alpha: 1. }))

        Children [
            make_button(MahjongTile::new(MahjongTilesIdentity::Dragon(DragonTiles::Red))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Dragon(DragonTiles::Green))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Dragon(DragonTiles::White))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Wind(Wind::East))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Wind(Wind::South))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Wind(Wind::West))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Wind(Wind::North))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Man(NumberTile::new(2).expect("passed 2")))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Sou(NumberTile::new(2).expect("passed 2")))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Pin(NumberTile::new(2).expect("passed 2")))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Man(NumberTile::new(3).expect("passed 3")))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Sou(NumberTile::new(3).expect("passed 3")))),
            make_button(MahjongTile::new(MahjongTilesIdentity::Pin(NumberTile::new(3).expect("passed 3")))),
        ]

    }
}
