use std::collections::HashMap;
use crate::transaction_core::Transaction;

pub struct TransactionReplacement {
    user_nonces: HashMap<String, u64>,
    pending_txs: HashMap<String, Transaction>,
    min_fee_increase: u64,
}

impl TransactionReplacement {
    pub fn new(min_fee_increase: u64) -> Self {
        Self {
            user_nonces: HashMap::new(),
            pending_txs: HashMap::new(),
            min_fee_increase,
        }
    }

    pub fn add_pending(&mut self, tx: Transaction) -> bool {
        let nonce = self.user_nonces.entry(tx.from.clone()).or_insert(0);
        if tx.fee < *nonce {
            return false;
        }
        *nonce = tx.fee;
        self.pending_txs.insert(tx.tx_id.clone(), tx);
        true
    }

    pub fn replace_transaction(&mut self, old_tx_id: &str, new_tx: Transaction) -> bool {
        let old_tx = match self.pending_txs.get(old_tx_id) {
            Some(t) => t,
            None => return false,
        };

        if old_tx.from != new_tx.from || new_tx.fee < old_tx.fee + self.min_fee_increase {
            return false;
        }

        self.pending_txs.remove(old_tx_id);
        self.pending_txs.insert(new_tx.tx_id.clone(), new_tx);
        true
    }

    pub fn cancel_transaction(&mut self, tx_id: &str, cancel_tx: Transaction) -> bool {
        let old_tx = match self.pending_txs.get(tx_id) {
            Some(t) => t,
            None => return false,
        };

        if old_tx.from != cancel_tx.from || cancel_tx.fee < old_tx.fee + self.min_fee_increase {
            return false;
        }

        self.pending_txs.remove(tx_id);
        true
    }

    pub fn get_pending(&self, tx_id: &str) -> Option<&Transaction> {
        self.pending_txs.get(tx_id)
    }
}
