pub mod body;
pub mod constants;
pub mod error;
pub mod head;
pub mod resolver;
pub mod types;
pub mod utils;

use body::PacketBody;
use constants::{PACKET_PREFIX_LENGTH, PROTOCOL_VERSION};
use error::PacketError;
use head::PacketHead;
use ps_buffer::Buffer;
use rkyv::{
    api::high::HighSerializer,
    bytecheck::CheckBytes,
    rancor::{Error, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    validation::{archive::ArchiveValidator, shared::SharedValidator, Validator},
    Archive, Archived, Serialize,
};
use utils::checksum;

#[repr(C)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Packet<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    pub head: PacketHead,
    pub body: T,
}

impl<T> Packet<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    pub fn serialize(&self) -> Result<Buffer, PacketError> {
        let serialized = rkyv::to_bytes::<Error>(self)?;
        let length = serialized.len() + PACKET_PREFIX_LENGTH;

        let mut buffer = Buffer::with_capacity(length)?;

        buffer.extend_from_slice(&(length as u32).to_le_bytes())?;
        buffer.extend_from_slice(&PROTOCOL_VERSION.to_le_bytes())?;
        buffer.extend_from_slice(&checksum(&serialized).to_le_bytes())?;
        buffer.extend_from_slice(&serialized)?;

        Ok(buffer)
    }

    /// Allows you to access the contents of a serialized `Packet`.
    pub fn access(data: &[u8]) -> Result<&Archived<Self>, PacketError> {
        let length = u32::from_le_bytes(data[0..4].try_into()?);
        let proto_ver = u32::from_le_bytes(data[4..8].try_into()?);
        let read_checksum = u64::from_le_bytes(data[8..16].try_into()?);

        if proto_ver > PROTOCOL_VERSION {
            Err(PacketError::UnsupportedProtocolVersion(proto_ver))?
        }

        let real_length = data.len() as u32;

        if real_length != length {
            Err(PacketError::IncorrectLengthPrefix(length, real_length))?
        }

        let data = &data[16..];
        let real_checksum = checksum(data);

        if real_checksum != read_checksum {
            Err(PacketError::IncorrectChecksum(read_checksum, real_checksum))?
        }

        let packet = rkyv::access::<Archived<Self>, Error>(data)?;

        Ok(packet)
    }
}

impl<T> From<T> for Packet<T>
where
    T: PacketBody,
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    fn from(value: T) -> Self {
        Self {
            head: PacketHead {
                packet_type: T::packet_type(),
                serial_number: None,
            },
            body: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ps_str::Utf8Encoder;
    use rkyv::{rancor::Error, Archive, Archived, Serialize};

    use crate::packet::body::PacketBody;

    use super::{error::PacketError, Packet};

    #[derive(Archive, Clone, Serialize)]
    struct Message {
        pub foo: u64,
        pub bar: String,
    }

    impl PacketBody for Message {
        fn packet_type() -> super::types::PacketType {
            120
        }
    }

    #[test]
    fn simple() -> Result<(), PacketError> {
        let message = Message {
            foo: 0x1122334455667788u64,
            bar: "Hello, world!".to_utf8_string(),
        };

        let packet = Packet::from(message.clone());

        let serialized = packet.serialize()?;
        let accessed = Packet::<(u32, u32)>::access(&serialized)?;

        let rkyv_serialized = rkyv::to_bytes::<Error>(&packet)?;
        let rkyv_accessed = rkyv::access::<Archived<(u32, u32)>, Error>(&rkyv_serialized)?;

        assert_eq!(&accessed.body, rkyv_accessed, "Should be identical");

        Ok(())
    }
}
