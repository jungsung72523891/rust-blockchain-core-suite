use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    balances: HashMap<String, u64>,
    contracts: HashMap<String, Vec<u8>>,
    block_height: u64,
    snapshots: HashMap<u64, ChainStateSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChainStateSnapshot {
    balances: HashMap<String, u64>,
    contracts: HashMap<String, Vec<u8>>,
}

impl ChainState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            contracts: HashMap::new(),
            block_height: 0,
            snapshots: HashMap::new(),
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn set_balance(&mut self, address: String, amount: u64) {
        self.balances.insert(address, amount);
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_balance = self.get_balance(from);
        if from_balance < amount {
            return false;
        }
        self.balances.insert(from.to_string(), from_balance - amount);
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        true
    }

    pub fn store_contract(&mut self, address: String, code: Vec<u8>) {
        self.contracts.insert(address, code);
    }

    pub fn get_contract(&self, address: &str) -> Option<&Vec<u8>> {
        self.contracts.get(address)
    }

    pub fn create_snapshot(&mut self, height: u64) {
        let snapshot = ChainStateSnapshot {
            balances: self.balances.clone(),
            contracts: self.contracts.clone(),
        };
        self.snapshots.insert(height, snapshot);
        self.block_height = height;
    }

    pub fn rollback(&mut self, height: u64) -> bool {
        if let Some(snapshot) = self.snapshots.get(&height) {
            self.balances = snapshot.balances.clone();
            self.contracts = snapshot.contracts.clone();
            self.block_height = height;
            true
        } else {
            false
        }
    }
}
