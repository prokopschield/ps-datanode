use rkyv::rancor::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error(transparent)]
    Rancor(#[from] Error),
}
