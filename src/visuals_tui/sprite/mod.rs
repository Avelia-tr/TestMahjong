use crate::visuals_tui::message::MessageError;

pub mod relative_sprite;
pub mod simple_sprite;
pub mod vector2;

pub trait Sprite {
    fn display(&self) -> Result<(), MessageError>;
    fn update(&mut self);
}
