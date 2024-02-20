use std::{io::Cursor, str::from_utf8};

use serenity::futures::io;

use crate::minecraft::varint::VarInt;

pub struct VarString(pub String);

impl VarString {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();

        let length = VarInt(self.0.len() as i32);

        data.extend(length.to_bytes());
        data.extend(self.0.as_bytes());

        data
    }

    pub fn from_bytes(mut data: Vec<u8>) -> io::Result<Self> {
        let cursor = Cursor::new(&data);
        let length = VarInt::from_bytes(cursor)?;
        let length_varint_size = length.to_bytes().len();
        data.drain(0..length_varint_size); // consume the string length

        log::debug!("VarString length: {}", length.0);

        let text = match from_utf8(data.as_slice()) {
            Ok(x) => x,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Parsing error: {}", e),
                ))
            }
        };

        Ok(Self(text.to_string()))
    }
}
