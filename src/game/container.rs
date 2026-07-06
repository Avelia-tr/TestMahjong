use crate::game::{hands::MahjongHand, tiles::Wind};

pub struct FourPlayerStorage {
    east_player: MahjongHand,
    south_player: MahjongHand,
    west_player: MahjongHand,
    north_player: MahjongHand,
}

impl FourPlayerStorage {
    pub fn hand_from_wind_mut(&mut self, wind: Wind) -> &mut MahjongHand {
        match wind {
            Wind::East => &mut self.east_player,
            Wind::South => &mut self.south_player,
            Wind::West => &mut self.west_player,
            Wind::North => &mut self.north_player,
        }
    }

    pub fn hand_from_wind(&self, wind: Wind) -> &MahjongHand {
        match wind {
            Wind::East => &self.east_player,
            Wind::South => &self.south_player,
            Wind::West => &self.west_player,
            Wind::North => &self.north_player,
        }
    }

    pub fn all_calls(&self, wind: Wind) -> Vec<Wind> {
        let mut res = vec![];

        let mut wind_next = wind.get_next();

        if self.hand_from_wind(wind_next).can_call() {
            res.push(wind_next);
        }

        wind_next = wind.get_next();

        if self.hand_from_wind(wind_next).can_call() {
            res.push(wind_next);
        }

        wind_next = wind.get_next();

        if self.hand_from_wind(wind_next).can_call() {
            res.push(wind_next);
        }
        res
    }
}
