use ed25519_dalek::{
    Signer, Verifier, SigningKey, VerifyingKey, 
    Signature, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH
};
use rand::rngs::OsRng;
use hex::{encode, decode};

pub struct Ed25519Crypto;

impl Ed25519Crypto {
    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = VerifyingKey::from(&signing_key);
        
        (signing_key.to_bytes().to_vec(), verifying_key.to_bytes().to_vec())
    }

    pub fn sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, String> {
        if secret_key.len() != SECRET_KEY_LENGTH {
            return Err("Invalid secret key length".to_string());
        }
        
        let sk_bytes: [u8; SECRET_KEY_LENGTH] = secret_key.try_into().map_err(|_| "Invalid key")?;
        let signing_key = SigningKey::from_bytes(&sk_bytes);
        let signature = signing_key.sign(message);
        
        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, String> {
        if public_key.len() != PUBLIC_KEY_LENGTH || signature.len() != SIGNATURE_LENGTH {
            return Err("Invalid key or signature length".to_string());
        }

        let pk_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key.try_into().map_err(|_| "Invalid key")?;
        let sig_bytes: [u8; SIGNATURE_LENGTH] = signature.try_into().map_err(|_| "Invalid signature")?;
        
        let verifying_key = VerifyingKey::from_bytes(&pk_bytes).map_err(|e| e.to_string())?;
        let signature = Signature::from_bytes(&sig_bytes);
        
        Ok(verifying_key.verify(message, &signature).is_ok())
    }

    pub fn pubkey_to_address(public_key: &[u8]) -> String {
        let hash = sha256::digest(public_key);
        format!("0x{}", &hash[0..40])
    }
}
