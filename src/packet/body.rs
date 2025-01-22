use std::ops::Deref;

use rkyv::{
    api::high::HighSerializer,
    bytecheck::CheckBytes,
    rancor::{Error, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    validation::{archive::ArchiveValidator, shared::SharedValidator, Validator},
    Archive, Serialize,
};

#[repr(transparent)]
#[derive(Archive, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PacketBody<T = ()>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    pub inner: T,
}

impl<T> PacketBody<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> AsRef<T> for PacketBody<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Deref for PacketBody<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<T> for PacketBody<T>
where
    for<'a> T: Archive + Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
    for<'a> <T as Archive>::Archived:
        CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
{
    fn from(inner: T) -> Self {
        Self { inner }
    }
}
