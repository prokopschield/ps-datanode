pub const PROTOCOL_VERSION: u32 = 1;
pub const PACKET_PREFIX_LENGTH: usize = std::mem::size_of::<(u32, u32, u64)>();
