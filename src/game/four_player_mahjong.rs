use std::marker::PhantomData;

use crate::game::{
    container::FourPlayerStorage,
    event::{NormalMahjongMachine, Request},
    event_data::{Call, CallDecision, DiscardDecision},
    hands::{self, DrawResult, MahjongHand},
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

// ignoring "chankan" possibility for now
struct FourPlayerRiichi<T: MahjongWall, State> {
    players: FourPlayerStorage,
    wall: T,
    current_player: Wind,
    main_wind: Wind,
    _marker: State,
}

impl<Wall: MahjongWall> FourPlayerRiichi<Wall, UnreadyState> {
    pub fn new(wall: Wall) -> Self {
        FourPlayerRiichi {
            players: todo!("implement a way to init storage"),
            wall,
            current_player: Wind::East,
            main_wind: Wind::East,
            _marker: UnreadyState,
        }
    }

    pub fn start_game(self) -> FourPMachine<Wall> {
        // setup ?
        FourPMachine::Discard(FourPlayerRiichi {
            _marker: WaitingForDiscard,
            players: self.players,
            wall: self.wall,
            current_player: self.current_player,
            main_wind: self.main_wind,
        })
    }
}

struct UnreadyState;
struct WaitingForDiscard;
struct WaitingForCalls;
struct FinalState;

trait GameState {}
impl GameState for WaitingForDiscard {}
impl GameState for WaitingForCalls {}

type WallDiscard<T> = FourPlayerRiichi<T, WaitingForDiscard>;
type WallCalls<T> = FourPlayerRiichi<T, WaitingForCalls>;
type FinalWall<T> = FourPlayerRiichi<T, FinalState>;

type FourPMachine<T> = NormalMahjongMachine<WallDiscard<T>, WallCalls<T>, FinalWall<T>>;

impl<Wall: MahjongWall> Request<CallDecision> for WallCalls<Wall> {
    type Machine = FourPMachine<Wall>;

    fn fullfill(self, need: CallDecision) -> Self::Machine {
        if need.is_empty() {
            let next_player = self.current_player.get_next();
            return self.draw(next_player);
        }

        todo!()
    }
}

impl<Wall: MahjongWall> Request<DiscardDecision> for WallDiscard<Wall> {
    type Machine = FourPMachine<Wall>;

    fn fullfill(self, need: DiscardDecision) -> Self::Machine {
        todo!()
    }
}

impl<Wall: MahjongWall, State: GameState> FourPlayerRiichi<Wall, State> {
    // should I assume current player is the
    fn draw(mut self, drawing_player: Wind) -> FourPMachine<Wall> {
        let Some(drawn_tile) = self.wall.draw() else {
            todo!("ryukyoku")
        };

        self.current_player = drawing_player;

        let player_hand = self.players.hand_from_wind_mut(drawing_player);

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::wall::impos::mock_wall;

    #[allow(dead_code)]
    fn test_a() {
        let game = FourPlayerRiichi::new(mock_wall());

        let machine: FourPMachine<_> = game.start_game();
    }
}
