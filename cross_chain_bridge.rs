use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainTx {
    pub tx_id: String,
    pub source_chain: String,
    pub target_chain: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub status: CrossChainStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CrossChainStatus {
    Pending,
    Confirmed,
    Failed,
    Completed,
}

pub struct CrossChainBridge {
    chain_id: String,
    trusted_relayers: Vec<String>,
    pending_txs: HashMap<String, CrossChainTx>,
}

impl CrossChainBridge {
    pub fn new(chain_id: String, relayers: Vec<String>) -> Self {
        Self {
            chain_id,
            trusted_relayers: relayers,
            pending_txs: HashMap::new(),
        }
    }

    pub fn lock_asset(&mut self, sender: String, target_chain: String, receiver: String, amount: u64) -> String {
        let tx_data = format!("{}{}{}{}{}", sender, target_chain, receiver, amount, std::time::SystemTime::now().elapsed().unwrap().as_millis());
        let tx_id = BlockchainCrypto::sha256_hash(tx_data.as_bytes());
        
        let tx = CrossChainTx {
            tx_id: tx_id.clone(),
            source_chain: self.chain_id.clone(),
            target_chain,
            sender,
            receiver,
            amount,
            signature: Vec::new(),
            status: CrossChainStatus::Pending,
        };
        
        self.pending_txs.insert(tx_id.clone(), tx);
        tx_id
    }

    pub fn verify_relayer(&self, relayer: &str) -> bool {
        self.trusted_relayers.contains(&relayer.to_string())
    }

    pub fn confirm_transaction(&mut self, tx_id: &str) -> bool {
        if let Some(tx) = self.pending_txs.get_mut(tx_id) {
            tx.status = CrossChainStatus::Confirmed;
            true
        } else {
            false
        }
    }

    pub fn complete_transaction(&mut self, tx_id: &str) -> bool {
        if let Some(tx) = self.pending_txs.get_mut(tx_id) {
            tx.status = CrossChainStatus::Completed;
            true
        } else {
            false
        }
    }
}
