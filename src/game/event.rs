use std::marker::PhantomData;

use crate::game::tiles::Wind;

macro_rules! type_bounds {
    ($bind:ident) => {
        type Draw: NeedDiscard<$bind = Self>;
        type Call: NeedCalls<$bind = Self>;
        type Special: NeedSpecial<$bind = Self>;
    };
}

macro_rules! OutsideQuery {
    () => {
        crate::game::event::RoundEvent<Self::Draw, Self::Call, Self::Special>
    };
}

pub struct DiscardDecision {
    // define the info we pass to choose the discarded tile
}

pub trait Player {}

pub enum NoCalls {}

impl Player for NoCalls {}

pub struct CallsDecision<T: Player> {
    pub calls: Vec<Call<T>>,
}
pub struct ChanKanDecision {
    pub robbing_players: Vec<Wind>,
}

pub trait NeedDiscard {
    type_bounds!(Draw);
    fn discard(self, info: DiscardDecision) -> OutsideQuery!(); // [TODO] decision parameter
}

pub trait NeedCalls {
    type_bounds!(Call);
    type PlayerType: Player;
    fn take(self, info: CallsDecision<Self::PlayerType>) -> OutsideQuery!(); // [TODO] decision parameter
}

pub trait NeedSpecial {
    type_bounds!(Special);
    fn give_context(self) -> OutsideQuery!();
}

enum Never {}

pub struct NoSpecialRequest<RoundDraw: NeedDiscard, RoundCall: NeedCalls> {
    absurd: Never,
    _tag_draw: PhantomData<RoundDraw>,
    _tag_call: PhantomData<RoundCall>,
}

impl<RoundDraw: NeedDiscard<Special = Self>, RoundCall: NeedCalls<Special = Self>> NeedSpecial
    for NoSpecialRequest<RoundDraw, RoundCall>
{
    type Special = Self;
    type Draw = RoundDraw;
    type Call = RoundCall;
    fn give_context(self) -> OutsideQuery!() {
        unreachable!("this type should not be constructable")
    }
}

pub enum WinInfo {}

pub enum RoundEvent<RoundDraw: NeedDiscard, RoundCall: NeedCalls, Special: NeedSpecial> {
    Draw(RoundDraw),
    Call(RoundCall),
    SpecialRequest(Special),
    EndState(WinInfo),
}
