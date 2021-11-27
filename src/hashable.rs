use sha2::{Digest, Sha256};
use sha2::digest::generic_array::GenericArray;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.bytes());
        let res = hasher.finalize();

        res.to_vec()
    }
}
