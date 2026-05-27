use crate::game::tiles::MahjongTile;

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
    pub fn is_closed(&self) -> bool {
        self.hand.iter().all(|x| x.is_closed())
    }

    pub fn draw(&mut self, drawn_tile: MahjongTile) -> DrawResult {
        todo!()
    }

    pub fn call(
        &mut self,
        possibilities: CallPossibility,
        discarded_tile: MahjongTile,
    ) -> CallResult {
        todo!()
    }

    pub fn can_call(&self) -> bool {
        todo!()
    }
}
