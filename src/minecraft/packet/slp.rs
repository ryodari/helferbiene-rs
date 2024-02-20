use std::io::{self, Cursor};

use crate::minecraft::{text_component::TextComponent, varint::VarInt, varstring::VarString};

use super::{Packet, PacketDecode, PacketEncode};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub struct SlpRequest;

impl Packet for SlpRequest {
    const PACKET_ID: VarInt = VarInt(0x00);
}

impl PacketEncode for SlpRequest {}

impl Into<Vec<u8>> for SlpRequest {
    fn into(self) -> Vec<u8> {
        let mut data = Vec::new();

        // PACKET_ID
        data.extend(Self::PACKET_ID.to_bytes());

        // PACKET_SIZE at the begging
        let size = VarInt(data.len() as i32);

        data.splice(0..0, size.to_bytes().drain(..));

        data
    }
}

///////

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpServerPlayerSample {
    pub name: String,
    pub id: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpServerPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<SlpServerPlayerSample>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum SlpServerDescription {
    Simple(String),
    Complex(TextComponent),
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpForgeModEntry {
    pub modid: String,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpForgeModInfo {
    // present on Forge servers
    #[serde(rename = "type")]
    pub type_: String,
    pub mod_list: Vec<SlpForgeModEntry>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlpResponse {
    pub version: SlpServerVersion,
    pub players: SlpServerPlayers,
    pub description: SlpServerDescription,
    pub favicon: Option<String>,
    pub enforces_secure_chat: Option<bool>,
    pub previews_chat: Option<bool>,
    pub prevents_chat_reports: Option<bool>,
    pub modinfo: Option<SlpForgeModInfo>,
}

impl Packet for SlpResponse {
    const PACKET_ID: VarInt = VarInt(0x00);
}

impl PacketDecode for SlpResponse {}

impl TryFrom<Vec<u8>> for SlpResponse {
    type Error = io::Error;

    fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
        {
            // remove packet size from value
            let cursor = Cursor::new(&value);
            let packet_size = VarInt::from_bytes(cursor)?;

            log::debug!("packet size: {}", packet_size.0);

            let packet_size_var_int_size = packet_size.to_bytes().len();
            value.drain(0..packet_size_var_int_size);
        }

        {
            // remove packet id from value
            let cursor = Cursor::new(&value);
            let packet_id = VarInt::from_bytes(cursor)?;

            log::debug!("packet id: {}", packet_id.0);

            let packet_id_var_int_size = packet_id.to_bytes().len();
            value.drain(0..packet_id_var_int_size);
        }

        let json_str = VarString::from_bytes(value)?;

        log::debug!("slp response: {}", json_str.0);

        Ok(serde_json::from_str(&json_str.0)?)
    }
}
