use crate::core::Block;
use crate::schema::blocks;
use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Result<Self, diesel::ConnectionError> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)?;
        let mut database = Database { connection };
        database.ensure_tables_initialized();
        Ok(database)
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

    pub fn get_block_by_hash(
        &mut self,
        hash_: &[u8; 32],
    ) -> Result<Option<Block>, diesel::result::Error> {
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

    pub fn get_blocks(
        &mut self,
        start: i64,
        limit: i64,
    ) -> Result<Vec<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let results = blocks
            .filter(crate::schema::blocks::number.ge(start))
            .order(crate::schema::blocks::number.asc())
            .limit(limit)
            .load::<crate::schema::StoredBlock>(&mut self.connection)?;
        Ok(results
            .iter()
            .map(|stored_block| Block::from(stored_block))
            .collect())
    }

    pub fn init_tables(&mut self) {
        let create_table_sql = "
            CREATE TABLE IF NOT EXISTS blocks (
                number BIGINT NOT NULL,
                hash BYTEA NOT NULL,
                parent_hash BYTEA NOT NULL,
                zk_pow_receipt BYTEA NOT NULL,
                PRIMARY KEY (number, hash)
            )
        ";

        // execute sql
        self.connection
            .batch_execute(create_table_sql)
            .expect("Failed to create table");
    }

    pub fn ensure_tables_initialized(&mut self) {
        use diesel::dsl::sql;
        use diesel::sql_types::Bool;

        let table_exists: bool = diesel::select(sql::<Bool>(
            "EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'blocks')",
        ))
        .get_result(&mut self.connection)
        .expect("Failed to check if table exists");

        if !table_exists {
            self.init_tables();
        }
    }
}
