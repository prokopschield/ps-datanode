pub mod body;
pub mod constants;
pub mod error;
pub mod head;
pub mod utils;

use body::PacketBody;
use constants::{PACKET_PREFIX_LENGTH, PROTOCOL_VERSION};
use error::PacketError;
use head::PacketHead;
use ps_buffer::Buffer;
use rkyv::{rancor::Error, Archive, Serialize};
use utils::checksum;

#[repr(C)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Packet {
    head: PacketHead,
    body: PacketBody,
}

impl Packet {
    pub fn serialize(&self) -> Result<Buffer, PacketError> {
        let serialized = rkyv::to_bytes::<Error>(self)?;
        let length = serialized.len() + PACKET_PREFIX_LENGTH;

        let mut buffer = Buffer::alloc(length);

        buffer[0..4].copy_from_slice(&(length as u32).to_le_bytes());
        buffer[4..8].copy_from_slice(&PROTOCOL_VERSION.to_le_bytes());
        buffer[8..16].copy_from_slice(&checksum(&serialized).to_le_bytes());
        buffer[16..].copy_from_slice(&serialized);

        Ok(buffer)
    }
}
