use crate::game::{
    event_data::{Call, CallDecision, DiscardDecision, GameResult},
    tiles::Wind,
};

#[must_use]
pub trait Request<Needs> {
    type Machine;
    fn fullfill(self, need: Needs) -> Self::Machine;
}

pub enum NormalMahjongMachine<
    NeedDiscard: Request<DiscardDecision>,
    NeedCall: Request<CallDecision>,
    FinalState,
> {
    Discard(NeedDiscard),
    Call(NeedCall),
    WinState(GameResult, FinalState),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {}
}
