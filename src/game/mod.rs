use crate::game::{hands::MahjongHand, tiles::Wind, wall::MahjongWall};

#[allow(unused)]
mod hands;
#[allow(unused)]
mod tiles;
#[allow(unused)]
mod wall;

struct FourPlayerStorage {
    east_player: MahjongHand,
    south_player: MahjongHand,
    west_player: MahjongHand,
    north_player: MahjongHand,
}

struct FourPlayerRiichi<T: MahjongWall> {
    players: FourPlayerStorage,
    wall: T,
}

impl<T: MahjongWall> FourPlayerRiichi<T> {
    fn game(mut self) {
        let mut current_player = Wind::East;

        while let Some(current_tile) = self.wall.draw() {
            println!("meow");

            let drawer = self.players.get_player_mut(current_player);

            let result = drawer.draw(current_tile);

            match result {
                hands::DrawResult::Tsumogiri(mahjong_tile) => todo!(),
                hands::DrawResult::Tedashi(mahjong_tile) => todo!(),
                hands::DrawResult::Riichi(mahjong_tile) => todo!(),
                hands::DrawResult::Kan(mahjong_tile) => todo!(),
                hands::DrawResult::Win => todo!(),
            }

            current_player = current_player.get_next();
        }

        // noten
    }
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
