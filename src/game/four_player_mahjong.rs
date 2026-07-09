use std::marker::PhantomData;

use crate::game::{
    container::FourPlayerStorage,
    event::{NormalMahjongMachine, Request},
    event_data::{
        Call, CallDecision, DiscardDecision, DiscardTile, GameResult, KanDecision, PlayerId,
    },
    hands::{self, MahjongHand},
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
        // shuffle the wall
        // fill the hands
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

trait ActiveState {}
impl ActiveState for WaitingForDiscard {}
impl ActiveState for WaitingForCalls {}

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

        if let rons = need.get_rons()
            && !rons.is_empty()
        {
            return FourPMachine::WinState(
                GameResult::Ron {
                    winning_hand: rons,
                    target: self.get_current_player_id(),
                },
                unsafe { self.transition_state(FinalState) },
            );
        }

        if let kans = need.get_kans()
            && !kans.is_empty()
        {
            for i in kans
                .iter()
                .filter_map(|x| self.players.wind_from_id(x.origin).map(|y| (y, x)))
            {
                todo!("refactor I think I've got a better structure in mind")
            }
        }

        if let pons = need.get_pons()
            && !pons.is_empty()
        {
            todo!("deal with pons")
        }

        if let chiis = need.get_kans()
            && !chiis.is_empty()
        {
            todo!("deal with chiis")
        }

        unreachable!("every case checked")
    }
}

impl<Wall: MahjongWall> Request<DiscardDecision> for WallDiscard<Wall> {
    type Machine = FourPMachine<Wall>;

    fn fullfill(self, need: DiscardDecision) -> Self::Machine {
        match need {
            // rework kan container ?
            DiscardDecision::Kan(mahjong_tile) => do_kan(self, mahjong_tile),
            DiscardDecision::Tsumo => do_tsumo(self),
            DiscardDecision::Discard(discard_tile) => do_discard(self, discard_tile),
        }
    }
}

impl<Wall: MahjongWall, State: ActiveState> FourPlayerRiichi<Wall, State> {
    fn draw(mut self, drawing_player: Wind) -> FourPMachine<Wall> {
        let Some(drawn_tile) = self.wall.draw() else {
            return FourPMachine::WinState(GameResult::Ryuukyoku, unsafe {
                self.transition_state(FinalState)
            });
        };

        let player_hand = self.players.hand_from_wind_mut(drawing_player);

        player_hand.add_tile(drawn_tile);

        FourPMachine::Discard(FourPlayerRiichi {
            current_player: drawing_player,
            ..unsafe { self.transition_state(WaitingForDiscard) }
        })
    }
}

impl<Wall: MahjongWall, T> FourPlayerRiichi<Wall, T> {
    unsafe fn transition_state<NextState>(
        self,
        state: NextState,
    ) -> FourPlayerRiichi<Wall, NextState> {
        FourPlayerRiichi {
            players: self.players,
            wall: self.wall,
            current_player: self.current_player,
            main_wind: self.main_wind,
            _marker: state,
        }
    }

    pub fn get_current_player_id(&self) -> PlayerId {
        self.players.player_from_wind(self.current_player).get_id()
    }
}

fn do_kan<Wall: MahjongWall>(state: WallDiscard<Wall>, kantile: KanDecision) -> FourPMachine<Wall> {
    println!("[Warning] non fully rule compliant SHOULD BE REVISED");

    // [TODO] add to kan/ do kan ?

    let current_player = state.current_player;
    // [TODO] implement deadwall draw

    state.draw(current_player)
}

fn do_tsumo<Wall: MahjongWall>(state: WallDiscard<Wall>) -> FourPMachine<Wall> {
    FourPMachine::WinState(GameResult::Tsumo(state.get_current_player_id()), unsafe {
        state.transition_state(FinalState)
    })
}

fn do_discard<Wall: MahjongWall>(
    mut state: WallDiscard<Wall>,
    discard_tile: DiscardTile,
) -> FourPMachine<Wall> {
    let discarder = state.players.player_from_wind_mut(state.current_player);
    // step 1 add tile to discard
    discarder.discard_tile(discard_tile);

    // step 1.5 make it riichi if needed

    if discard_tile.riichi {
        discarder.hand.make_riichi();
    }

    // step 2 verify if any call
    if let can_call = state.players.all_calls(state.current_player)
        && can_call.is_empty()
    {
        // step 2a if any go to call
        FourPMachine::Call(unsafe { state.transition_state(WaitingForCalls) })
    } else {
        // step 2b skip to next draw ?
        let current_player = state.current_player;

        state.draw(current_player)
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

        // run (machine, runner) ?
    }

    // bwa built in
    #[test]
    fn test_b() {
        // noop
    }
}
