use std::collections::{HashSet, HashMap};
use std::time::Instant;

pub struct ValidatorSet {
    active_validators: HashSet<String>,
    pending_validators: HashSet<String>,
    jailed_validators: HashSet<String>,
    validator_stakes: HashMap<String, u64>,
    min_stake: u64,
}

impl ValidatorSet {
    pub fn new(min_stake: u64) -> Self {
        Self {
            active_validators: HashSet::new(),
            pending_validators: HashSet::new(),
            jailed_validators: HashSet::new(),
            validator_stakes: HashMap::new(),
            min_stake,
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u64) -> bool {
        if self.jailed_validators.contains(&address) || stake < self.min_stake {
            return false;
        }
        self.pending_validators.insert(address);
        self.validator_stakes.insert(address, stake);
        true
    }

    pub fn activate_pending(&mut self) {
        for addr in self.pending_validators.drain() {
            self.active_validators.insert(addr);
        }
    }

    pub fn jail_validator(&mut self, address: &str) {
        self.active_validators.remove(address);
        self.pending_validators.remove(address);
        self.jailed_validators.insert(address.to_string());
    }

    pub fn unjail_validator(&mut self, address: &str) {
        self.jailed_validators.remove(address);
    }

    pub fn is_active(&self, address: &str) -> bool {
        self.active_validators.contains(address)
    }

    pub fn get_stake(&self, address: &str) -> u64 {
        *self.validator_stakes.get(address).unwrap_or(&0)
    }

    pub fn active_count(&self) -> usize {
        self.active_validators.len()
    }
}
