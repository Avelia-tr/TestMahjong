use crate::game::tiles::MahjongTile;

pub trait MahjongWall {
    fn draw(&mut self) -> Option<MahjongTile>;
    fn draw_dead_wall(&mut self) -> Option<MahjongTile>;
    fn reveal_dora(&mut self) -> Option<MahjongTile>;

    fn peek_dora(&self, index: usize) -> Option<MahjongTile>;
    fn peek_ura_dora(&self, index: usize) -> Option<MahjongTile>;
    fn get_visible_doras(&self) -> &[MahjongTile];
    fn get_visible_ura_doras(&self) -> &[MahjongTile];

    fn poll_tile_remaining(&self) -> usize;
}

#[rustfmt::skip]
pub mod impos {

    use super::{MahjongWall, MahjongTile};

    pub enum ImpossibleWall {}

    impl MahjongWall for ImpossibleWall {
        fn poll_tile_remaining(&self) -> usize { todo!() }

        fn draw(&mut self) -> Option<MahjongTile> { todo!() }

        fn get_visible_doras(&self) -> &[MahjongTile] { todo!() }

        fn reveal_dora(&mut self) -> Option<MahjongTile> { todo!() }

        fn get_visible_ura_doras(&self) -> &[MahjongTile] { todo!() }

        fn draw_dead_wall(&mut self) -> Option<MahjongTile> { todo!() }

        fn peek_dora(&self, index: usize) -> Option<MahjongTile> { todo!() }

        fn peek_ura_dora(&self, index: usize) -> Option<MahjongTile> { todo!() }

    }

    #[allow(dead_code)]
    pub fn mock_wall() -> ImpossibleWall {
        unreachable!("[Error] this is for type testing only, ImpossibleWall is not constructible do not run this code")
    }
}
