use crate::game::{
    event_data::{ChiiCallInfo, KanCallInfo, PonCallInfo},
    hand_data::call_info::MadeCall,
    tiles::MahjongTile,
};

// [todo] revise API ?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandBlock {
    Unit(MahjongTile),
    ClosedKan(MahjongTile, MahjongTile, MahjongTile, MahjongTile),

    Chii(MadeCall<ChiiCallInfo>),
    Pon(MadeCall<PonCallInfo>),
    OpenKan(MadeCall<KanCallInfo>),
    AddedKan(MahjongTile, MadeCall<PonCallInfo>),
}

impl HandBlock {
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        matches!(self, Self::Unit(..) | Self::ClosedKan(..))
    }

    #[must_use]
    pub const fn is_open(&self) -> bool {
        !self.is_closed()
    }

    /// Returns `true` if the hand block is [`Unit`].
    ///
    /// [`Unit`]: HandBlock::Unit
    #[must_use]
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit(..))
    }
}
