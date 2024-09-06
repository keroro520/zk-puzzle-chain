use diesel::prelude::*;
// use diesel::sql_types::*;
use crate::core::Block;

table! {
    blocks (number, hash) {
        number -> BigInt,
        hash -> Binary,
        parent_hash -> Binary,
        zk_pow_receipt -> Binary,
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = blocks)]
pub struct StoredBlock {
    pub number: i64,
    pub hash: Vec<u8>,
    pub parent_hash: Vec<u8>,
    pub zk_pow_receipt: Vec<u8>,
}

impl From<Block> for StoredBlock {
    fn from(block: Block) -> Self {

        StoredBlock {
            number: block.number as i64,
            hash: block.hash().to_vec(),
            parent_hash: block.parent_hash.to_vec(),
            zk_pow_receipt: bincode::serialize(&block.zk_pow_receipt).unwrap(),
        }
    }
}

impl From<&StoredBlock> for Block {
    fn from(stored_block: &StoredBlock) -> Self {
        Block {
            number: stored_block.number as u64,
            parent_hash: stored_block.parent_hash.clone().try_into().unwrap(),
            zk_pow_receipt: bincode::deserialize(&stored_block.zk_pow_receipt).unwrap(),
        }
    }
}

// diesel::joinable!(blocks -> blocks (parent_hash));
// diesel::allow_tables_to_appear_in_same_query!(blocks);
