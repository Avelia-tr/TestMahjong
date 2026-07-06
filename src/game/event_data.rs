use crate::game::{
    hands::MahjongHand,
    tiles::{MahjongTile, Wind},
};

enum GameResult {
    Ron { winning_hand: PlayerId },
    Tsumo(PlayerId),
    //not dealing with kan
    //RinshanKaihou,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlayerId(u32);

pub struct Player {
    id: PlayerId,
    pub score: i32,
}

// Maybe have this as generic ?
pub enum DiscardDecision {
    Discard(MahjongTile),
    Kan(MahjongTile),
    Tsumo,
}

#[derive(PartialEq, Eq, Clone, Copy)]
// [TODO] Should show options for options other than "ron"
pub enum Call {
    Chii(PlayerId),
    Pon(PlayerId),
    Kan(PlayerId),
    Ron(PlayerId),
}

pub struct CallDecision {
    pub calls_made: Vec<Call>,
}

impl CallDecision {
    pub fn is_empty(&self) -> bool {
        self.calls_made.is_empty()
    }

    pub fn get_rons(&self) -> Vec<PlayerId> {
        self.calls_made
            .iter()
            .filter_map(|&x| match x {
                Call::Ron(y) => Some(y),
                _ => None,
            })
            .collect()
    }
}
