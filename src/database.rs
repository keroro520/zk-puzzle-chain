use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::blocks;
use crate::schema::StoredBlock;
use crate::core::Block;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        let connection = SqliteConnection::establish(database_url)?;
        Ok(Database { connection })
    }

    pub fn insert_block(&mut self, block: &Block) -> Result<(), diesel::result::Error> {
        let stored_block: StoredBlock = block.into();
        diesel::insert_into(blocks::table)
            .values(&stored_block)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn get_block(&mut self, number: i64) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;

        let result = blocks
            .filter(crate::schema::blocks::number.eq(number))
            .first::<StoredBlock>(&mut self.connection)
            .optional()?;

        Ok(result.map(|stored_block| (&stored_block).into()))
    }

    pub fn get_latest_block(&mut self) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;

        let result = blocks
            .order(crate::schema::blocks::number.desc())
            .first::<StoredBlock>(&mut self.connection)
            .optional()?;

        Ok(result.map(|stored_block| (&stored_block).into()))
    }
}
