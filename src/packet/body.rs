use super::types::PacketType;

pub trait PacketBody {
    /// A globally unique identifier for this type.
    fn packet_type() -> PacketType;
}
