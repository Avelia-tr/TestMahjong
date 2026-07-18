use crate::game::{
    event_data::{DiscardTile, Player, PlayerId},
    hand_data::hand_block::HandBlock,
    hands::MahjongHand,
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

#[derive(Debug)]
pub struct FourPlayerStorage {
    east: PlayerStorage,
    south: PlayerStorage,
    west: PlayerStorage,
    north: PlayerStorage,
}

#[derive(Debug)]
pub struct PlayerStorage {
    pub hand: MahjongHand,
    player: Player,
    pub discard: Vec<DiscardTile>,
}

impl PlayerStorage {
    pub fn new(hand: MahjongHand, player: Player) -> Self {
        Self {
            hand,
            player,
            discard: vec![],
        }
    }

    pub fn get_id(&self) -> PlayerId {
        self.player.get_id()
    }

    pub fn discard_tile(&mut self, tile: DiscardTile) {
        self.discard.push(tile);
    }

    pub fn call_last(&mut self) -> Option<MahjongTile> {
        match self.discard.last_mut() {
            Some(d_tile) => {
                if d_tile.stolen {
                    None
                } else {
                    d_tile.stolen = true;
                    Some(d_tile.tile)
                }
            }
            None => None,
        }
    }
}

impl FourPlayerStorage {
    pub fn new<Wall: MahjongWall>(wall: &mut Wall) -> Option<Self> {
        println!("[Warning] improper initialization should take some already existing player");

        let mut east_p = vec![];
        let mut south_p = vec![];
        let mut west_p = vec![];
        let mut north_p = vec![];

        // every player takes 4 tiles each after the other
        for _ in 0..3 {
            for _ in 0..4 {
                east_p.push(HandBlock::Unit(wall.draw()?));
            }
            for _ in 0..4 {
                south_p.push(HandBlock::Unit(wall.draw()?));
            }
            for _ in 0..4 {
                west_p.push(HandBlock::Unit(wall.draw()?));
            }
            for _ in 0..4 {
                north_p.push(HandBlock::Unit(wall.draw()?));
            }
        }

        // final tile
        east_p.push(HandBlock::Unit(wall.draw()?));
        south_p.push(HandBlock::Unit(wall.draw()?));
        west_p.push(HandBlock::Unit(wall.draw()?));
        north_p.push(HandBlock::Unit(wall.draw()?));

        Some(Self {
            east: PlayerStorage::new(MahjongHand::new(east_p), Player::new_temp(1)),
            south: PlayerStorage::new(MahjongHand::new(south_p), Player::new_temp(2)),
            west: PlayerStorage::new(MahjongHand::new(west_p), Player::new_temp(3)),
            north: PlayerStorage::new(MahjongHand::new(north_p), Player::new_temp(4)),
        })
    }
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
