use std::collections::{HashMap, BTreeMap};
use crate::transaction_core::Transaction;

pub struct TransactionIndexer {
    tx_by_hash: HashMap<String, Transaction>,
    tx_by_address: HashMap<String, Vec<String>>,
    tx_by_block: BTreeMap<u64, Vec<String>>,
}

impl TransactionIndexer {
    pub fn new() -> Self {
        Self {
            tx_by_hash: HashMap::new(),
            tx_by_address: HashMap::new(),
            tx_by_block: BTreeMap::new(),
        }
    }

    pub fn index_transaction(&mut self, tx: Transaction, block_height: u64) {
        let tx_id = tx.tx_id.clone();
        self.tx_by_hash.insert(tx_id.clone(), tx.clone());
        
        self.tx_by_address.entry(tx.from.clone()).or_default().push(tx_id.clone());
        self.tx_by_address.entry(tx.to.clone()).or_default().push(tx_id.clone());
        self.tx_by_block.entry(block_height).or_default().push(tx_id);
    }

    pub fn get_transaction(&self, tx_id: &str) -> Option<&Transaction> {
        self.tx_by_hash.get(tx_id)
    }

    pub fn get_address_transactions(&self, address: &str) -> Vec<&Transaction> {
        self.tx_by_address
            .get(address)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|id| self.tx_by_hash.get(id))
            .collect()
    }

    pub fn get_block_transactions(&self, block_height: u64) -> Vec<&Transaction> {
        self.tx_by_block
            .get(&block_height)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|id| self.tx_by_hash.get(id))
            .collect()
    }

    pub fn clear(&mut self) {
        self.tx_by_hash.clear();
        self.tx_by_address.clear();
        self.tx_by_block.clear();
    }
}
