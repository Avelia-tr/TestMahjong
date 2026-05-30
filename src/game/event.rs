use std::marker::PhantomData;

use crate::game::tiles::Wind;

macro_rules! type_bounds {
    ($bind:ident) => {
        type Draw: NeedDiscard<$bind = Self>;
        type Call: NeedCalls<$bind = Self>;
        type ChanKan: NeedChankan<$bind = Self>;
        type Special: NeedSpecial<$bind = Self>;
    };
}

macro_rules! OutsideQuery {
    () => {
        crate::game::event::RoundEvent<Self::Draw, Self::Call, Self::ChanKan, Self::Special>
    };
}

pub struct DiscardDecision {
    // define the info we pass to choose the discarded tile
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Call {
    Chii(Wind),
    Pon(Wind),
    Kan(Wind),
    Ron(Wind),
}

pub struct CallsDecision {
    pub calls: Vec<Call>,
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
    fn discard(self, info: CallsDecision) -> OutsideQuery!(); // [TODO] decision parameter
}
pub trait NeedChankan {
    type_bounds!(ChanKan);
    fn ron(self, info: ChanKanDecision) -> OutsideQuery!();
}

pub trait NeedSpecial {
    type_bounds!(Special);
    fn give_context(self) -> OutsideQuery!();
}

enum Never {}

pub struct NoSpecialRequest<RoundDraw: NeedDiscard, RoundCall: NeedCalls, Chankan: NeedChankan> {
    absurd: Never,
    _tag_draw: PhantomData<RoundDraw>,
    _tag_call: PhantomData<RoundCall>,
    _tag_chankan: PhantomData<Chankan>,
}

impl<
    RoundDraw: NeedDiscard<Special = Self>,
    RoundCall: NeedCalls<Special = Self>,
    Chankan: NeedChankan<Special = Self>,
> NeedSpecial for NoSpecialRequest<RoundDraw, RoundCall, Chankan>
{
    type Special = Self;
    type Draw = RoundDraw;
    type Call = RoundCall;
    type ChanKan = Chankan;
    fn give_context(self) -> OutsideQuery!() {
        unreachable!("this type should not be constructable")
    }
}

pub enum WinInfo {}

pub enum RoundEvent<
    RoundDraw: NeedDiscard,
    RoundCall: NeedCalls,
    Chankan: NeedChankan,
    Special: NeedSpecial,
> {
    Draw(RoundDraw),
    Kan(Chankan),
    Call(RoundCall),
    SpecialRequest(Special),
    EndState(WinInfo),
}
