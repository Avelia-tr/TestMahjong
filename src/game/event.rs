pub trait NeedDiscard {
    type Round: ProcessRound;
    fn discard(self) -> Self::Round; // [TODO] decision parameter
}

pub trait NeedCalls {
    type Round: ProcessRound;
    fn discard(self) -> Self::Round; // [TODO] decision parameter
}
pub trait NeedChankan {
    type Round: ProcessRound;
    fn ron(self) -> Self::Round;
}

pub trait ProcessRound {
    type Draw: NeedDiscard<Round = Self>;
    type Call: NeedCalls<Round = Self>;
    type ChanKan: NeedChankan<Round = Self>;
    fn tick(self) -> RoundEvent<Self::Draw, Self::Call, Self::ChanKan>;
}

pub enum WinInfo {}

pub enum RoundEvent<RoundDraw: NeedDiscard, RoundCall: NeedCalls, Chankan: NeedChankan> {
    Draw(RoundDraw),
    Kan(Chankan),
    Call(RoundCall),
    EndState(WinInfo),
}
