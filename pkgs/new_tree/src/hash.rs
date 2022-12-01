pub const HASH_SIZE: usize = 8;

pub type Hasher = fn(&str) -> [u8; HASH_SIZE];

pub fn default_hasher(data: &str) -> [u8; HASH_SIZE] {
    [0; HASH_SIZE]
}

pub fn default_hasher_with_seed(seed: u64) -> Hasher {
    fn hasher(data: &str) -> [u8; HASH_SIZE] {
        [0; HASH_SIZE]
    }

    hasher
}
