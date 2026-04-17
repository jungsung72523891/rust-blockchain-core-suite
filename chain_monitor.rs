use std::collections::VecDeque;
use std::time::{Instant, Duration};
use crate::basic_blockchain::Blockchain;

pub struct ChainMonitor {
    blockchain: Blockchain,
    block_times: VecDeque<Duration>,
    tps_records: VecDeque<u64>,
    start_time: Instant,
    max_records: usize,
}

impl ChainMonitor {
    pub fn new(blockchain: Blockchain, max_records: usize) -> Self {
        Self {
            blockchain,
            block_times: VecDeque::with_capacity(max_records),
            tps_records: VecDeque::with_capacity(max_records),
            start_time: Instant::now(),
            max_records,
        }
    }

    pub fn record_block_time(&mut self, duration: Duration) {
        if self.block_times.len() >= self.max_records {
            self.block_times.pop_front();
        }
        self.block_times.push_back(duration);
    }

    pub fn record_tps(&mut self, tps: u64) {
        if self.tps_records.len() >= self.max_records {
            self.tps_records.pop_front();
        }
        self.tps_records.push_back(tps);
    }

    pub fn avg_block_time(&self) -> Duration {
        if self.block_times.is_empty() {
            return Duration::ZERO;
        }
        let total: Duration = self.block_times.iter().sum();
        total / self.block_times.len() as u32
    }

    pub fn avg_tps(&self) -> u64 {
        if self.tps_records.is_empty() {
            return 0;
        }
        let total: u64 = self.tps_records.iter().sum();
        total / self.tps_records.len() as u64
    }

    pub fn chain_height(&self) -> u64 {
        self.blockchain.chain.len() as u64 - 1
    }

    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}
