use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::basic_blockchain::{Block, Blockchain};
use crate::p2p_transport::P2PTransport;

pub struct BlockSyncer {
    blockchain: Arc<Mutex<Blockchain>>,
    p2p: Arc<Mutex<P2PTransport>>,
    sync_queue: VecDeque<u64>,
    batch_size: u64,
}

impl BlockSyncer {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>, p2p: Arc<Mutex<P2PTransport>>) -> Self {
        Self {
            blockchain,
            p2p,
            sync_queue: VecDeque::new(),
            batch_size: 10,
        }
    }

    pub fn start_sync(&mut self, target_height: u64) {
        let current = self.blockchain.lock().unwrap().chain.len() as u64;
        for i in current..target_height {
            self.sync_queue.push_back(i);
        }
    }

    pub fn sync_step(&mut self) -> Option<Block> {
        let index = self.sync_queue.pop_front()?;
        let request = format!("GET_BLOCK:{}", index);
        
        let _ = self.p2p.lock().unwrap().broadcast(request.as_bytes());
        None
    }

    pub fn receive_block(&mut self, block: Block) -> bool {
        let mut chain = self.blockchain.lock().unwrap();
        let last = chain.chain.last().unwrap();
        
        if block.index == last.index + 1 && block.previous_hash == last.hash {
            chain.chain.push(block);
            true
        } else {
            false
        }
    }

    pub fn is_syncing(&self) -> bool {
        !self.sync_queue.is_empty()
    }

    pub fn remaining_blocks(&self) -> usize {
        self.sync_queue.len()
    }
}
