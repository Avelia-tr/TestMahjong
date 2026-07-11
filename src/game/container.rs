use crate::game::{
    event_data::{DiscardTile, Player, PlayerId},
    hands::MahjongHand,
    tiles::Wind,
};

pub struct FourPlayerStorage {
    east: PlayerStorage,
    south: PlayerStorage,
    west: PlayerStorage,
    north: PlayerStorage,
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
            Wind::East => &mut self.east,
            Wind::South => &mut self.south,
            Wind::West => &mut self.west,
            Wind::North => &mut self.north,
        }
    }

    pub fn player_from_wind(&self, wind: Wind) -> &PlayerStorage {
        match wind {
            Wind::East => &self.east,
            Wind::South => &self.south,
            Wind::West => &self.west,
            Wind::North => &self.north,
        }
    }

    pub fn hand_from_wind_mut(&mut self, wind: Wind) -> &mut MahjongHand {
        &mut self.player_from_wind_mut(wind).hand
    }

    pub fn hand_from_wind(&self, wind: Wind) -> &MahjongHand {
        &self.player_from_wind(wind).hand
    }

    pub fn hand_from_id(&self, id: PlayerId) -> Option<&MahjongHand> {
        if self.east.player.get_id() == id {
            Some(&self.east.hand)
        } else if self.south.player.get_id() == id {
            Some(&self.south.hand)
        } else if self.west.player.get_id() == id {
            Some(&self.west.hand)
        } else if self.north.player.get_id() == id {
            Some(&self.north.hand)
        } else {
            None
        }
    }

    pub fn hand_from_id_mut(&mut self, id: PlayerId) -> Option<&mut MahjongHand> {
        if self.east.player.get_id() == id {
            Some(&mut self.east.hand)
        } else if self.south.player.get_id() == id {
            Some(&mut self.south.hand)
        } else if self.west.player.get_id() == id {
            Some(&mut self.west.hand)
        } else if self.north.player.get_id() == id {
            Some(&mut self.north.hand)
        } else {
            None
        }
    }

    pub fn wind_from_id(&self, id: PlayerId) -> Option<Wind> {
        if self.east.player.get_id() == id {
            Some(Wind::East)
        } else if self.south.player.get_id() == id {
            Some(Wind::South)
        } else if self.west.player.get_id() == id {
            Some(Wind::West)
        } else if self.north.player.get_id() == id {
            Some(Wind::North)
        } else {
            None
        }
    }

    pub fn all_calls(&self, wind: Wind) -> Vec<Wind> {
        let mut res = vec![];

        let mut wind_next = wind.get_next();
        let _ = self;

        //if self.hand_from_wind(wind_next).can_call() { res.push(wind_next); }

        wind_next = wind.get_next();

        //if self.hand_from_wind(wind_next).can_call() { res.push(wind_next); }

        wind_next = wind.get_next();

        //if self.hand_from_wind(wind_next).can_call() { res.push(wind_next); }
        res
    }
}
