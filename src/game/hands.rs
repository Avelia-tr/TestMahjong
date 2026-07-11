use std::io::Write;

use crate::game::{
    event_data::{ChiiCallInfo, PlayerId, PonCallInfo},
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
        fn fun_name(x: &HandBlock) -> bool {
            x.is_closed()
        }
        self.hand.iter().all(HandBlock::is_closed)
    }

    pub fn pon(&mut self, info: PonCallInfo) {
        //for i in tiles { let Some(x) = self .hand .iter() .position(|x| matches!(x, HandBlock::Unit(i))) else { // ? todo!() };

        //self.hand.swap_remove(x);
        //}

        //self.hand.push(HandBlock::Pon(tiles[0], tiles[1], tiles[2]));
        todo!("do pon")
    }

    // [todo] revise API ?
    pub fn chii(&mut self, info: ChiiCallInfo) {
        todo!("do_chii");
    }

    pub fn add_tile(&mut self, tile: MahjongTile) {
        self.hand.push(HandBlock::Unit(tile));
    }
}
