pub mod core;

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    ZK_POW_GUEST_ELF, ZK_POW_GUEST_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use crate::core::Block;

fn main() {
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
    let prove_info = prover
        .prove(env, ZK_POW_GUEST_ELF)
        .unwrap();

    // Extract the receipt
    let receipt = prove_info.receipt;

    // Verify the receipt
    receipt
        .verify(ZK_POW_GUEST_ID)
        .unwrap();

    // Get Tip Block number via RPC `get_tip_block_number`

    println!("Current tip block number: {}", tip_block_number);



    // Generate a new block via `generate_block`
    let block = Block {
        number: 0,
        parent_hash: [0; 32],
        zk_pow_receipt: receipt,
    };

    println!("Guest program successfully verified.");
    println!("Inputs: x = {}, y = {}", x, y);
    println!("Sum: x + y = {}", x + y);
}
