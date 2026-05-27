use crate::visuals_tui::{
    image::Image,
    message,
    sprite::{
        Sprite,
        vector2::{SizeVector2, Vector2i},
    },
};

struct RelativeSprite {
    image: Image,
    childs: Vec<RelativeSprite>,
    position: Vector2i,
    size: SizeVector2,
}

impl Sprite for RelativeSprite {
    fn display(&self) -> Result<(), message::MessageError> {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}

impl RelativeSprite {
    fn display_child(&self, image: &Image) -> Result<(), message::MessageError> {
        // display childs recursively
        todo!()
    }
}
