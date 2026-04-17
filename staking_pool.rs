use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StakingPool {
    stakes: HashMap<String, UserStake>,
    total_staked: u64,
    reward_rate: f64,
    unstake_lock_period: u64,
}

#[derive(Debug, Clone)]
struct UserStake {
    amount: u64,
    timestamp: u128,
    reward_debt: u64,
}

impl StakingPool {
    pub fn new(reward_rate: f64, unstake_hours: u64) -> Self {
        Self {
            stakes: HashMap::new(),
            total_staked: 0,
            reward_rate,
            unstake_lock_period: unstake_hours * 3600,
        }
    }

    pub fn stake(&mut self, user: String, amount: u64) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let entry = self.stakes.entry(user).or_insert(UserStake {
            amount: 0,
            timestamp: now,
            reward_debt: 0,
        });
        entry.amount += amount;
        self.total_staked += amount;
    }

    pub fn unstake(&mut self, user: &str, amount: u64) -> Result<u64, String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let stake = self.stakes.get_mut(user).ok_or("No stake found")?;
        
        if now - stake.timestamp < self.unstake_lock_period {
            return Err("Unstake lock period not expired".to_string());
        }
        if stake.amount < amount {
            return Err("Insufficient stake".to_string());
        }

        stake.amount -= amount;
        self.total_staked -= amount;
        Ok(self.calculate_rewards(user))
    }

    pub fn calculate_rewards(&self, user: &str) -> u64 {
        let stake = self.stakes.get(user).unwrap();
        (stake.amount as f64 * self.reward_rate) as u64
    }

    pub fn get_stake(&self, user: &str) -> u64 {
        self.stakes.get(user).map(|s| s.amount).unwrap_or(0)
    }
}
