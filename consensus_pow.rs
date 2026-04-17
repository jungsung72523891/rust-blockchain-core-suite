use crate::basic_blockchain::{Block, Blockchain};

pub struct PoWConsensus {
    difficulty: u32,
}

impl PoWConsensus {
    pub fn new(difficulty: u32) -> Self {
        Self { difficulty }
    }

    pub fn mine_block(&self, mut block: Block) -> Block {
        let target = "0".repeat(self.difficulty as usize);
        let mut nonce = 0;
        
        loop {
            let hash = Block::calculate_hash(
                block.index,
                block.timestamp,
                &block.data,
                &block.previous_hash,
                nonce,
            );
            
            if hash.starts_with(&target) {
                block.hash = hash;
                block.nonce = nonce;
                break;
            }
            nonce += 1;
        }
        block
    }

    pub fn adjust_difficulty(&self, blockchain: &Blockchain) -> u32 {
        if blockchain.chain.len() < 10 {
            return self.difficulty;
        }
        
        let last_block = blockchain.chain.last().unwrap();
        let prev_10th = &blockchain.chain[blockchain.chain.len() - 10];
        let time_diff = last_block.timestamp - prev_10th.timestamp;
        
        if time_diff < 60000 {
            self.difficulty + 1
        } else if time_diff > 120000 {
            self.difficulty - 1
        } else {
            self.difficulty
        }
    }

    pub fn is_block_valid(&self, block: &Block) -> bool {
        let target = "0".repeat(self.difficulty as usize);
        let computed_hash = Block::calculate_hash(
            block.index,
            block.timestamp,
            &block.data,
            &block.previous_hash,
            block.nonce,
        );
        computed_hash == block.hash && computed_hash.starts_with(&target)
    }
}
