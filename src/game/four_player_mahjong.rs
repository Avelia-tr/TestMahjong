use std::marker::PhantomData;

use crate::game::{
    container::FourPlayerStorage,
    event::{
        self, Call, CallsDecision, ChanKanDecision, DiscardDecision, NeedCalls, NeedDiscard,
        NoCalls, NoSpecialRequest,
    },
    hands::{self, DrawResult, MahjongHand},
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

use super::event::RoundEvent;

macro_rules! type_bounds {
    () => {
        type Draw = FourPlayerRiichi<Wall, WaitingForDiscard>;
        type Call = FourPlayerRiichi<Wall, WaitingForCalls>;
        type Special = NoSpecialRequest<Self::Draw, Self::Call>;
    };
}

macro_rules! OutsideQuery {
    () => {
        crate::game::event::RoundEvent<Self::Draw, Self::Call, Self::Special>
    };
}

// ignoring "chankan" possibility for now
struct FourPlayerRiichi<T: MahjongWall, State> {
    players: FourPlayerStorage,
    wall: T,
    current_player: Wind,
    main_wind: Wind,
    _marker: PhantomData<State>,
}

struct WaitingForDiscard;
struct WaitingForCalls;

impl<Wall: MahjongWall> NeedDiscard for FourPlayerRiichi<Wall, WaitingForDiscard> {
    type_bounds!();
    fn discard(self, info: DiscardDecision) -> OutsideQuery!() {
        todo!("mutate current player hand, and draw if nobody can call");
        RoundEvent::Call(FourPlayerRiichi {
            _marker: PhantomData::<WaitingForCalls>,
            wall: self.wall,
            main_wind: self.main_wind,
            current_player: self.current_player,
            players: self.players,
        })
    }
}

impl<Wall: MahjongWall> NeedCalls for FourPlayerRiichi<Wall, WaitingForCalls> {
    type_bounds!();
    type PlayerType = NoCalls;
    fn take(self, info: CallsDecision<Self::PlayerType>) -> OutsideQuery!() {
        if !info.calls.is_empty() {
            todo!("draw a tile");
            return RoundEvent::Draw(FourPlayerRiichi {
                _marker: PhantomData::<WaitingForDiscard>,
                players: self.players,
                wall: self.wall,
                current_player: self.current_player.get_next(),
                main_wind: self.main_wind,
            });
        }

        // priority ron > kan > pon > chii

        let rons: Vec<_> = info
            .calls
            .iter
            .filter(|x| matches!(x, Call::Ron(_)))
            .collect();

        if !rons.is_empty() {
            return RoundEvent::EndState(todo!());
        }

        // should have a match
        // do pons ?
        let calls = info.calls.iter().filter(|x| matches!(x, Call::Pon(_)));
        // this does not worky

        todo!("")
    }
}

impl<Wall: MahjongWall> FourPlayerRiichi<Wall, WaitingForDiscard> {
    fn do_rons(&self) {}
}

impl<Wall: MahjongWall, State> FourPlayerRiichi<Wall, State> {
    fn draw(mut self) -> Option<FourPlayerRiichi<Wall, WaitingForDiscard>> {
        let tile = self.wall.draw()?;
        Some(self.add_tile_to_hand(self.current_player, tile))
    }

    fn add_tile_to_hand(
        mut self,
        hand: Wind,
        tile: MahjongTile,
    ) -> FourPlayerRiichi<Wall, WaitingForDiscard> {
        todo!()
    }
}
