use std::collections::{HashMap, BTreeSet};
use crate::transaction_core::Transaction;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MempoolTx {
    tx_id: String,
    fee: u64,
    timestamp: u128,
}

impl Ord for MempoolTx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.fee.cmp(&self.fee)
            .then_with(|| self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialOrd for MempoolTx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MempoolManager {
    transactions: HashMap<String, Transaction>,
    ordered_txs: BTreeSet<MempoolTx>,
    max_size: usize,
}

impl MempoolManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            ordered_txs: BTreeSet::new(),
            max_size,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if self.transactions.contains_key(&tx.tx_id) || self.transactions.len() >= self.max_size {
            return false;
        }

        let mempool_tx = MempoolTx {
            tx_id: tx.tx_id.clone(),
            fee: tx.fee,
            timestamp: tx.timestamp,
        };

        self.transactions.insert(tx.tx_id.clone(), tx);
        self.ordered_txs.insert(mempool_tx);
        true
    }

    pub fn remove_transaction(&mut self, tx_id: &str) {
        if let Some(tx) = self.transactions.remove(tx_id) {
            let mempool_tx = MempoolTx {
                tx_id: tx.tx_id,
                fee: tx.fee,
                timestamp: tx.timestamp,
            };
            self.ordered_txs.remove(&mempool_tx);
        }
    }

    pub fn get_top_transactions(&self, count: usize) -> Vec<Transaction> {
        self.ordered_txs
            .iter()
            .take(count)
            .filter_map(|mt| self.transactions.get(&mt.tx_id))
            .cloned()
            .collect()
    }

    pub fn contains(&self, tx_id: &str) -> bool {
        self.transactions.contains_key(tx_id)
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
        self.ordered_txs.clear();
    }
}
