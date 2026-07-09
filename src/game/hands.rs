use std::io::Write;

use crate::game::{
    event_data::PlayerId,
    tiles::{MahjongTile, Wind},
};

// [todo] revise API ?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandBlock {
    Unit(MahjongTile),
    ClosedKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),

    Chii(MahjongTile, MahjongTile, MahjongTile),
    Pon(MahjongTile, MahjongTile, MahjongTile),
    OpenKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),
    AddedKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),
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
    pub fn make_riichi(&mut self) {
        self.riichi = true;
    }

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
                // ?
                todo!()
            };

            self.hand.swap_remove(x);
        }

        self.hand.push(HandBlock::Pon(tiles[0], tiles[1], tiles[2]));
    }

    // [todo] revise API ?
    pub fn chii(&mut self, tiles: [MahjongTile; 3]) {
        todo!();
    }

    pub fn can_call(&self) -> bool {
        todo!()
    }

    pub fn add_tile(&mut self, tile: MahjongTile) {
        self.hand.push(HandBlock::Unit(tile));
    }
}
