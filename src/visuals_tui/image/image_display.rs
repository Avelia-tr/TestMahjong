use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use crate::visuals_tui::{
    image::image_type::ImageType,
    message::{error::*, message_enum::*, send},
};

// just to guarantee not repeating id in a feasable time
static ALLOCATOR_ID: AtomicU32 = AtomicU32::new(1);

pub struct Image {
    id: ImageId,
    placement_id: ImagePlacementId,
    used_placement_id: Arc<AtomicU32>,
}

impl Image {
    pub fn new(image: ImageType) -> Result<Image, LoadError> {
        let id = Self::load(image)?;

        Ok(Self {
            id,
            placement_id: ImagePlacementId::default(),
            used_placement_id: AtomicU32::new(1).into(),
        })
    }

    pub fn display(&self, display_param: ImageDisplayParam) -> Result<(), MessageError> {
        send::send_message(Message(
            Action::Put(self.id, self.placement_id.into(), display_param),
            None,
        ))
    }

    pub fn display_at(&self, display_param: ImageDisplayParam) -> Result<(), MessageError> {
        send::send_message(Message(
            Action::Put(self.id, self.placement_id.into(), display_param),
            None,
        ))
    }

    pub fn load(image: ImageType) -> Result<ImageId, LoadError> {
        if !image.verify_integrity()? {
            return Err(LoadError::FileNotFound);
        }
        let raw_id = ALLOCATOR_ID.fetch_add(1, Ordering::SeqCst);
        // we should crash if a non valid id has been reached
        let id = ImageId::new(raw_id).expect("valid allocation");

        send::send_message_with_payload(
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

impl Clone for Image {
    fn clone(&self) -> Self {
        self.used_placement_id.fetch_add(1, Ordering::SeqCst);
        let new_placement_id =
            match ImagePlacementId::new(self.used_placement_id.load(Ordering::Acquire)) {
                Some(x) => x,
                None => {
                    self.used_placement_id.fetch_add(1, Ordering::SeqCst);
                    ImagePlacementId::new(self.used_placement_id.load(Ordering::Acquire))
                        .expect("valid id since we just passed through 0")
                }
            };

        Self {
            id: self.id,
            placement_id: new_placement_id,
            used_placement_id: self.used_placement_id.clone(),
        }
    }
}
