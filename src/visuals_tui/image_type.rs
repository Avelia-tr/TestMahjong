use std::{fs, io};

use crate::visuals_tui::image_display_message::{ImageFormat, ImageTransmitMedium};

pub enum ImageType {
    PNGPath(String),
    PNGData(Vec<u8>),
}

impl ImageType {
    pub fn verify_integrity(&self) -> io::Result<bool> {
        match self {
            ImageType::PNGPath(path) => fs::exists(path),
            ImageType::PNGData(_) => Ok(true),
        }
    }

    pub fn get_type(&self) -> ImageFormat {
        match self {
            ImageType::PNGPath(_) => ImageFormat::Png,
            ImageType::PNGData(_) => ImageFormat::Png,
        }
    }

    pub fn get_medium(&self) -> ImageTransmitMedium {
        match self {
            ImageType::PNGPath(_) => ImageTransmitMedium::File,
            ImageType::PNGData(_) => ImageTransmitMedium::Direct,
        }
    }

    pub fn get_payload(&self) -> Vec<u8> {
        match self {
            ImageType::PNGPath(x) => x.clone().into_bytes(),
            ImageType::PNGData(x) => x.clone(),
        }
    }
}
