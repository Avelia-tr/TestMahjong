use std::io::{self, Write};

use base64::prelude::*;

use crate::visuals_tui::{
    message::{
        EncodeMessage, Message, MessageError, SupressLevel, error::ToMessageError,
        error_parsing::parse_error_kitty,
    },
    utils::Rawmodder,
};

const PREFIX: &[u8] = b"\x1B_G";
const SEPARATOR: &[u8] = b";";
const SUFFIX: &[u8] = b"\x1B\\";

pub fn send_message_with_payload(header: Message, payload: Vec<u8>) -> Result<(), MessageError> {
    let guard = Rawmodder::enable().to_message_error(&header)?;

    let mut out = io::stdout().lock();

    out.write_all(&create_message_with_payload(&header, payload))
        .to_message_error(&header)?;
    out.flush().to_message_error(&header)?;

    let is_error_suppressed = matches!(
        header.1,
        Some(SupressLevel::SuppressSuccess | SupressLevel::Everything)
    );

    if !is_error_suppressed {
        let _ = parse_error_kitty(&guard)
            .to_message_error(&header)?
            .to_message_error(&header)?;
    }

    drop(guard);

    Ok(())
}

pub fn send_message(message: Message) -> Result<(), MessageError> {
    let guard = Rawmodder::enable().to_message_error(&message)?;

    let mut out = io::stdout().lock();

    out.write_all(&create_message(&message))
        .to_message_error(&message)?;
    out.flush().to_message_error(&message)?;

    let is_error_suppressed = match message.1 {
        Some(SupressLevel::Everything | SupressLevel::SuppressSuccess) => true,
        _ => false,
    };

    if !is_error_suppressed {
        let _ = parse_error_kitty(&guard)
            .to_message_error(&message)?
            .to_message_error(&message)?;
    }

    drop(guard);

    Ok(())
}

fn create_message(message: &Message) -> Vec<u8> {
    let mut v = vec![];
    v.extend_from_slice(PREFIX);
    v.extend_from_slice(&message.encode());
    v.extend_from_slice(SUFFIX);
    v
}

fn create_message_with_payload(header: &Message, payload: Vec<u8>) -> Vec<u8> {
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
