pub mod core;
pub mod database;
pub mod schema;
pub mod rpc;

fn main() {
    dotenv::dotenv().ok();

    // Initialize the database
    let db = ::std::sync::Arc::new(::std::sync::Mutex::new(crate::database::Database::new().expect("Failed to initialize database")));

    // Create and start the RPC server
    let rpc_server = crate::rpc::RpcServer::new(db);
    let port = std::env::var("RPC_PORT").unwrap_or("8999".to_string()).parse().unwrap();
    if let Err(e) = rpc_server.start(port) {
        eprintln!("Failed to start RPC server: {:?}", e);
    } else {
        println!("RPC server started on port {}", port);
    }
}
