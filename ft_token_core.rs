use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FungibleToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<String, HashMap<String, u64>>,
}

impl FungibleToken {
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: String) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner, total_supply);
        
        Self {
            name,
            symbol,
            decimals,
            total_supply,
            balances,
            allowances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_balance = self.balances.get(from).copied().unwrap_or(0);
        if from_balance < amount {
            return false;
        }

        self.balances.insert(from.to_string(), from_balance - amount);
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        true
    }

    pub fn approve(&mut self, owner: &str, spender: String, amount: u64) {
        let owner_allowances = self.allowances.entry(owner.to_string()).or_insert_with(HashMap::new);
        owner_allowances.insert(spender, amount);
    }

    pub fn transfer_from(&mut self, spender: &str, from: &str, to: &str, amount: u64) -> bool {
        let allowance = self.allowances
            .get(from)
            .and_then(|a| a.get(spender))
            .copied()
            .unwrap_or(0);

        if allowance < amount {
            return false;
        }

        if !self.transfer(from, to, amount) {
            return false;
        }

        let owner_allowances = self.allowances.get_mut(from).unwrap();
        owner_allowances.insert(spender.to_string(), allowance - amount);
        true
    }

    pub fn balance_of(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
}
