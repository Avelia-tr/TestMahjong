use std::io::Write;

use crate::game::tiles::{MahjongTile, Wind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandBlock {
    Unit(MahjongTile),
    ClosedKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),

    Chii(MahjongTile, MahjongTile, MahjongTile),
    Pon(MahjongTile, MahjongTile, MahjongTile),
    OpenKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),
    AddedKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrawResult {
    Tsumogiri(MahjongTile),
    Tedashi(MahjongTile),
    Riichi(MahjongTile),
    Win,
    Kan(MahjongTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CallResult {
    Pass,
    Chii {
        called_tile: MahjongTile,
        othertiles: [MahjongTile; 2],
        wind: Wind,
    },
    Pon {
        called_tile: MahjongTile,
        othertiles: [MahjongTile; 2],
        wind: Wind,
    },
    OpenKan {
        called_tile: MahjongTile,
        othertiles: [MahjongTile; 2],
        wind: Wind,
    },
    AddedKan {
        called_tile: MahjongTile,
        add_tile: MahjongTile,
        othertiles: [MahjongTile; 2],
        wind: Wind,
    },
}

#[derive(Debug)]
pub struct MahjongHand {
    hand: Vec<HandBlock>,
    riichi: bool,
}

impl HandBlock {
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Unit(_) | Self::ClosedKan(_, _, _, _))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallPossibility {
    pon: bool,
    chii: bool,
    kan: bool,
}

impl MahjongHand {
    pub fn is_closed(&self) -> bool {
        self.hand.iter().all(|x| x.is_closed())
    }

    pub fn pon(&mut self, tiles: [MahjongTile; 3]) {
        for i in tiles {
            let Some(x) = self
                .hand
                .iter()
                .position(|x| matches!(x, HandBlock::Unit(i)))
            else {
                todo!()
            };

            self.hand.swap_remove(x);
        }

        self.hand.push(HandBlock::Pon(tiles[0], tiles[1], tiles[2]));
    }

    pub fn chii(&mut self, tiles: [MahjongTile; 3]) {
        todo!();
    }

    pub fn can_call(&self) -> bool {
        todo!()
    }
}
