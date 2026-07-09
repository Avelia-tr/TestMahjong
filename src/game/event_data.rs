use crate::game::{
    hands::MahjongHand,
    tiles::{MahjongTile, Wind},
};

pub enum GameResult {
    Ron {
        winning_hand: Vec<PlayerId>,
        target: PlayerId,
    },
    Tsumo(PlayerId),
    Ryuukyoku,
    //not dealing with kan
    //RinshanKaihou,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlayerId(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    id: PlayerId,
    pub score: i32,
}

impl Player {
    pub fn get_id(&self) -> PlayerId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardTileType {
    Tsumogiri(MahjongTile),
    Tedashi(MahjongTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscardTile {
    pub kind: DiscardTileType,
    pub riichi: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KanDecision {}

// Maybe have this as generic ?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardDecision {
    Discard(DiscardTile),
    Kan(KanDecision),
    Tsumo,
}

#[derive(PartialEq, Eq, Clone, Copy)]
// [TODO] Should show options for options other than "ron"
pub enum Call {
    Ron(PlayerId),
    Kan(KanCallInfo),
    Pon(PonCallInfo),
    Chii(ChiiCallInfo),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct KanCallInfo {
    pub origin: PlayerId,
    pub tiles: [MahjongTile; 3],
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct PonCallInfo {
    pub origin: PlayerId,
    pub tiles: [MahjongTile; 2],
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ChiiCallInfo {
    pub origin: PlayerId,
    pub tiles: [MahjongTile; 2],
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

    pub fn get_chii(&self) -> Vec<ChiiCallInfo> {
        self.calls_made
            .iter()
            .filter_map(|&x| match x {
                Call::Chii(y) => Some(y),
                _ => None,
            })
            .collect()
    }

    pub fn get_kans(&self) -> Vec<KanCallInfo> {
        self.calls_made
            .iter()
            .filter_map(|&x| match x {
                Call::Kan(y) => Some(y),
                _ => None,
            })
            .collect()
    }

    pub fn get_pons(&self) -> Vec<PonCallInfo> {
        self.calls_made
            .iter()
            .filter_map(|&x| match x {
                Call::Pon(y) => Some(y),
                _ => None,
            })
            .collect()
    }
}
