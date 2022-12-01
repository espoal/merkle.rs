use xxhash_rust::xxh3::{xxh3_64, xxh3_64_with_seed};

pub const HASH_SIZE: usize = 8;

pub type Hasher = Box<dyn Fn(&str) -> [u8; HASH_SIZE]>;

pub fn default_hasher() -> Hasher {
    Box::new(|data: &str| -> [u8; HASH_SIZE] {
        return xxh3_64(data.as_bytes()).to_be_bytes();
    })
}

pub fn default_hasher_with_seed(seed: u64) -> Hasher {
    Box::new(move |data: &str| -> [u8; HASH_SIZE] {
        return xxh3_64_with_seed(data.as_bytes(), seed).to_be_bytes();
    })
}
