use std::collections::HashMap;
use crate::basic_blockchain::{Block, Blockchain};

pub struct ForkHandler {
    forks: HashMap<u64, Vec<Block>>,
    main_chain: Blockchain,
}

impl ForkHandler {
    pub fn new(main_chain: Blockchain) -> Self {
        Self {
            forks: HashMap::new(),
            main_chain,
        }
    }

    pub fn add_fork_block(&mut self, block: Block) {
        self.forks.entry(block.index).or_default().push(block);
    }

    pub fn find_longest_chain(&self) -> Vec<Block> {
        let mut longest = self.main_chain.chain.clone();
        for fork in self.forks.values() {
            if fork.len() > longest.len() {
                longest = fork.clone();
            }
        }
        longest
    }

    pub fn resolve_forks(&mut self) -> bool {
        let longest = self.find_longest_chain();
        if longest.len() > self.main_chain.chain.len() {
            self.main_chain.chain = longest;
            self.forks.clear();
            true
        } else {
            false
        }
    }

    pub fn is_on_main_chain(&self, block: &Block) -> bool {
        self.main_chain.chain.contains(block)
    }

    pub fn get_fork_count(&self) -> usize {
        self.forks.values().map(|v| v.len()).sum()
    }

    pub fn main_chain_height(&self) -> u64 {
        self.main_chain.chain.len() as u64 - 1
    }
}
