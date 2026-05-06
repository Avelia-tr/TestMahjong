use crate::visuals_tui::message::MessageError;

mod simple_sprite;
mod vector2;

trait Sprite {
    fn display() -> Result<(), MessageError>;
    fn update();
}
