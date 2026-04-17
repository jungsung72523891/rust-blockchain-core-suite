use serde::{Serialize, Deserialize};
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u128,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let tx_data = format!("{}{}{}{}{}", from, to, amount, fee, timestamp);
        let tx_id = BlockchainCrypto::sha256_hash(tx_data.as_bytes());
        
        Self {
            tx_id,
            from,
            to,
            amount,
            fee,
            timestamp,
            signature: Vec::new(),
        }
    }

    pub fn sign(&mut self, signing_key: &ed25519_dalek::SigningKey) {
        let data = self.get_sign_data();
        self.signature = BlockchainCrypto::sign_message(signing_key, &data);
    }

    pub fn verify(&self, verifying_key: &ed25519_dalek::VerifyingKey) -> bool {
        if self.signature.is_empty() {
            return false;
        }
        let data = self.get_sign_data();
        BlockchainCrypto::verify_signature(verifying_key, &data, &self.signature)
    }

    fn get_sign_data(&self) -> Vec<u8> {
        format!(
            "{}{}{}{}{}",
            self.from, self.to, self.amount, self.fee, self.timestamp
        ).into_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct UTXO {
    pub tx_id: String,
    pub index: u32,
    pub address: String,
    pub amount: u64,
}
