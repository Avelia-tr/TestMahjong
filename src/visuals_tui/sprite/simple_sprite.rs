use std::io;

use crate::visuals_tui::{
    display_screen::OutUtils,
    image::{Image, image_type::ImageType},
    message,
    sprite::{Sprite, vector2::Vector2u},
    utils::Rawmodder,
};

pub struct SimpleSprite {
    image: Image,
    pub position: Vector2u,
    pub z_index: i32,
}

impl SimpleSprite {
    pub fn new(image_type: ImageType) -> Result<Self, message::LoadError> {
        // note to self : cannot use the spread operator ...Default::default() because we would need
        // a default image and that doesn't mean anything
        // using Option<Image> would weaken the invariant that "all SimpleSprite can be displayed"
        Image::new(image_type).map(|image| Self {
            image,
            position: Default::default(),
            z_index: Default::default(),
        })
    }

    pub fn with_position(self, position: Vector2u) -> Self {
        Self { position, ..self }
    }
}

impl Sprite for SimpleSprite {
    fn display(&self) -> Result<(), message::MessageError> {
        let guard = Rawmodder::enable()?;
        io::stdout().move_cursor_to(self.position.x, self.position.y)?;
        self.image.display(message::ImageDisplayParam {
            z_index: self.z_index.into(),
            ..Default::default()
        })?;

        drop(guard);

        Ok(())
    }

    fn update(&mut self) {
        //noop
    }
}
