use crate::game::{
    event::{NeedCalls, NeedChankan, NeedDiscard, ProcessRound, RoundEvent},
    hands::MahjongHand,
    tiles::{MahjongTile, Wind},
    wall::MahjongWall,
};

mod event;
mod four_player_mahjong;
#[allow(unused)]
mod hands;
#[allow(unused)]
mod tiles;
#[allow(unused)]
mod wall;

enum GameResult {
    Ron { winning_hand: MahjongHand },
    Tsumo(MahjongHand, Wind),
    RinshanKaihou,
}

struct PlayerId(u32);

struct Player {
    id: PlayerId,
    pub score: i32,
}

fn do_round(mut round: impl ProcessRound) {
    let mut round_tick = round.tick();

    let end_of_round;
    loop {
        let thingy = match round_tick {
            RoundEvent::Draw(draw) => draw.discard(),
            RoundEvent::Kan(kan) => kan.ron(),
            RoundEvent::Call(call) => call.discard(),
            RoundEvent::EndState(win_info) => {
                end_of_round = win_info;
                break;
            }
        };

        round_tick = thingy.tick();
    }
}
