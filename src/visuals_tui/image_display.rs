use std::{
    io::{self, Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use base64::prelude::*;

use crate::visuals_tui::{
    error::MessageError,
    image_display_message::{
        Action, EncodeMessage, ImageDisplayParam, ImageId, ImagePlacementId, Message, SupressLevel,
        TransmitParam,
    },
    image_type::ImageType,
    utils::Rawmodder,
};

// just to guarantee not repeating id in a feasable time
static ALLOCATOR_ID: AtomicU32 = AtomicU32::new(8);

pub struct Image {
    id: ImageId,
    placement_id: ImagePlacementId,
    used_placement_id: Arc<AtomicU32>,
}

impl Image {
    pub fn new(image: ImageType) -> Result<Image, MessageError> {
        let id = Self::load(image)?;

        Ok(Self {
            id,
            placement_id: ImagePlacementId::default(),
            used_placement_id: AtomicU32::new(1).into(),
        })
    }

    pub fn display_simple(&self) -> io::Result<()> {
        send_message(
            Message(
                Action::Put(
                    self.id,
                    Some(self.placement_id),
                    ImageDisplayParam::default(),
                ),
                Some(SupressLevel::Everything),
            ),
            vec![],
        )?;

        Ok(())
    }

    pub fn display_custom(&self, display_param: ImageDisplayParam) -> io::Result<()> {
        send_message(
            Message(
                Action::Put(self.id, self.placement_id.into(), display_param),
                Some(SupressLevel::Everything),
            ),
            vec![],
        )?;

        Ok(())
    }

    pub fn clone_display(&self, display_param: ImageDisplayParam) -> io::Result<Self> {
        self.used_placement_id.fetch_add(1, Ordering::SeqCst);
        send_message(
            Message(
                Action::Put(
                    self.id,
                    ImagePlacementId::new(self.used_placement_id.load(Ordering::Acquire)),
                    display_param,
                ),
                Some(SupressLevel::Everything),
            ),
            vec![],
        )?;

        Ok(Self {
            id: self.id,
            placement_id: self.placement_id,
            used_placement_id: self.used_placement_id.clone(),
        })
    }

    pub fn load(image: ImageType) -> Result<ImageId, MessageError> {
        if !image.verify_integrity()? {
            return Err(MessageError::NotFound);
        }
        let raw_id = ALLOCATOR_ID.fetch_add(1, Ordering::SeqCst);
        // we should crash if a non valid id has been reached
        let id = ImageId::new(raw_id).expect("valid allocation");

        send_message(
            Message(
                Action::Transmit(TransmitParam {
                    format: image.get_type().into(),
                    medium: image.get_medium().into(),
                    image_id: id.into(),
                    ..Default::default()
                }),
                Some(SupressLevel::Everything),
            ),
            image.get_payload(),
        )?;

        Ok(id)
    }
}

const PREFIX: &[u8] = b"\x1B_G";
const SEPARATOR: &[u8] = b";";
const SUFFIX: &[u8] = b"\x1B\\";

pub fn send_message(header: Message, payload: Vec<u8>) -> io::Result<()> {
    let guard = Rawmodder::enable()?;

    let mut out = stdout().lock();

    out.write_all(&create_message(header, payload))?;
    out.flush()?;

    // TODO: find a way to handle answer from out

    drop(guard);

    Ok(())
}

fn create_message(header: Message, payload: Vec<u8>) -> Vec<u8> {
    let mut v = vec![];
    v.extend_from_slice(PREFIX);
    v.extend_from_slice(&header.encode());
    if !payload.is_empty() {
        v.extend_from_slice(SEPARATOR);
        v.extend_from_slice(BASE64_STANDARD.encode(payload).as_bytes());
    }
    v.extend_from_slice(SUFFIX);
    v
}
