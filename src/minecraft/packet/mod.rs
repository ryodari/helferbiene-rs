pub mod handshake;
pub mod slp;

use crate::minecraft::varint::VarInt;

/*
Field Name 	Field Type 	Notes
Length 	VarInt 	Length of Packet ID + Data
Packet ID 	VarInt
Data 	Byte Array 	Depends on the connection state and packet ID, see the sections below
*/
pub trait Packet {
    const PACKET_ID: VarInt;
}

pub trait PacketEncode: Packet + Into<Vec<u8>> {}

pub trait PacketDecode: Packet + TryFrom<Vec<u8>> {}
