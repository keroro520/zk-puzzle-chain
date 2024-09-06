use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub parent_hash: [u8; 32],
    pub zk_pow_receipt: Receipt,
}