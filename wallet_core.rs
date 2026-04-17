use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::{blockchain_crypto::BlockchainCrypto, transaction_core::Transaction};

pub struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let (signing_key, verifying_key) = BlockchainCrypto::generate_key_pair();
        let address = Self::generate_address(&verifying_key);
        
        Self {
            signing_key,
            verifying_key,
            address,
        }
    }

    fn generate_address(verifying_key: &VerifyingKey) -> String {
        let pub_key_bytes = verifying_key.to_bytes();
        let hash = BlockchainCrypto::sha256_hash(&pub_key_bytes);
        format!("0x{}", &hash[0..40])
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn public_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    pub fn create_transaction(&self, to: String, amount: u64, fee: u64) -> Transaction {
        let mut tx = Transaction::new(self.address.clone(), to, amount, fee);
        tx.sign(&self.signing_key);
        tx
    }

    pub fn sign_custom_data(&self, data: &[u8]) -> Vec<u8> {
        BlockchainCrypto::sign_message(&self.signing_key, data)
    }
}
