use std::collections::HashSet;
use crate::blockchain_crypto::BlockchainCrypto;

pub struct MultiSigWallet {
    owners: HashSet<String>,
    required: u8,
    transactions: HashMap<String, MultiSigTx>,
}

#[derive(Debug, Clone)]
struct MultiSigTx {
    to: String,
    value: u64,
    data: Vec<u8>,
    confirmations: HashSet<String>,
    executed: bool,
}

impl MultiSigWallet {
    pub fn new(owners: Vec<String>, required: u8) -> Self {
        Self {
            owners: owners.into_iter().collect(),
            required,
            transactions: HashMap::new(),
        }
    }

    pub fn submit_transaction(&mut self, sender: &str, to: String, value: u64, data: Vec<u8>) -> Option<String> {
        if !self.owners.contains(sender) {
            return None;
        }

        let tx_id = BlockchainCrypto::sha256_hash(format!("{}{}{:?}", to, value, data).as_bytes());
        self.transactions.insert(tx_id.clone(), MultiSigTx {
            to,
            value,
            data,
            confirmations: HashSet::new(),
            executed: false,
        });
        Some(tx_id)
    }

    pub fn confirm_transaction(&mut self, sender: &str, tx_id: &str) -> bool {
        let tx = match self.transactions.get_mut(tx_id) {
            Some(t) => t,
            None => return false,
        };

        if !self.owners.contains(sender) || tx.executed {
            return false;
        }

        tx.confirmations.insert(sender.to_string());
        true
    }

    pub fn execute_transaction(&mut self, tx_id: &str) -> bool {
        let tx = self.transactions.get_mut(tx_id)?;
        if tx.executed || tx.confirmations.len() < self.required as usize {
            return false;
        }
        tx.executed = true;
        true
    }

    pub fn is_confirmed(&self, tx_id: &str) -> bool {
        self.transactions
            .get(tx_id)
            .map(|t| t.confirmations.len() >= self.required as usize && !t.executed)
            .unwrap_or(false)
    }
}
