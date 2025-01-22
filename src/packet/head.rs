use rkyv::{Archive, Serialize};

use super::types::{PacketType, SerialNumber};

#[repr(C)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PacketHead {
    /// See `PacketBody`.
    pub packet_type: PacketType,
    /// Used for packet ordering in unreliable protocols.
    pub serial_number: Option<SerialNumber>,
}
