use std::{
    env::current_dir,
    io::{self, Read},
    path::PathBuf,
};

use crate::visuals_tui::{
    display_screen::DisplayScreen,
    error::{LoadError, MessageError},
    image_display::Image,
    image_display_message::{CursorMovementMode, ImageDisplayParam},
    image_type::ImageType,
};

const RESSOURCE: &str = "Ressources";
const CAT_PATH: &str = "pngtree-pink-cute-cat-icon-animal-png-yuri-png-image_5230763.png";

// shows 2 images and quit on input
pub fn example_1() -> Result<(), LoadError> {
    let screen = DisplayScreen::enable()?;

    let cat_path = PathBuf::from_iter([current_dir().unwrap(), RESSOURCE.into(), CAT_PATH.into()]);

    let image = Image::new(ImageType::PNGPath(cat_path))?;
    let image2 = Image::new(ImageType::PNGData(get_image_data()))?;

    image.display_custom(ImageDisplayParam {
        cursor_movement_mode: Some(CursorMovementMode::StaticAfterImage),
        ..Default::default()
    })?;

    image2.display_custom(ImageDisplayParam {
        cursor_movement_mode: Some(CursorMovementMode::StaticAfterImage),
        z_index: Some(1),
        ..Default::default()
    })?;

    io::stdin().read_exact(&mut [1])?;

    drop(screen);
    Ok(())
}

fn get_image_data() -> Vec<u8> {
    std::fs::read("Ressources/google.png").expect("meow")
}
