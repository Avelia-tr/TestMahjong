use std::marker::PhantomData;

use crate::game::{
    hands::{self, DrawResult, MahjongHand},
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

enum DiscardResult {
    Ron,
    NewWind(Wind),
}

struct FourPlayerStorage {
    east_player: MahjongHand,
    south_player: MahjongHand,
    west_player: MahjongHand,
    north_player: MahjongHand,
}

struct FourPlayerRiichi<T: MahjongWall, State> {
    players: FourPlayerStorage,
    wall: T,
    _marker: PhantomData<State>,
}

impl FourPlayerStorage {
    fn get_player_mut(&mut self, wind: Wind) -> &mut MahjongHand {
        match wind {
            Wind::East => &mut self.east_player,
            Wind::South => &mut self.south_player,
            Wind::West => &mut self.west_player,
            Wind::North => &mut self.north_player,
        }
    }

    fn get_player(&self, wind: Wind) -> &MahjongHand {
        match wind {
            Wind::East => &self.east_player,
            Wind::South => &self.south_player,
            Wind::West => &self.west_player,
            Wind::North => &self.north_player,
        }
    }

    fn all_calls(&self, wind: Wind) -> Vec<Wind> {
        let mut res = vec![];

        let mut wind_next = wind.get_next();

        if self.get_player(wind_next).can_call() {
            res.push(wind_next);
        }

        wind_next = wind.get_next();

        if self.get_player(wind_next).can_call() {
            res.push(wind_next);
        }

        wind_next = wind.get_next();

        if self.get_player(wind_next).can_call() {
            res.push(wind_next);
        }
        res
    }
}
