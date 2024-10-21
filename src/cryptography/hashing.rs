use blake2::{Blake2b, Digest};

pub fn hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
