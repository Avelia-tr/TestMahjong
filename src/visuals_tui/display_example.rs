use std::{
    env::current_dir,
    io::{self, Read},
    path::PathBuf,
    thread,
    time::Duration,
};

use crate::visuals_tui::{
    display_screen::DisplayScreen, image::image_display::Image, image::image_type::ImageType,
    message::error::LoadError, message::message_enum::*,
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
