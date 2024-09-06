use risc0_zkvm::guest::env;

fn main() {
    let x: u32 = env::read();
    let y: u32 = env::read();

    assert!(x + y == 1024, "x + y must be 1024");
}
