use std::{cmp, io::Write};

use crate::game::{
    event_data::{CallInfo, ChiiCallInfo, PlayerId, PonCallInfo},
    hand_data::{call_info::MadeCall, hand_block::HandBlock},
    tiles::{MahjongTile, Wind},
};

#[derive(Debug)]
pub struct MahjongHand {
    hand: Vec<HandBlock>,
    riichi: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallPossibility {
    pon: bool,
    chii: bool,
    kan: bool,
}

impl MahjongHand {
    pub fn new(hand: Vec<HandBlock>) -> Self {
        Self {
            hand,
            riichi: false,
        }
    }

    pub fn make_riichi(&mut self) {
        self.riichi = true;
    }

    pub fn is_closed(&self) -> bool {
        self.hand.iter().all(HandBlock::is_closed)
    }

    pub fn pon(&mut self, tile_called: MahjongTile, info: PonCallInfo) {
        let mut new_hand: Vec<HandBlock> = self
            .hand
            .iter()
            .filter_map(|x| match x {
                HandBlock::Unit(mahjong_tile) if info.tiles.contains(mahjong_tile) => None,
                x => Some(*x),
            })
            .collect();

        debug_assert!(new_hand.len() + 2 == self.hand.len());

        new_hand.push(HandBlock::Pon(MadeCall::new(tile_called, info)));

        self.hand = new_hand;
    }

    // [todo] revise API ?
    pub fn chii(&mut self, tile_called: MahjongTile, info: ChiiCallInfo) {
        debug_assert!(
            is_valid_chii(tile_called, info),
            "assertion failed, not a valid chii"
        );

        let mut new_hand: Vec<HandBlock> = self
            .hand
            .iter()
            .filter_map(|x| match x {
                HandBlock::Unit(mahjong_tile) if info.tiles.contains(mahjong_tile) => None,
                x => Some(*x),
            })
            .collect();

        debug_assert!(new_hand.len() + 2 == self.hand.len());

        new_hand.push(HandBlock::Chii(MadeCall::new(tile_called, info)));

        self.hand = new_hand;
        todo!("do_chii");
    }

    pub fn add_tile(&mut self, tile: MahjongTile) {
        self.hand.push(HandBlock::Unit(tile));
    }
}

fn is_valid_chii(tile_called: MahjongTile, info: ChiiCallInfo) -> bool {
    let mut test = [&[tile_called], &info.tiles[..]].concat();

    test.sort();

    for [prev, next] in test.array_windows() {
        if !prev.follows(*next) {
            return false;
        }
    }

    true
}
