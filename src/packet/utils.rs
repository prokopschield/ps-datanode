use std::hash::Hasher;

pub fn checksum(data: &[u8]) -> u64 {
    let mut hasher = fnv::FnvHasher::default();

    hasher.write(data);

    hasher.finish()
}
