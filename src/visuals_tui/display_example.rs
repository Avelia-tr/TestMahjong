use std::io::{self, Read, stdin};

use crate::visuals_tui::{
    display_screen::DisplayScreen,
    image::{image_display::Image, image_type::ImageType},
    message::{error::LoadError, message_enum::*},
    sprite::{Sprite, simple_sprite::SimpleSprite, vector2::Vector2u},
    utils::Rawmodder,
};

const RESSOURCE: &str = "Ressources";
const CAT_PATH: &str = "pngtree-pink-cute-cat-icon-animal-png-yuri-png-image_5230763.png";
const GOOGLE_PATH: &str = "google.png";

// shows 2 images and quit on input
pub fn example_1() -> Result<(), LoadError> {
    let screen = DisplayScreen::enable()?;

    //let cat_path = PathBuf::from_iter([current_dir().unwrap(), RESSOURCE.into(), CAT_PATH.into()]);

    let image = Image::new(ImageType::new_png_load(CAT_PATH)?)?;
    let image2 = Image::new(ImageType::new_png_load(GOOGLE_PATH)?)?;

    image.display(ImageDisplayParam {
        cursor_movement_mode: Some(CursorMovementMode::StaticAfterImage),
        ..Default::default()
    })?;

    image2.display(ImageDisplayParam {
        cursor_movement_mode: Some(CursorMovementMode::StaticAfterImage),
        z_index: Some(1),
        ..Default::default()
    })?;

    io::stdin().read_exact(&mut [1])?;

    drop(screen);
    Ok(())
}

pub fn example_moving_image() -> Result<(), LoadError> {
    let guard = Rawmodder::enable();
    let screen = DisplayScreen::enable()?;
    let mut image = SimpleSprite::new(ImageType::new_png_load(CAT_PATH)?)?;
    let mut input_buffer: [u8; _] = [0];

    loop {
        image.display()?;
        stdin().lock().read_exact(&mut input_buffer)?;
        match input_buffer[0] {
            b'w' => image.position -= Vector2u::new(0, 1),
            b's' => image.position += Vector2u::new(0, 1),
            b'a' => image.position -= Vector2u::new(1, 0),
            b'd' => image.position += Vector2u::new(1, 0),
            b'q' => break,
            _ => (),
        }
    }

    drop(screen);
    drop(guard);
    Ok(())
}
