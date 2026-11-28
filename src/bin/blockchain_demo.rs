#[path = "../blockchain.rs"]
mod blockchain;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    blockchain::run_demo()
}
