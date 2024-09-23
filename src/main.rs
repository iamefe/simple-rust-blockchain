mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    // Create a new blockchain with difficulty 4 (adjust difficulty level here)
    let mut my_blockchain = Blockchain::new(4);

    // Add some blocks with data
    println!("Mining block 1...");
    my_blockchain.add_block("Block 1 Data".to_string());

    println!("Mining block 2...");
    my_blockchain.add_block("Block 2 Data".to_string());

    println!("Mining block 3...");
    my_blockchain.add_block("Block 3 Data".to_string());

    // Display the blockchain
    for block in &my_blockchain.chain {
        println!("{:?}", block);
    }

    // Validate the blockchain
    println!("Is blockchain valid? {}", my_blockchain.is_valid_chain());
}
