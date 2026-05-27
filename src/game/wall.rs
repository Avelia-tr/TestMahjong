use std::sync::Arc;

use crate::game::tiles::{MahjongTile, MahjongTilesIdentity, NumberTile};

pub trait MahjongWall {
    fn draw(&mut self) -> Option<MahjongTile>;
    fn draw_dead_wall(&mut self) -> MahjongTile;
    fn reveal_dora(&mut self) -> Option<MahjongTile>;

    fn get_dora(&self, index: usize) -> Option<MahjongTile>;
    fn get_ura_dora(&self, index: usize) -> Option<MahjongTile>;
    fn get_visible_doras(&self, index: usize) -> &[MahjongTile];
    fn get_visible_ura_doras(&self, index: usize) -> &[MahjongTile];

    fn poll_tile_remaining(&self) -> usize;
}
