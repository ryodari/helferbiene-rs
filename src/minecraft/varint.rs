use std::io::{self, Read};

pub struct VarInt(pub i32);

impl VarInt {
    const SEGMENT_BITS: u8 = 0b0111_1111;
    const CONTINUE_BIT: u8 = 0b1000_0000;

    pub fn from_bytes<R: Read>(reader: R) -> io::Result<Self> {
        let mut value: i32 = 0;

        let mut position = 0;

        let mut bytes = reader.bytes();
        loop {
            let current_byte = match bytes.next() {
                Some(x) => x?,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid input buffer",
                    ))
                }
            };

            value |= ((current_byte & Self::SEGMENT_BITS) as i32) << position;

            if current_byte & Self::CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }
        }

        Ok(Self(value))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut value = self.0;

        loop {
            let mut byte = (value & Self::SEGMENT_BITS as i32) as u8;
            value >>= 7;
            if value != 0 {
                byte |= Self::CONTINUE_BIT;
            }
            result.push(byte);

            if value == 0 {
                break;
            }
        }

        result
    }
}
