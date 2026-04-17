use std::collections::HashMap;
use rand::Rng;

pub struct PoSConsensus {
    validators: HashMap<String, u64>,
    min_stake: u64,
}

impl PoSConsensus {
    pub fn new(min_stake: u64) -> Self {
        Self {
            validators: HashMap::new(),
            min_stake,
        }
    }

    pub fn stake(&mut self, address: String, amount: u64) {
        *self.validators.entry(address).or_insert(0) += amount;
    }

    pub fn unstake(&mut self, address: &str, amount: u64) {
        if let Some(stake) = self.validators.get_mut(address) {
            *stake = stake.saturating_sub(amount);
            if *stake == 0 {
                self.validators.remove(address);
            }
        }
    }

    pub fn select_validator(&self) -> Option<String> {
        let eligible: Vec<(&String, &u64)> = self.validators
            .iter()
            .filter(|(_, &s)| s >= self.min_stake)
            .collect();
        
        if eligible.is_empty() {
            return None;
        }

        let total_stake: u64 = eligible.iter().map(|(_, s)| **s).sum();
        let mut rng = rand::thread_rng();
        let mut rand_val = rng.gen_range(0..total_stake);

        for (addr, stake) in eligible {
            rand_val = rand_val.saturating_sub(**stake);
            if rand_val == 0 {
                return Some(addr.clone());
            }
        }
        eligible.last().map(|(a, _)| a.to_string())
    }

    pub fn slash(&mut self, address: &str) {
        self.validators.remove(address);
    }

    pub fn get_stake(&self, address: &str) -> u64 {
        *self.validators.get(address).unwrap_or(&0)
    }
}
