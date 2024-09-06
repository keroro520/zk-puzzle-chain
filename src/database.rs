use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use crate::schema::blocks;
use crate::core::Block;

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Result<Self, diesel::ConnectionError> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)?;
        Ok(Database { connection })
    }

    pub fn insert_block(&mut self, block: &Block) -> Result<(), diesel::result::Error> {
        let stored_block = crate::schema::StoredBlock::from(block);
        diesel::insert_into(blocks::table)
            .values(&stored_block)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn get_block(&mut self, number_: i64) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let result = blocks
            .filter(crate::schema::blocks::number.eq(number_))
            .first::<crate::schema::StoredBlock>(&mut self.connection)
            .optional()?;
        Ok(result.map(|stored_block| Block::from(&stored_block)))
    }

    pub fn get_block_by_hash(&mut self, hash_: &[u8; 32]) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let result = blocks
            .filter(crate::schema::blocks::hash.eq(hash_.to_vec()))
            .first::<crate::schema::StoredBlock>(&mut self.connection)
            .optional()?;
        Ok(result.map(|stored_block| Block::from(&stored_block)))
    }

    pub fn get_latest_block(&mut self) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let result = blocks
            .order(crate::schema::blocks::number.desc())
            .first::<crate::schema::StoredBlock>(&mut self.connection)
            .optional()?;
        Ok(result.map(|stored_block| Block::from(&stored_block)))
    }

    pub fn get_blocks(&mut self, start: i64, limit: i64) -> Result<Vec<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let results = blocks
            .filter(crate::schema::blocks::number.ge(start))
            .order(crate::schema::blocks::number.asc())
            .limit(limit)
            .load::<crate::schema::StoredBlock>(&mut self.connection)?;
        Ok(results.iter().map(|stored_block| Block::from(stored_block)).collect())
    }
}
