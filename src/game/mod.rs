use crate::game::{hands::MahjongHand, wall::MahjongWall};

#[allow(unused)]
mod hands;
#[allow(unused)]
mod tiles;
#[allow(unused)]
mod wall;

struct FourPlayerRiichi {
    east_player: MahjongHand,
    south_player: MahjongHand,
    west_player: MahjongHand,
    north_player: MahjongHand,
    wall: MahjongWall,
}
