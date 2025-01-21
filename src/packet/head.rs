use rkyv::{Archive, Serialize};

#[repr(C)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PacketHead {
    /* TODO */
    /* TODO */
}
