# Simple Blockchain Implementation in Rust

This is a simple blockchain implementation built using Rust. The project was created to understand how blockchains work and demonstrate Rust skills. It supports Proof of Work (PoW) mining and allows for the addition of new blocks to the chain.

## Features

- **Blocks:** Each block contains an index, timestamp, data, previous block hash, current block hash, and a nonce.
- **Blockchain:** A chain of blocks starting with a hardcoded "Genesis Block".
- **Proof of Work (PoW):** Each block is mined by solving a hash puzzle with a certain difficulty level (number of leading zeros in the hash).
- **Validation:** The blockchain can be validated to ensure that no blocks have been tampered with.

## Project Structure

```
├── src
│   ├── block.rs    # Block struct and block-related functions
│   ├── blockchain.rs # Blockchain struct and functions for managing the chain
│   └── main.rs     # Entry point; sets up and runs the blockchain
└── Cargo.toml      # Project configuration
```

## How It Works

1. **Block**: A block stores essential information like:
   - index: Position in the chain.
   - timestamp: When the block was created.
   - data: Content stored in the block (e.g., transactions).
   - previous_hash: Hash of the previous block in the chain.
   - hash: Hash of the current block (calculated using its data and nonce).
   - nonce: A counter used to modify the hash until a valid one is found (mining).

2. **Mining**: Mining involves finding a hash that meets a specific difficulty target (a hash with a certain number of leading zeros). The block's nonce is incremented repeatedly until a valid hash is found.

3. **Blockchain**: The blockchain is a sequence of blocks starting with the **Genesis Block**. New blocks are added to the chain, and each block is linked to the previous one by storing the previous block's hash.

4. **Validation**: The blockchain is validated by checking if each block's hash is correctly calculated and if the previous_hash of each block matches the hash of the preceding block.

## Example Output

```bash
Mining block 1...
Block mined: 000061ca53b22ba02dc21c3b395c0abda92df5dda8fc34f0d1342e9378672178
Mining block 2...
Block mined: 00000e8b122f04b789ef16eb8fe6503cb35fe49671173a0a3f79ed7e4169a876
Is blockchain valid? true
```

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/simple-blockchain-rust.git
   ```

2. Navigate into the project directory:
   ```bash
   cd simple-blockchain-rust
   ```

3. Build and run the project:
   ```bash
   cargo run
   ```

## Configuration

- **Difficulty**: The mining difficulty can be adjusted by modifying the `Blockchain::new(difficulty)` function call in `main.rs`. A higher difficulty will require more computational effort for mining.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Code Sample

Here's a sample of the blockchain validation function:

```rust
pub fn is_valid_chain(&self) -> bool {
    for i in 1..self.chain.len() {
        let current_block = &self.chain[i];
        let previous_block = &self.chain[i - 1];

        // Validate current block's hash and previous block's hash
        if current_block.hash != current_block.calculate_hash() || 
           current_block.previous_hash != previous_block.hash {
            return false;
        }
    }
    true
}
```

The validation starts from block `1` because block `0` (Genesis Block) has no "previous block" to check against.