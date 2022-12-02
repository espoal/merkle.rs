use xxhash_rust::xxh3::{xxh3_64, xxh3_64_with_seed};

pub const HASH_SIZE: usize = 8;
pub type HashBuff = [u8; HASH_SIZE];

pub type Hasher = Box<dyn Fn(&[u8]) -> HashBuff>;

pub fn default_hasher() -> Hasher {
    Box::new(|data: &[u8]| -> HashBuff {
        return xxh3_64(data).to_be_bytes();
    })
}

pub fn default_hasher_with_seed(seed: u64) -> Hasher {
    Box::new(move |data: &[u8]| -> HashBuff {
        return xxh3_64_with_seed(data, seed).to_be_bytes();
    })
}
