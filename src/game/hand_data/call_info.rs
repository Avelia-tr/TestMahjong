use std::{cmp, fmt::Debug};

use crate::game::{event_data::CallInfo, tiles::MahjongTile};

// Invariant to hold : this is a call that was made
#[derive(Clone, Copy)]
pub struct MadeCall<T: CallInfo + Clone + Copy> {
    tile: MahjongTile,
    info: T,
}

impl<T: CallInfo + Clone + Copy + Debug> Debug for MadeCall<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MadeCall")
            .field("tile", &self.tile)
            .field("info", &self.info)
            .finish()
    }
}

impl<T: CallInfo + Clone + Copy> MadeCall<T> {
    pub fn new(tile: MahjongTile, info: T) -> Self {
        Self { tile, info }
    }

    pub fn info(&self) -> &T {
        &self.info
    }

    pub fn tile(&self) -> MahjongTile {
        self.tile
    }
}

impl<T: CallInfo + Clone + Copy> Ord for MadeCall<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.tile.cmp(&other.tile)
    }
}

impl<T: CallInfo + Clone + Copy> PartialOrd for MadeCall<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: CallInfo + Clone + Copy> Eq for MadeCall<T> {}

impl<T: CallInfo + Clone + Copy> PartialEq for MadeCall<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tile == other.tile
    }
}
