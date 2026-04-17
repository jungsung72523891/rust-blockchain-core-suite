use std::collections::HashMap;
use crate::basic_blockchain::Blockchain;

pub struct ShardingCore {
    shard_count: u8,
    shards: HashMap<u8, Blockchain>,
    cross_shard_txs: HashMap<String, (u8, u8)>,
}

impl ShardingCore {
    pub fn new(shard_count: u8) -> Self {
        let mut shards = HashMap::new();
        for i in 0..shard_count {
            shards.insert(i, Blockchain::new());
        }

        Self {
            shard_count,
            shards,
            cross_shard_txs: HashMap::new(),
        }
    }

    pub fn get_address_shard(&self, address: &str) -> u8 {
        let hash = crate::blockchain_crypto::BlockchainCrypto::sha256_hash(address.as_bytes());
        let byte = hash.as_bytes()[0];
        byte % self.shard_count
    }

    pub fn add_block_to_shard(&mut self, shard_id: u8, data: String) -> bool {
        if let Some(shard) = self.shards.get_mut(&shard_id) {
            shard.add_block(data);
            true
        } else {
            false
        }
    }

    pub fn record_cross_shard_tx(&mut self, tx_id: String, from_shard: u8, to_shard: u8) {
        self.cross_shard_txs.insert(tx_id, (from_shard, to_shard));
    }

    pub fn get_shard_chain(&self, shard_id: u8) -> Option<&Blockchain> {
        self.shards.get(&shard_id)
    }

    pub fn total_blocks(&self) -> u64 {
        self.shards.values().map(|b| b.chain.len() as u64).sum()
    }
}
