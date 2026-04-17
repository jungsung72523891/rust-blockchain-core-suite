use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng}
};
use rand::Rng;

pub struct NetworkEncryptor {
    cipher: Aes256Gcm,
}

impl NetworkEncryptor {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill(&mut key);
        key
    }

    fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        OsRng.fill(&mut nonce);
        nonce
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let nonce = Self::generate_nonce();
        let nonce_obj = Nonce::from_slice(&nonce);
        
        let mut encrypted = self.cipher.encrypt(nonce_obj, data).unwrap_or_default();
        let mut result = Vec::with_capacity(nonce.len() + encrypted.len());
        
        result.extend_from_slice(&nonce);
        result.append(&mut encrypted);
        result
    }

    pub fn decrypt(&self, data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 12 {
            return None;
        }
        let (nonce, ciphertext) = data.split_at(12);
        let nonce_obj = Nonce::from_slice(nonce);
        
        self.cipher.decrypt(nonce_obj, ciphertext).ok()
    }
}
