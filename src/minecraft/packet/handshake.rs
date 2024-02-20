use crate::minecraft::{varint::VarInt, varstring::VarString};

use super::{Packet, PacketEncode};

pub struct Handshake {
    protocol_version: VarInt,
    server_address: VarString,
    server_port: u16,
    next_state: VarInt,
}

impl Packet for Handshake {
    const PACKET_ID: VarInt = VarInt(0x00);
}

impl Handshake {
    #[allow(dead_code)]
    pub const VERSION_UNSPECIFIED: VarInt = VarInt(-1);
    #[allow(dead_code)]
    pub const NEXT_STATE_STATUS: VarInt = VarInt(1);

    pub fn new(version: VarInt, server: String, port: u16, next: VarInt) -> Self {
        Self {
            protocol_version: version,
            server_address: VarString(server),
            server_port: port,
            next_state: next,
        }
    }
}

impl PacketEncode for Handshake {}

impl Into<Vec<u8>> for Handshake {
    fn into(self) -> Vec<u8> {
        let mut data = Vec::new();

        // PACKET_ID
        data.extend(Self::PACKET_ID.to_bytes());

        // PROTOCOL_VERSION
        data.extend(self.protocol_version.to_bytes());

        // SERVER_ADDRESS
        data.extend(self.server_address.to_bytes());

        // SERVER_PORT
        data.extend(self.server_port.to_be_bytes());

        // NEXT_STATE
        data.extend(self.next_state.to_bytes());

        // PACKET_SIZE at the begging
        let size = VarInt(data.len() as i32);

        data.splice(0..0, size.to_bytes().drain(..));

        data
    }
}
