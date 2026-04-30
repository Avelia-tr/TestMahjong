use std::sync::Arc;

use crate::game::tiles::{MahjongTile, MahjongTilesIdentity, NumberTile};

pub struct MahjongWall {
    wall_tiles: Arc<[MahjongTile]>,
    current_index: usize,
}

impl MahjongWall {
    fn new(wall_tiles: Arc<[MahjongTile]>) -> Self {
        Self {
            wall_tiles,
            current_index: 0,
        }
    }
}
