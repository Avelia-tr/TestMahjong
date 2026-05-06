use crate::visuals_tui::{image::Image, sprite::vector2::Vector2u};

pub struct SimpleSprite {
    image: Image,
    pub position: Vector2u,
}

impl SimpleSprite {
    pub fn new(image: Image, position: Vector2u) -> Self {
        Self { image, position }
    }
}
