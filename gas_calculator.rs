use crate::transaction_core::Transaction;

pub struct GasCalculator {
    base_fee: u64,
    max_priority_fee: u64,
    gas_limit_multiplier: f64,
    congestion_factor: f64,
}

impl GasCalculator {
    pub fn new(base_fee: u64, max_priority_fee: u64) -> Self {
        Self {
            base_fee,
            max_priority_fee,
            gas_limit_multiplier: 1.2,
            congestion_factor: 1.0,
        }
    }

    pub fn calculate_tx_gas(&self, tx: &Transaction) -> u64 {
        let base_cost = self.base_fee;
        let size_cost = std::mem::size_of_val(tx) as u64 / 100;
        let priority_cost = self.max_priority_fee;
        
        ((base_cost + size_cost + priority_cost) as f64 * self.congestion_factor) as u64
    }

    pub fn calculate_gas_limit(&self, base_gas: u64) -> u64 {
        (base_gas as f64 * self.gas_limit_multiplier) as u64
    }

    pub fn update_congestion(&mut self, mempool_size: usize, max_mempool: usize) {
        let ratio = mempool_size as f64 / max_mempool as f64;
        self.congestion_factor = 1.0 + (ratio * 2.0);
    }

    pub fn effective_gas_price(&self, tx: &Transaction) -> u64 {
        let gas = self.calculate_tx_gas(tx);
        gas.max(tx.fee)
    }
}
