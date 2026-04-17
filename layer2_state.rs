use std::collections::HashMap;
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone)]
pub struct Layer2State {
    pub state_root: String,
    pub accounts: HashMap<String, L2Account>,
    pub tx_count: u64,
    pub batch_number: u64,
}

#[derive(Debug, Clone)]
pub struct L2Account {
    pub balance: u64,
    pub nonce: u64,
    pub last_update_batch: u64,
}

impl Layer2State {
    pub fn new() -> Self {
        Self {
            state_root: String::new(),
            accounts: HashMap::new(),
            tx_count: 0,
            batch_number: 0,
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.accounts.get(address).map(|a| a.balance).unwrap_or(0)
    }

    pub fn update_balance(&mut self, address: String, balance: u64) {
        let nonce = self.accounts.get(&address).map(|a| a.nonce + 1).unwrap_or(0);
        self.accounts.insert(address, L2Account {
            balance,
            nonce,
            last_update_batch: self.batch_number,
        });
        self.update_state_root();
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_balance = self.get_balance(from);
        if from_balance < amount {
            return false;
        }

        self.update_balance(from.to_string(), from_balance - amount);
        self.update_balance(to.to_string(), self.get_balance(to) + amount);
        self.tx_count += 1;
        true
    }

    fn update_state_root(&mut self) {
        let data = format!("{:?}{}", self.accounts, self.tx_count);
        self.state_root = BlockchainCrypto::sha256_hash(data.as_bytes());
    }

    pub fn new_batch(&mut self) {
        self.batch_number += 1;
    }
}
