use crate::block::Block;

// Blockchain structure
pub struct Blockchain {
    pub chain: Vec<Block>,  // A vector of blocks representing the chain
    pub difficulty: usize,  // Difficulty level for mining (number of leading zeros)
}

impl Blockchain {
    // Create a new blockchain with the genesis block
    pub fn new(difficulty: usize) -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(), // Empty chain
            difficulty,
        };

        // Add the genesis block (the first block in the blockchain)
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    // Create a new block and add it to the chain
    pub fn add_block(&mut self, data: String) {
        let previous_hash = if self.chain.is_empty() {
            String::from("0") // If it's the first block, the previous hash is '0'
        } else {
            self.chain.last().unwrap().hash.clone() // Get the last block's hash
        };

        let mut new_block = Block::new(self.chain.len() as u32, data, previous_hash);
        new_block.mine_block(self.difficulty); // Mine the new block
        self.chain.push(new_block); // Add it to the chain
    }

    // Validate the entire blockchain
    pub fn is_valid_chain(&self) -> bool {
        /*
        The loop in the is_valid_chain function starts from 1 because the Genesis Block (at index 0) doesn't 
        have a previous block to validate against. So, the loop starts with the first block after the 
        Genesis Block (i.e., Block 1) and checks the integrity of the chain from there.
         */
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's hash is valid
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // Check if the current block's previous hash matches the previous block's hash
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
