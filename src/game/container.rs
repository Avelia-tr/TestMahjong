use crate::game::{
    event_data::{DiscardTile, Player, PlayerId},
    hands::MahjongHand,
    tiles::Wind,
};

pub struct FourPlayerStorage {
    east_player: PlayerStorage,
    south_player: PlayerStorage,
    west_player: PlayerStorage,
    north_player: PlayerStorage,
}

pub struct PlayerStorage {
    pub hand: MahjongHand,
    player: Player,
    discard: Vec<DiscardTile>,
}

impl PlayerStorage {
    pub fn get_id(&self) -> PlayerId {
        self.player.get_id()
    }

    pub fn discard_tile(&mut self, tile: DiscardTile) {
        self.discard.push(tile);
    }
}

impl FourPlayerStorage {
    pub fn player_from_wind_mut(&mut self, wind: Wind) -> &mut PlayerStorage {
        match wind {
            Wind::East => &mut self.east_player,
            Wind::South => &mut self.south_player,
            Wind::West => &mut self.west_player,
            Wind::North => &mut self.north_player,
        }
    }

    pub fn player_from_wind(&self, wind: Wind) -> &PlayerStorage {
        match wind {
            Wind::East => &self.east_player,
            Wind::South => &self.south_player,
            Wind::West => &self.west_player,
            Wind::North => &self.north_player,
        }
    }

    pub fn hand_from_wind_mut(&mut self, wind: Wind) -> &mut MahjongHand {
        &mut self.player_from_wind_mut(wind).hand
    }

    pub fn hand_from_wind(&self, wind: Wind) -> &MahjongHand {
        &self.player_from_wind(wind).hand
    }

    pub fn hand_from_id(&self, id: PlayerId) -> Option<&MahjongHand> {
        if self.east_player.player.get_id() == id {
            Some(&self.east_player.hand)
        } else if self.south_player.player.get_id() == id {
            Some(&self.south_player.hand)
        } else if self.west_player.player.get_id() == id {
            Some(&self.west_player.hand)
        } else if self.north_player.player.get_id() == id {
            Some(&self.north_player.hand)
        } else {
            None
        }
    }

    pub fn hand_from_id_mut(&mut self, id: PlayerId) -> Option<&mut MahjongHand> {
        if self.east_player.player.get_id() == id {
            Some(&mut self.east_player.hand)
        } else if self.south_player.player.get_id() == id {
            Some(&mut self.south_player.hand)
        } else if self.west_player.player.get_id() == id {
            Some(&mut self.west_player.hand)
        } else if self.north_player.player.get_id() == id {
            Some(&mut self.north_player.hand)
        } else {
            None
        }
    }

    pub fn wind_from_id(&self, id: PlayerId) -> Option<Wind> {
        if self.east_player.player.get_id() == id {
            Some(Wind::East)
        } else if self.south_player.player.get_id() == id {
            Some(Wind::South)
        } else if self.west_player.player.get_id() == id {
            Some(Wind::West)
        } else if self.north_player.player.get_id() == id {
            Some(Wind::North)
        } else {
            None
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
