use bevy::{
    color::palettes::{
        css::{BLACK, WHITE_SMOKE},
        tailwind::{GRAY_400, GREEN_600},
    },
    prelude::*,
};

use crate::game::tiles::{MahjongTile, MahjongTilesIdentity, NumberTile};

pub struct TilesPlugins;

impl Plugin for TilesPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, (update_button, update_hover));
    }
}

#[derive(Component, Clone)]
// change to like sprite or something ?
#[require(Text)]
pub struct TileButton {
    pub tile: MahjongTile,
}

impl Default for TileButton {
    fn default() -> Self {
        Self {
            tile: MahjongTile {
                identity: MahjongTilesIdentity::Man(
                    NumberTile::new(1).expect("passed a valid value in"),
                ),
                red: false,
            },
        }
    }
}
fn update_button(
    buttons: Query<(&mut Text, &TileButton), Or<(Added<TileButton>, Changed<TileButton>)>>,
) {
    for (mut text, infos) in buttons {
        text.0 = infos.tile.identity.to_string();
    }
}

pub fn make_button(tile: MahjongTile) -> impl Scene {
    bsn! {
        TileButton { tile: tile}
        Node {
            margin: UiRect::all(px(5)),
            padding:UiRect::all(px(7)),
        }
        BackgroundColor(COLOR_NONE)
        TextColor(BLACK)
        TextLayout {
            justify: Justify::Center
        }
        Button
    }
}

const COLOR_PRESSED: Color = Color::Srgba(GREEN_600);
const COLOR_HOVERED: Color = Color::Srgba(GRAY_400);
const COLOR_NONE: Color = Color::Srgba(WHITE_SMOKE);

fn update_hover(
    buttons: Query<
        (&Interaction, &mut BackgroundColor, &TileButton, &mut Node),
        Changed<Interaction>,
    >,
) {
    for (&int, mut color, tile, mut node) in buttons {
        match int {
            Interaction::Pressed => {
                color.0 = COLOR_PRESSED;
                info!("pressed : {}", tile.tile.identity.to_string());
            }
            Interaction::Hovered => {
                node.bottom = px(10);
                color.0 = COLOR_HOVERED;
            }
            Interaction::None => {
                color.0 = COLOR_NONE;
                node.bottom = px(0);
            }
        }
    }
}
