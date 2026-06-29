use crate::game::{hands::MahjongHand, tiles::Wind};

enum GameResult {
    Ron { winning_hand: PlayerId },
    Tsumo(PlayerId),
    //not dealing with kan
    //RinshanKaihou,
}

struct PlayerId(u32);

struct Player {
    id: PlayerId,
    pub score: i32,
}

pub struct DiscardDecision {
    // define the info we pass to choose the discarded tile
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Call {
    Chii(PlayerId),
    Pon(PlayerId),
    Kan(PlayerId),
    Ron(PlayerId),
}
