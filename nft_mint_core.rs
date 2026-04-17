use std::collections::HashMap;
use crate::blockchain_crypto::BlockchainCrypto;

#[derive(Debug, Clone)]
pub struct NFTToken {
    pub token_id: String,
    pub owner: String,
    pub metadata_uri: String,
    pub mint_time: u128,
}

pub struct NFTMintCore {
    tokens: HashMap<String, NFTToken>,
    owner_tokens: HashMap<String, Vec<String>>,
    total_supply: u64,
}

impl NFTMintCore {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            owner_tokens: HashMap::new(),
            total_supply: 0,
        }
    }

    pub fn mint(&mut self, owner: String, metadata_uri: String) -> String {
        let mint_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let raw_id = format!("{}{}{}", owner, metadata_uri, mint_time);
        let token_id = BlockchainCrypto::sha256_hash(raw_id.as_bytes());

        let token = NFTToken {
            token_id: token_id.clone(),
            owner: owner.clone(),
            metadata_uri,
            mint_time,
        };

        self.tokens.insert(token_id.clone(), token);
        self.owner_tokens.entry(owner).or_default().push(token_id.clone());
        self.total_supply += 1;
        token_id
    }

    pub fn transfer(&mut self, from: &str, to: &str, token_id: &str) -> bool {
        let token = match self.tokens.get_mut(token_id) {
            Some(t) => t,
            None => return false,
        };

        if token.owner != from {
            return false;
        }

        token.owner = to.to_string();
        
        if let Some(tokens) = self.owner_tokens.get_mut(from) {
            tokens.retain(|id| id != token_id);
        }
        self.owner_tokens.entry(to.to_string()).or_default().push(token_id.to_string());
        true
    }

    pub fn get_owner(&self, token_id: &str) -> Option<String> {
        self.tokens.get(token_id).map(|t| t.owner.clone())
    }

    pub fn get_user_tokens(&self, owner: &str) -> Vec<String> {
        self.owner_tokens.get(owner).cloned().unwrap_or_default()
    }
}
