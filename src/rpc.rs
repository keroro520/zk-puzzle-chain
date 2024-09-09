use jsonrpc_core::{Error, IoHandler, Result, Params, Value};
use jsonrpc_http_server::ServerBuilder;
use serde_json::json;
use crate::core::Block;
use crate::database::Database;

use std::sync::{Arc, Mutex}; 

pub struct RpcServer {
    database: Arc<Mutex<Database>>,
}

impl RpcServer {
    pub fn new(database: Arc<Mutex<Database>>) -> Self {
        RpcServer { database }
    }

    pub fn start(&self, port: u16) -> Result<()> { // Box<dyn std::error::Error>> {
        let mut io = IoHandler::default();

        let db = self.database.clone();
        io.add_method("generate_block", move |params: Params| {
            let db = db.clone();
            async move {
                match params {
                    Params::Array(params) => {
                        let block: Block = serde_json::from_value(params[0].clone())
                            .map_err(|e| Error::invalid_params(format!("Invalid block format: {}", e)))?;
                        let mut locked = db.lock()
                            .map_err(internal_error)?;
                        locked.insert_block(&block).map_err(internal_error)?;
                        Ok(json!({
                            "block_number": block.number,
                            "block_hash": hex::encode(block.hash()),
                        }))
                    }
                    _ => {
                        return Err(Error::invalid_params("Invalid params"));
                    }
                }
            }
        });

        let db = self.database.clone();
        io.add_method("get_tip_block_number", move |_params: Params| {
            let db = db.clone();
            async move {
                let mut locked = db.lock()
                    .map_err(internal_error)?;
                match locked.get_latest_block().map_err(internal_error)? {
                    Some(block) => Ok(Value::Number(block.number.into())),
                    None => Ok(Value::Number(0.into())),
                }
            }
        });

        let db = self.database.clone();
        io.add_method("get_tip_block", move |_params: Params| {
            let db = db.clone();
            async move {
                let mut locked = db.lock()
                    .map_err(internal_error)?;
                match locked.get_latest_block().map_err(internal_error)? {
                    Some(block) => Ok(serde_json::to_value(block).map_err(internal_error)?),
                    None => Ok(Value::Number(0.into())),
                }
            }
        });

        let db = self.database.clone();
        io.add_method("get_block_by_number", move |params: Params| {
            let db = db.clone();
            async move {
                let block_number: u64 = params.parse()?;
                let mut locked = db.lock()
                    .map_err(internal_error)?;
                match locked.get_block(block_number as i64).map_err(internal_error)? {
                    Some(block) => Ok(serde_json::to_value(block).map_err(internal_error)?),
                    None => Err(Error::invalid_params("Block not found")),
                }
            }
        });

        let db = self.database.clone();
        io.add_method("get_block_by_hash", move |params: Params| {
            let db = db.clone();

            async move {
                let block_hash: String = params.parse()?;
                let block_hash = hex::decode(block_hash).map_err(|e| Error::invalid_params(e.to_string()))?;
                if block_hash.len() != 32 {
                    return Err(Error::invalid_params("Invalid block hash"));
                }
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&block_hash);
                
                let mut locked = db.lock()
                    .map_err(internal_error)?;
                match locked.get_block_by_hash(&hash).map_err(internal_error)? {
                    Some(block) => Ok(serde_json::to_value(block).map_err(internal_error)?),
                    None => Err(Error::invalid_params("Block not found")),
                }
            }
        });

        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&format!("127.0.0.1:{}", port).parse().map_err(internal_error)?)
            .expect("Failed to start RPC server");

        println!("RPC server started on port {}", port);
        server.wait();

        Ok(())
    }
}

fn internal_error(e: impl std::error::Error) -> Error {
    eprintln!("Internal error: {:?}", e);
    Error::internal_error()
}