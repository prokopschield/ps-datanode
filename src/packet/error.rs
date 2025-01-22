use std::array::TryFromSliceError;

use rkyv::rancor::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error(transparent)]
    Rancor(#[from] Error),
    #[error(transparent)]
    TryFromSliceError(#[from] TryFromSliceError),
    #[error("Incorrect checksum: {0} != {1}")]
    IncorrectChecksum(u64, u64),
    #[error("Incorrent length prefix: {0} != {1}")]
    IncorrectLengthPrefix(u32, u32),
    #[error("Unsupported protocol version {0}")]
    UnsupportedProtocolVersion(u32),
}
