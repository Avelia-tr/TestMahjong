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

enum DrawResult {
    Tsumogiri(MahjongTile),
    Tedashi(MahjongTile),
    Riichi(MahjongTile),
    Win,
    Kan(MahjongTile),
}

pub struct MahjongHand {
    hand: Vec<HandBlock>,
    riichi: bool,
}

impl HandBlock {
    fn is_closed(&self) -> bool {
        matches!(self, Self::Unit(_) | Self::ClosedKan(_, _, _, _))
    }
}

impl MahjongHand {
    fn is_closed(&self) -> bool {
        self.hand.iter().all(|x| x.is_closed())
    }

    fn draw(&mut self) -> () {}
}
