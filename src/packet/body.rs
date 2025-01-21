use rkyv::{Archive, Serialize};

#[repr(C)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PacketBody {
    /* TODO */
    /* TODO */
}
