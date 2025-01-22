use rkyv::{
    bytecheck::CheckBytes,
    rancor::Strategy,
    ser::{allocator::ArenaHandle, sharing::Share, Serializer},
    util::AlignedVec,
    validation::{archive::ArchiveValidator, shared::SharedValidator, Validator},
    Archive, Archived, Serialize,
};

use crate::traits::connection::ConnectionBox;

use super::{body::PacketBody, error::PacketError, Packet};

pub trait PacketResolver
where
    for<'a> <Self as PacketResolver>::Target:
        Serialize<Strategy<Serializer<AlignedVec, ArenaHandle<'a>, Share>, rkyv::rancor::Error>>,
    for<'a> <<Self as PacketResolver>::Target as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    type Target: PacketBody;

    fn resolve(
        packet: &Archived<Packet<Self::Target>>,
        connection: ConnectionBox,
    ) -> Result<(), PacketError>;

    fn resolve_bytes(bytes: &[u8], connection: ConnectionBox) -> Result<(), PacketError> {
        Self::resolve(Packet::<Self::Target>::access(bytes)?, connection)
    }
}
