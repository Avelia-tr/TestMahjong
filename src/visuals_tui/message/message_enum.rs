use std::num::NonZeroU32;

pub struct Message(pub Action, pub Option<SupressLevel>);

#[derive(Debug)]
pub enum Action {
    Transmit(TransmitParam),
    TransmitAndDisplay(TransmitParam, ImageDisplayParam),
    Put(ImageId, Option<ImagePlacementId>, ImageDisplayParam),
    Delete(DeletionMessage),
    AnimatedTransmit,
    AnimationControl,
    AnimationCompose,
    Query,
}

#[derive(Debug)]
pub enum SupressLevel {
    None,
    SuppressSuccess,
    Everything,
}

#[derive(Default, Debug)]
pub struct TransmitParam {
    pub format: Option<ImageFormat>,
    pub medium: Option<ImageTransmitMedium>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub size: Option<usize>,
    pub offset: Option<usize>,
    pub image_id: Option<ImageId>,
    pub image_number: Option<ImageNumber>,
    pub placement_id: Option<ImagePlacementId>,
    pub compression_style: Option<DataCompression>,
    pub last_chunk: Option<bool>,
}

#[derive(Debug)]
pub enum ImageFormat {
    Rgb24 = 24,
    Rgb32 = 32,
    Png = 100,
}

#[derive(Debug)]
pub enum ImageTransmitMedium {
    Direct,
    File,
    Temp,
    SharedObject,
}

#[derive(Copy, Clone, Debug)]
pub struct ImageId(NonZeroU32);
#[derive(Copy, Clone, Debug)]
pub struct ImageNumber(NonZeroU32);
#[derive(Copy, Clone, Debug)]
pub struct ImagePlacementId(NonZeroU32);

macro_rules! impl_default_wrapper {
    ($t:ty) => {
        impl Default for $t {
            fn default() -> Self {
                Self(unsafe { NonZeroU32::new_unchecked(1) })
            }
        }
    };
}

macro_rules! impl_new_nonzero_wrapper {
    ($t:ty) => {
        impl $t {
            pub fn new(x: u32) -> Option<Self> {
                NonZeroU32::new(x).map(Self)
            }
        }
    };
}

impl_new_nonzero_wrapper!(ImageId);
impl_new_nonzero_wrapper!(ImageNumber);
impl_new_nonzero_wrapper!(ImagePlacementId);

impl_default_wrapper!(ImageId);
impl_default_wrapper!(ImageNumber);
impl_default_wrapper!(ImagePlacementId);

#[derive(Debug)]
pub enum DataCompression {
    Zlib,
}

#[derive(Default, Debug)]
pub struct ImageDisplayParam {
    pub offset_width: Option<u32>,
    pub offset_height: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub offset_tile_x: Option<u32>,
    pub offset_tile_y: Option<u32>,
    pub column_span: Option<u32>,
    pub row_span: Option<u32>,
    pub cursor_movement_mode: Option<CursorMovementMode>,
    pub placeholder_unicode: Option<bool>,
    pub z_index: Option<i32>,
    pub relative_placement_id: Option<ImagePlacementId>,
    pub parent_id: Option<ImageId>,
    pub relative_offset_x: Option<i32>,
    pub relative_offset_y: Option<i32>,
}

#[derive(Debug)]
pub enum CursorMovementMode {
    MoveAfterImage = 0,
    StaticAfterImage = 1,
}

#[derive(Debug)]
pub enum DeletionMessage {
    All,
    AllWithImageID(ImageId, Option<ImagePlacementId>),
    Newest(ImageNumber, Option<ImagePlacementId>),
    AtCurrentCursor,
    AnimationFrame,
    AtX(u32),
    AtY(u32),
    AtZ(u32),
    AtXY { x: u32, y: u32 },
    AtXYZ { x: u32, y: u32, z: i32 },
    InRangeID { start: ImageId, end: ImageId },
}

pub trait EncodeMessage {
    fn encode(&self) -> Vec<u8>;
}

impl EncodeMessage for Message {
    fn encode(&self) -> Vec<u8> {
        let mut message = vec![];

        message.extend(self.0.encode());
        message.extend(self.1.encode());

        message
    }
}

impl Action {
    fn get_key(&self) -> u8 {
        match self {
            Action::Transmit(_) => b't',
            Action::TransmitAndDisplay(_, _) => b'T',
            Action::Put(_, _, _) => b'p',
            Action::Delete(_) => b'd',
            Action::AnimatedTransmit => b'f',
            Action::AnimationControl => b'a',
            Action::AnimationCompose => b'c',
            Action::Query => b'q',
        }
    }
}

impl EncodeMessage for Action {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::from(b"a=");
        buffer.push(self.get_key());

        buffer.extend(match self {
            Action::Transmit(transmit_param) => transmit_param.encode(),
            Action::TransmitAndDisplay(transmit_param, image_display_param) => {
                encode_transmit_display(transmit_param, image_display_param)
            }
            Action::Put(image_id, image_placement_id, image_display_param) => {
                encode_put(image_id, image_placement_id, image_display_param)
            }
            Action::Delete(deletion_message) => deletion_message.encode(),
            Action::AnimatedTransmit => todo!(),
            Action::AnimationControl => todo!(),
            Action::AnimationCompose => todo!(),
            Action::Query => todo!(),
        });
        buffer
    }
}

fn encode_transmit_display(
    transmit_param: &TransmitParam,
    image_display_param: &ImageDisplayParam,
) -> Vec<u8> {
    let mut buf = vec![];
    buf.extend(transmit_param.encode());
    buf.extend(image_display_param.encode());
    buf
}

fn encode_put(
    image_id: &ImageId,
    image_placement_id: &Option<ImagePlacementId>,
    image_display_param: &ImageDisplayParam,
) -> Vec<u8> {
    let mut buf = vec![];
    buf.extend(image_id.encode());
    buf.extend(image_placement_id.encode());
    buf.extend(image_display_param.encode());
    buf
}

impl<T: EncodeMessage> EncodeMessage for Option<T> {
    fn encode(&self) -> Vec<u8> {
        match self {
            Some(to_encode) => to_encode.encode(),
            None => vec![],
        }
    }
}

impl EncodeMessage for SupressLevel {
    fn encode(&self) -> Vec<u8> {
        match self {
            SupressLevel::None => b",q=0".to_vec(),
            SupressLevel::SuppressSuccess => b",q=1".to_vec(),
            SupressLevel::Everything => b",q=1".to_vec(),
        }
    }
}

macro_rules! impl_Encode_u8_wrapper {
    ($t:ty,$prefix:expr) => {
        impl EncodeMessage for $t {
            fn encode(&self) -> Vec<u8> {
                let mut buf = Vec::new();
                buf.push(b',');
                buf.extend_from_slice($prefix);
                buf.extend(self.0.to_string().into_bytes());

                buf
            }
        }
    };
}

impl_Encode_u8_wrapper!(ImageId, b"i=");
impl_Encode_u8_wrapper!(ImageNumber, b"I=");
impl_Encode_u8_wrapper!(ImagePlacementId, b"p=");

impl EncodeMessage for ImageFormat {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b",f=");
        buf.extend_from_slice(match self {
            ImageFormat::Rgb24 => b"24",
            ImageFormat::Rgb32 => b"32",
            ImageFormat::Png => b"100",
        });
        buf
    }
}

impl EncodeMessage for ImageTransmitMedium {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b",t=");
        buf.extend_from_slice(match self {
            ImageTransmitMedium::Direct => b"d",
            ImageTransmitMedium::File => b"f",
            ImageTransmitMedium::Temp => b"t",
            ImageTransmitMedium::SharedObject => b"s",
        });
        buf
    }
}

impl EncodeMessage for DataCompression {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b",o=");
        buf.extend_from_slice(match self {
            DataCompression::Zlib => b"z",
        });
        buf
    }
}

impl EncodeMessage for TransmitParam {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.format.encode());
        buffer.extend(self.medium.encode());

        encode_custom(&mut buffer, self.width, b",s=", |x| {
            x.to_string().into_bytes()
        });
        encode_custom(&mut buffer, self.height, b",v=", |x| {
            x.to_string().into_bytes()
        });
        encode_custom(&mut buffer, self.size, b",S=", |x| {
            x.to_string().into_bytes()
        });
        encode_custom(&mut buffer, self.offset, b",O=", |x| {
            x.to_string().into_bytes()
        });

        buffer.extend(self.image_id.encode());
        buffer.extend(self.image_number.encode());
        buffer.extend(self.placement_id.encode());
        buffer.extend(self.compression_style.encode());

        encode_custom(&mut buffer, self.last_chunk, b",O=", |x| {
            if x { Vec::from(b"0") } else { Vec::from(b"1") }
        });

        buffer
    }
}

impl EncodeMessage for CursorMovementMode {
    fn encode(&self) -> Vec<u8> {
        match self {
            CursorMovementMode::MoveAfterImage => b",C=0".into(),
            CursorMovementMode::StaticAfterImage => b",C=1".into(),
        }
    }
}

fn int_to_bytes(x: impl ToString) -> Vec<u8> {
    x.to_string().into_bytes()
}

impl EncodeMessage for ImageDisplayParam {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        encode_custom(&mut buffer, self.offset_width, b",x=", int_to_bytes);
        encode_custom(&mut buffer, self.offset_height, b",y=", int_to_bytes);
        encode_custom(&mut buffer, self.width, b",w=", int_to_bytes);
        encode_custom(&mut buffer, self.height, b",h=", int_to_bytes);
        encode_custom(&mut buffer, self.offset_tile_x, b",X=", int_to_bytes);
        encode_custom(&mut buffer, self.offset_tile_y, b",Y=", int_to_bytes);
        encode_custom(&mut buffer, self.column_span, b",c=", int_to_bytes);
        encode_custom(&mut buffer, self.row_span, b",r=", int_to_bytes);
        encode_custom(&mut buffer, self.placeholder_unicode, b",U=", |x| {
            if x { Vec::from(b"1") } else { Vec::from(b"0") }
        });
        buffer.extend(self.cursor_movement_mode.encode());
        encode_custom(&mut buffer, self.z_index, b",z=", int_to_bytes);

        buffer.extend(self.relative_placement_id.encode());
        buffer.extend(self.parent_id.encode());

        encode_custom(&mut buffer, self.relative_offset_x, b",H=", int_to_bytes);
        encode_custom(&mut buffer, self.relative_offset_y, b",V=", int_to_bytes);

        buffer
    }
}

fn encode_custom<T>(
    buffer: &mut Vec<u8>,
    option: Option<T>,
    prefix: &[u8],
    mapper: impl Fn(T) -> Vec<u8>,
) {
    if let Some(value) = option {
        buffer.extend_from_slice(prefix);
        buffer.extend(mapper(value));
    }
}

impl DeletionMessage {
    const fn get_key(&self) -> u8 {
        match self {
            DeletionMessage::All => b'a',
            DeletionMessage::AllWithImageID(_, _) => b'i',
            DeletionMessage::Newest(_, _) => b'n',
            DeletionMessage::AtCurrentCursor => b'c',
            DeletionMessage::AnimationFrame => b'f',
            DeletionMessage::AtX(_) => b'x',
            DeletionMessage::AtY(_) => b'y',
            DeletionMessage::AtZ(_) => b'z',
            DeletionMessage::AtXY { x: _, y: _ } => b'p',
            DeletionMessage::AtXYZ { x: _, y: _, z: _ } => b'p',
            DeletionMessage::InRangeID { start: _, end: _ } => b'r',
        }
    }
}

impl EncodeMessage for DeletionMessage {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::from(b",d=");
        buffer.push(self.get_key());

        buffer.extend(match self {
            DeletionMessage::All => Vec::<u8>::new(),
            DeletionMessage::AllWithImageID(image_id, image_placement_id) => {
                encode_deletion_all_with_id(image_id, image_placement_id)
            }
            DeletionMessage::Newest(image_number, image_placement_id) => {
                encode_deletion_newest(image_number, image_placement_id)
            }
            DeletionMessage::AtCurrentCursor => Vec::new(),
            DeletionMessage::AnimationFrame => Vec::new(),
            DeletionMessage::AtX(integer) => encode_deletion_coord(integer, b",x="),
            DeletionMessage::AtY(integer) => encode_deletion_coord(integer, b",y="),
            DeletionMessage::AtZ(integer) => encode_deletion_coord(integer, b",y="),
            DeletionMessage::AtXY { x, y } => encode_deletion_xy(x, y),
            DeletionMessage::AtXYZ { x, y, z } => encode_deletion_xyz(x, y, z),
            DeletionMessage::InRangeID { start, end } => encode_deletion_range(start, end),
        });

        buffer
    }
}

fn encode_deletion_all_with_id(
    image_id: &ImageId,
    image_placement_id: &Option<ImagePlacementId>,
) -> Vec<u8> {
    let mut buffer = Vec::new();

    buffer.extend(image_id.encode());
    buffer.extend(image_placement_id.encode());

    buffer
}

fn encode_deletion_newest(
    image_number: &ImageNumber,
    image_placement_id: &Option<ImagePlacementId>,
) -> Vec<u8> {
    let mut buffer = Vec::new();

    buffer.extend(image_number.encode());
    buffer.extend(image_placement_id.encode());

    buffer
}

fn encode_deletion_coord(integer: &u32, prefix: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::from(prefix);
    buffer.extend(integer.to_string().into_bytes());
    buffer
}

fn encode_deletion_xy(x: &u32, y: &u32) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(encode_deletion_coord(x, b",x="));
    buffer.extend(encode_deletion_coord(y, b",y="));
    buffer
}

fn encode_deletion_xyz(x: &u32, y: &u32, z: &i32) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(encode_deletion_coord(x, b",x="));
    buffer.extend(encode_deletion_coord(y, b",y="));
    buffer.extend_from_slice(b",z=");
    buffer.extend(z.to_string().into_bytes());
    buffer
}

fn encode_deletion_range(start: &ImageId, end: &ImageId) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(b",x=");
    buffer.extend(start.0.to_string().into_bytes());
    buffer.extend_from_slice(b",y=");
    buffer.extend(end.0.to_string().into_bytes());
    buffer
}
