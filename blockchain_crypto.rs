use rand::rngs::OsRng;
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey};

pub struct BlockchainCrypto;

impl BlockchainCrypto {
    pub fn generate_key_pair() -> (SigningKey, VerifyingKey) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = VerifyingKey::from(&signing_key);
        (signing_key, verifying_key)
    }

    pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
        let signature = signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    pub fn verify_signature(
        verifying_key: &VerifyingKey,
        message: &[u8],
        signature: &[u8],
    ) -> bool {
        let sig = match ed25519_dalek::Signature::from_bytes(signature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        verifying_key.verify(message, &sig).is_ok()
    }

    pub fn sha256_hash(data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn double_sha256(data: &[u8]) -> String {
        let first = Self::sha256_hash(data);
        Self::sha256_hash(first.as_bytes())
    }
}
