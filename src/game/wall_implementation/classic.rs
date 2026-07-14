use std::sync::Arc;

use crate::game::{
    tiles::MahjongTile,
    wall::MahjongWall,
    wall_implementation::wall_generation::{ProtoWall, consum_n},
};

#[derive(Debug)]
pub struct ClassicWall {
    wall: WallPart,
    dead_wall: WallPart,
    doras: WallPart,
    ura_doras: Arc<[MahjongTile]>,
}

#[derive(Debug)]
struct WallPart {
    wall: Arc<[MahjongTile]>,
    cursor: usize,
}

impl WallPart {
    fn draw(&mut self) -> Option<MahjongTile> {
        self.wall.get(self.cursor + 1).map(|&x| {
            self.cursor += 1;
            x
        })
    }

    fn peek(&self, idx: usize) -> Option<MahjongTile> {
        self.wall.get(idx).copied()
    }

    pub fn len(&self) -> usize {
        self.wall.len()
    }
}

impl MahjongWall for ClassicWall {
    fn draw(&mut self) -> Option<MahjongTile> {
        self.wall.draw()
    }

    fn draw_dead_wall(&mut self) -> Option<MahjongTile> {
        self.dead_wall.draw()
    }

    fn reveal_dora(&mut self) -> Option<MahjongTile> {
        self.doras.draw()
    }

    fn peek_dora(&self, index: usize) -> Option<MahjongTile> {
        self.doras.peek(index)
    }

    fn peek_ura_dora(&self, index: usize) -> Option<MahjongTile> {
        self.ura_doras.get(self.doras.cursor).copied()
    }

    fn get_visible_doras(&self) -> &[MahjongTile] {
        &self.doras.wall[..self.doras.cursor]
    }

    fn get_visible_ura_doras(&self) -> &[MahjongTile] {
        &self.ura_doras[..self.doras.cursor]
    }

    fn poll_tile_remaining(&self) -> usize {
        self.wall.len() - self.wall.cursor
    }
}

impl ClassicWall {
    pub fn new_non_suffling(proto: impl ProtoWall) -> Self {
        let mut proto_new = proto.into_iter();

        Self {
            dead_wall: WallPart {
                wall: consum_n(&mut proto_new, 4),
                cursor: 0,
            },
            doras: WallPart {
                wall: consum_n(&mut proto_new, 5),
                cursor: 0,
            },
            ura_doras: consum_n(&mut proto_new, 5),
            wall: WallPart {
                wall: proto_new.collect::<Vec<_>>().into(),
                cursor: 0,
            },
        }
    }
}
