use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub parent_hash: [u8; 32],
    pub zk_pow_receipt: Receipt,
}

impl Block {
    pub fn hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&self.number.to_le_bytes());
        hasher.update(&self.parent_hash);
        hasher.update(&bincode::serialize(&self.zk_pow_receipt).unwrap());
        hasher.finalize().into()
    }
}
