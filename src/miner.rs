pub mod core;

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use methods::{ZK_POW_GUEST_ELF, ZK_POW_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json::{Map, Value};
use tokio;

use crate::core::Block;

#[tokio::main]
async fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Create two inputs that sum to 1024
    let x: u32 = 512;
    let y: u32 = 512;

    // Create an executor environment and add the inputs
    let env = ExecutorEnv::builder()
        .write(&x)
        .unwrap()
        .write(&y)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover
    let prover = default_prover();

    // Prove the guest program
    let prove_info = prover.prove(env, ZK_POW_GUEST_ELF).unwrap();

    // Extract the receipt
    let receipt = prove_info.receipt;

    // Verify the receipt
    receipt.verify(ZK_POW_GUEST_ID).unwrap();

    let client: HttpClient = HttpClientBuilder::default()
        .build("http://127.0.0.1:8999")
        .unwrap();

    // Get Tip Block number via RPC `get_tip_block_number`
    let tip_block_number: u64 = client
        .request("get_tip_block_number", jsonrpsee::rpc_params!())
        .await
        .expect("Failed to get tip block number");

    // Generate a new block via `generate_block`
    let block = Block {
        number: tip_block_number + 1,
        parent_hash: [0; 32],
        zk_pow_receipt: receipt,
    };
    let result: Value = client
        .request("generate_block", jsonrpsee::rpc_params!(block))
        .await
        .expect("Failed to generate block");
    println!(
        "Block generated: {}",
        serde_json::to_string(&result).unwrap()
    );
}
