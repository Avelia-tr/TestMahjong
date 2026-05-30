use std::marker::PhantomData;

use crate::game::{
    container::FourPlayerStorage,
    event::{
        self, Call, CallsDecision, ChanKanDecision, DiscardDecision, NeedCalls, NeedChankan,
        NeedDiscard, NoSpecialRequest,
    },
    hands::{self, DrawResult, MahjongHand},
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

use super::event::RoundEvent;

macro_rules! type_bounds {
    () => {
        type Draw = FourPlayerRiichi<T, WaitingForDiscard>;
        type Call = FourPlayerRiichi<T, WaitingForCalls>;
        type ChanKan = FourPlayerRiichi<T, WaitingForKan>;
        type Special = NoSpecialRequest<Self::Draw, Self::Call, Self::ChanKan>;
    };
}

macro_rules! OutsideQuery {
    () => {
        crate::game::event::RoundEvent<Self::Draw, Self::Call, Self::ChanKan, Self::Special>
    };
}

struct FourPlayerRiichi<T: MahjongWall, State> {
    players: FourPlayerStorage,
    wall: T,
    current_player: Wind,
    main_wind: Wind,
    _marker: PhantomData<State>,
}

struct Ready;
struct WaitingForDiscard;
struct WaitingForCalls;
struct WaitingForKan;

impl<T: MahjongWall> NeedDiscard for FourPlayerRiichi<T, WaitingForDiscard> {
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

impl<T: MahjongWall> NeedCalls for FourPlayerRiichi<T, WaitingForCalls> {
    type_bounds!();
    fn discard(self, info: CallsDecision) -> OutsideQuery!() {
        if !info.calls.is_empty() {
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
        let calls = info
            .calls
            .iter()
            .filter(|x| !matches!(x, Call::Ron(_)))
            .min()
            .expect("we already checked the possibilities of no matches");

        todo!()
    }
}

impl<T: MahjongWall> NeedChankan for FourPlayerRiichi<T, WaitingForKan> {
    type_bounds!();
    fn ron(self, chan_kan_decision: ChanKanDecision) -> OutsideQuery!() {
        if chan_kan_decision.robbing_players.is_empty() {
            return RoundEvent::Draw(FourPlayerRiichi {
                _marker: PhantomData::<WaitingForDiscard>,
                players: self.players,
                wall: self.wall,
                current_player: self.current_player.get_next(),
                main_wind: self.main_wind,
            });
        }

        // robbing a kan
        RoundEvent::EndState(todo!())
    }
}
