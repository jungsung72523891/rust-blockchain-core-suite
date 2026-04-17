use crate::{
    basic_blockchain::{Blockchain, Block},
    state_store::ChainState,
    transaction_core::Transaction,
};

pub struct ChainBootstrap;

impl ChainBootstrap {
    pub fn create_genesis_block(alloc: Vec<(String, u64)>) -> (Blockchain, ChainState) {
        let mut blockchain = Blockchain::new();
        let mut state = ChainState::new();

        for (address, balance) in alloc {
            state.set_balance(address, balance);
        }

        let genesis_data = serde_json::to_string(&alloc).unwrap_or_default();
        let genesis_block = Block::new(0, genesis_data, "0".to_string());
        blockchain.chain.clear();
        blockchain.chain.push(genesis_block);
        state.create_snapshot(0);

        (blockchain, state)
    }

    pub fn init_testnet() -> (Blockchain, ChainState) {
        let alloc = vec![
            ("0x0000000000000000000000000000000000000001".to_string(), 1_000_000_000),
            ("0x0000000000000000000000000000000000000002".to_string(), 500_000_000),
        ];
        Self::create_genesis_block(alloc)
    }

    pub fn validate_genesis(block: &Block) -> bool {
        block.index == 0 && block.previous_hash == "0" && !block.hash.is_empty()
    }

    pub fn genesis_transactions() -> Vec<Transaction> {
        Vec::new()
    }
}
