use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    pub event_id: String,
    pub contract_address: String,
    pub event_name: String,
    pub data: Vec<String>,
    pub block_height: u64,
    pub tx_id: String,
}

pub struct ContractEventManager {
    events: HashMap<String, Vec<ContractEvent>>,
    by_tx_id: HashMap<String, ContractEvent>,
}

impl ContractEventManager {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            by_tx_id: HashMap::new(),
        }
    }

    pub fn emit_event(
        &mut self,
        contract_address: String,
        event_name: String,
        data: Vec<String>,
        block_height: u64,
        tx_id: String,
    ) {
        let raw_id = format!("{}{}{}{}", contract_address, tx_id, block_height, event_name);
        let event_id = BlockchainCrypto::sha256_hash(raw_id.as_bytes());

        let event = ContractEvent {
            event_id: event_id.clone(),
            contract_address: contract_address.clone(),
            event_name,
            data,
            block_height,
            tx_id: tx_id.clone(),
        };

        self.events.entry(contract_address).or_default().push(event.clone());
        self.by_tx_id.insert(tx_id, event);
    }

    pub fn get_contract_events(&self, contract_address: &str) -> Vec<&ContractEvent> {
        self.events
            .get(contract_address)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    pub fn get_event_by_tx(&self, tx_id: &str) -> Option<&ContractEvent> {
        self.by_tx_id.get(tx_id)
    }

    pub fn filter_by_name(&self, contract_address: &str, event_name: &str) -> Vec<&ContractEvent> {
        self.get_contract_events(contract_address)
            .into_iter()
            .filter(|e| e.event_name == event_name)
            .collect()
    }
}
