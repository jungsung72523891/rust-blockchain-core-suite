use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleData {
    pub key: String,
    pub value: String,
    pub timestamp: u128,
    pub source: String,
    pub signature: Vec<u8>,
}

pub struct OracleCore {
    trusted_sources: Vec<String>,
    data_store: HashMap<String, OracleData>,
}

impl OracleCore {
    pub fn new(trusted_sources: Vec<String>) -> Self {
        Self {
            trusted_sources,
            data_store: HashMap::new(),
        }
    }

    pub fn submit_data(&mut self, mut data: OracleData) -> bool {
        if !self.trusted_sources.contains(&data.source) {
            return false;
        }

        data.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.data_store.insert(data.key.clone(), data);
        true
    }

    pub fn get_data(&self, key: &str) -> Option<&OracleData> {
        self.data_store.get(key)
    }

    pub fn is_data_fresh(&self, key: &str, max_age_ms: u128) -> bool {
        let data = match self.data_store.get(key) {
            Some(d) => d,
            None => return false,
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        now - data.timestamp <= max_age_ms
    }

    pub fn add_trusted_source(&mut self, source: String) {
        if !self.trusted_sources.contains(&source) {
            self.trusted_sources.push(source);
        }
    }
}
