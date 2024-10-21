extern crate sha2;
use sha2::{Sha256, Digest};

pub struct AtomicSwap {
    hashlock: [u8; 32],
    timelock: u64,
}

impl AtomicSwap {
    pub fn new(secret: &[u8], timelock: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let hashlock = hasher.finalize().into();
        AtomicSwap { hashlock, timelock }
    }

    pub fn unlock(&self, secret: &[u8], current_time: u64) -> bool {
        if current_time > self.timelock {
            println!("Timelock expired. Cannot unlock.");
            return false;
        }
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let provided_hashlock = hasher.finalize();
        if provided_hashlock.as_slice() == self.hashlock {
            println!("Atomic swap unlocked successfully.");
            true
        } else {
            println!("Incorrect secret provided.");
            false
        }
    }
}

pub fn start_atomic_swap() {
    let secret = b"cross-chain secret";
    let timelock = 1000;
    let swap = AtomicSwap::new(secret, timelock);
    let result = swap.unlock(secret, 500);
    println!("Atomic swap unlock result: {}", result);
}
