use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const GENESIS_PREVIOUS_HASH: &str = "0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    fn calculate_hash(index: u64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}:{}:{}:{}", index, data.len(), data, previous_hash);

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }

    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let hash = Self::calculate_hash(index, &data, &previous_hash);

        Self {
            index,
            data,
            previous_hash,
            hash,
        }
    }

    fn is_hash_valid(&self) -> bool {
        self.hash == Self::calculate_hash(self.index, &self.data, &self.previous_hash)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(
            0,
            "Genesis Block".to_string(),
            GENESIS_PREVIOUS_HASH.to_string(),
        );

        Self {
            chain: vec![genesis],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let previous = self.chain.last().expect("Blockchain cannot be empty");

        let block = Block::new(previous.index + 1, data.to_string(), previous.hash.clone());

        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        let genesis = &self.chain[0];

        if genesis.index != 0
            || genesis.previous_hash != GENESIS_PREVIOUS_HASH
            || !genesis.is_hash_valid()
        {
            return false;
        }

        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.index != i as u64
                || current.previous_hash != previous.hash
                || !current.is_hash_valid()
            {
                return false;
            }
        }

        true
    }

    pub fn tamper_block(&mut self, index: usize, new_data: &str) -> bool {
        if let Some(block) = self.chain.get_mut(index) {
            block.data = new_data.to_string();
            true
        } else {
            false
        }
    }

    pub fn rebuild_from(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.chain.len() {
            return false;
        }

        for i in index..self.chain.len() {
            let previous_hash = self.chain[i - 1].hash.clone();
            let block = &mut self.chain[i];

            block.index = i as u64;
            block.previous_hash = previous_hash;
            block.hash = Block::calculate_hash(block.index, &block.data, &block.previous_hash);
        }

        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("------------------------------");
            println!("Block: {}", block.index);
            println!("Data: {}", block.data);
            println!("Previous hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
        }

        println!("------------------------------");
        println!("Chain valid: {}", self.is_valid());
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BLOCKCHAIN DEMONSTRATION ===");

    let mut blockchain = Blockchain::new();

    blockchain.add_block("User authentication successful");
    blockchain.add_block("Wallet created");
    blockchain.add_block("Token account created");
    blockchain.add_block("Transaction signed");
    blockchain.add_block("Security event recorded");

    println!("\n=== ORIGINAL BLOCKCHAIN ===");
    blockchain.print_chain();

    assert!(blockchain.is_valid());

    println!("\n=== TAMPERING EXPERIMENT ===");

    blockchain.tamper_block(2, "Modified transaction data");
    blockchain.print_chain();

    assert!(!blockchain.is_valid());

    println!("\nTampering detected successfully!");

    println!("\n=== BLOCKCHAIN RECOVERY ===");

    blockchain.rebuild_from(2);
    blockchain.print_chain();

    assert!(blockchain.is_valid());

    println!("\nBlockchain successfully restored!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_and_five_blocks_are_valid() {
        let mut blockchain = Blockchain::new();

        for i in 1..=5 {
            blockchain.add_block(&format!("Test event {i}"));
        }

        assert_eq!(blockchain.chain.len(), 6);
        assert!(blockchain.is_valid());
    }

    #[test]
    fn modified_block_is_detected() {
        let mut blockchain = Blockchain::new();

        for i in 1..=5 {
            blockchain.add_block(&format!("Event {i}"));
        }

        assert!(blockchain.tamper_block(2, "Modified data"));
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn modified_block_breaks_following_links() {
        let mut blockchain = Blockchain::new();

        for i in 1..=5 {
            blockchain.add_block(&format!("Event {i}"));
        }

        blockchain.tamper_block(2, "Tampered event");

        let block = &blockchain.chain[2];

        let recalculated_hash =
            Block::calculate_hash(block.index, &block.data, &block.previous_hash);

        assert_ne!(block.hash, recalculated_hash);
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn rebuilding_restores_chain_integrity() {
        let mut blockchain = Blockchain::new();

        for i in 1..=5 {
            blockchain.add_block(&format!("Event {i}"));
        }

        blockchain.tamper_block(2, "Modified event");

        assert!(!blockchain.is_valid());
        assert!(blockchain.rebuild_from(2));
        assert!(blockchain.is_valid());
    }

    #[test]
    fn invalid_rebuild_index_is_rejected() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("Event");

        assert!(!blockchain.rebuild_from(0));
        assert!(!blockchain.rebuild_from(10));
    }
}
