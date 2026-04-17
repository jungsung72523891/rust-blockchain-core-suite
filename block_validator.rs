use crate::{basic_blockchain::Block, transaction_core::Transaction, blockchain_crypto::BlockchainCrypto};

pub struct BlockValidator;

impl BlockValidator {
    pub fn validate_block_structure(block: &Block) -> bool {
        if block.index == 0 {
            return block.previous_hash == "0" && !block.hash.is_empty();
        }

        let computed_hash = Block::calculate_hash(
            block.index,
            block.timestamp,
            &block.data,
            &block.previous_hash,
            block.nonce,
        );

        computed_hash == block.hash
    }

    pub fn validate_prev_block(block: &Block, prev_block: &Block) -> bool {
        block.index == prev_block.index + 1 && block.previous_hash == prev_block.hash
    }

    pub fn validate_transactions(transactions: &[Transaction]) -> bool {
        for tx in transactions {
            if tx.from.is_empty() || tx.to.is_empty() || tx.amount == 0 {
                return false;
            }
            if tx.tx_id != BlockchainCrypto::sha256_hash(tx.get_sign_data().as_bytes()) {
                return false;
            }
        }
        true
    }

    pub fn validate_timestamp(block: &Block, prev_block: &Block) -> bool {
        block.timestamp > prev_block.timestamp && 
        block.timestamp <= std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() + 3600000
    }

    pub fn full_validation(block: &Block, prev_block: &Block) -> bool {
        Self::validate_block_structure(block) &&
        Self::validate_prev_block(block, prev_block) &&
        Self::validate_timestamp(block, prev_block)
    }
}
