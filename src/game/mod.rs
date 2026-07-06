use crate::game::{
    hands::MahjongHand,
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

mod container;
mod event;
mod event_data;
mod four_player_mahjong;
#[allow(unused)]
mod hands;
#[allow(unused)]
mod tiles;
#[allow(unused)]
mod wall;

enum GameResult {
    Ron { winning_hand: MahjongHand },
    Tsumo(MahjongHand, Wind),
    RinshanKaihou,
}
