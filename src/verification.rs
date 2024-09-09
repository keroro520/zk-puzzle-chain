use crate::core::Block;
use methods::ZK_POW_GUEST_ID;

#[derive(Debug)]
pub struct BlockHeaderVerification {
    pub parent_block_number: u64,
    pub parent_block_hash: [u8; 32],
    pub block: Block,
}

impl BlockHeaderVerification {
    pub fn verify(&self) -> Result<(), String> {
        // Verify the parent block number and hash
        if self.block.number != self.parent_block_number + 1 {
            return Err("Invalid block number".to_string());
        }
        if self.block.parent_hash != self.parent_block_hash {
            return Err("Invalid parent block hash".to_string());
        }

        // Verify the ZK-PoW proof using RISC0
        match self.block.zk_pow_receipt.verify(ZK_POW_GUEST_ID) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("ZK-PoW proof verification failed: {}", e)),
        }
    }
}
