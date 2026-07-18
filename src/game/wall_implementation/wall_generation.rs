use std::sync::Arc;

use crate::game::{
    rand::DeterministicRng,
    tiles::{DragonTiles, MahjongTile, MahjongTilesIdentity, NumberTile, Wind},
};

pub fn classic_wall_tiles() -> Vec<MahjongTile> {
    use MahjongTilesIdentity as MTI;
    let mut res = vec![];

    for _ in 0..4 {
        res.extend(creates_sets_tile(MTI::Dragon(DragonTiles::Red)));
        res.extend(creates_sets_tile(MTI::Wind(Wind::East)));
        res.extend(creates_sets_tile(MTI::Man(NumberTile::N1)));
        res.extend(creates_sets_tile(MTI::Pin(NumberTile::N1)));
        res.extend(creates_sets_tile(MTI::Sou(NumberTile::N1)));
    }

    res
}

pub fn creates_sets_tile(tile_type: MahjongTilesIdentity) -> Vec<MahjongTile> {
    let mut res: Vec<_> = vec![MahjongTile::new(tile_type)];
    let mut meow = tile_type.get_dora();
    while meow != tile_type {
        res.push(MahjongTile::new(meow));
        meow = meow.get_dora();
    }
    res
}

pub trait ProtoWall: IntoIterator<Item = MahjongTile> {
    fn make_tile_red(&mut self, kind: MahjongTilesIdentity, amount: usize) -> &mut Self;
    fn shuffle(self, rand: &mut impl DeterministicRng) -> Self;
}

impl ProtoWall for Vec<MahjongTile> {
    fn make_tile_red(&mut self, kind: MahjongTilesIdentity, amount: usize) -> &mut Self {
        self.iter_mut()
            .filter(|x| x.identity == kind)
            .take(amount)
            .for_each(|x| {
                x.red = true;
            });

        self
    }

    // pass a random generator ?
    fn shuffle(mut self, rand: &mut impl DeterministicRng) -> Self {
        const SHUFFLE_DEPTH: usize = 300;
        let self_len: u64 = self.len().try_into().expect("should work ?");
        for i in rand
            .map(|x| {
                (x % self_len)
                    .try_into()
                    .expect("no panic cuz we moduloed by a valid usize")
            })
            .take(SHUFFLE_DEPTH)
        {
            self.swap(0, i);
        }

        self
    }
}

pub fn classic_3p_wall() -> Vec<MahjongTile> {
    classic_wall_tiles()
        .iter()
        .filter(|&&x| {
            !(matches!(x.identity, MahjongTilesIdentity::Man(_)) && x.identity.is_terminal())
        })
        .copied()
        .collect()
}

pub fn consum_n<T: Clone>(iterthingy: &mut impl Iterator<Item = T>, n: usize) -> Arc<[T]> {
    let mut res = vec![];
    for _ in 0..n {
        if let Some(x) = iterthingy.next() {
            res.push(x);
        }
    }
    res.into()
}
