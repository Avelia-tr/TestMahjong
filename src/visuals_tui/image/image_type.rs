use std::{fs, io, os::unix::ffi::OsStrExt, path::PathBuf};

use crate::visuals_tui::message::message_enum::{ImageFormat, ImageTransmitMedium};

pub enum ImageType {
    PNGPath(PathBuf),
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
            ImageType::PNGPath(x) => x.as_os_str().as_bytes().into(),
            ImageType::PNGData(x) => x.clone(),
        }
    }

    const BASE_FOLDER: &str = "Ressources/";

    pub fn new_png_load(path: &str) -> io::Result<Self> {
        std::fs::read([Self::BASE_FOLDER, path].concat()).map(ImageType::PNGData)
    }
}
