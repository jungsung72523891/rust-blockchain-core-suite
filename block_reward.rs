use crate::state_store::ChainState;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BlockReward {
    base_reward: u64,
    halving_interval: u64,
    halving_factor: f64,
    treasury_address: String,
    treasury_tax: u8,
}

impl BlockReward {
    pub fn new(
        base_reward: u64,
        halving_interval: u64,
        treasury_address: String,
        treasury_tax: u8,
    ) -> Self {
        Self {
            base_reward,
            halving_interval,
            halving_factor: 0.5,
            treasury_address,
            treasury_tax,
        }
    }

    pub fn calculate_reward(&self, block_height: u64) -> u64 {
        let halvings = block_height / self.halving_interval;
        let mut reward = self.base_reward as f64;
        
        for _ in 0..halvings {
            reward *= self.halving_factor;
        }
        reward as u64
    }

    pub fn distribute_reward(
        &self,
        state: &mut ChainState,
        miner: String,
        block_height: u64,
    ) -> u64 {
        let total = self.calculate_reward(block_height);
        let treasury = total * self.treasury_tax as u64 / 100;
        let miner_reward = total - treasury;

        state.transfer(&self.treasury_address, &miner, miner_reward);
        total
    }

    pub fn mint_reward(&self, state: &mut ChainState, block_height: u64) {
        let reward = self.calculate_reward(block_height);
        let current = state.get_balance(&self.treasury_address);
        state.set_balance(self.treasury_address.clone(), current + reward);
    }
}
