use rand::rngs::OsRng;
use sha2::Sha256;
use digest::Digest;

pub struct ZKProofCore;

impl ZKProofCore {
    pub fn generate_proof(secret: u64, public_input: u64) -> (Vec<u8>, Vec<u8>) {
        let mut rng = OsRng;
        let mut randomness = [0u8; 32];
        rng.fill(&mut randomness);

        let mut hasher = Sha256::new();
        hasher.update(secret.to_be_bytes());
        hasher.update(public_input.to_be_bytes());
        hasher.update(randomness);
        let proof = hasher.finalize().to_vec();

        (proof, randomness.to_vec())
    }

    pub fn verify_proof(proof: &[u8], public_input: u64, commitment: &[u8], randomness: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(commitment);
        hasher.update(public_input.to_be_bytes());
        hasher.update(randomness);
        let computed = hasher.finalize();
        computed.as_slice() == proof
    }

    pub fn create_commitment(secret: u64) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(secret.to_be_bytes());
        hasher.finalize().to_vec()
    }

    pub fn verify_balance_proof(balance: u64, min_balance: u64) -> bool {
        let commitment = Self::create_commitment(balance);
        let (proof, randomness) = Self::generate_proof(balance, min_balance);
        Self::verify_proof(&proof, min_balance, &commitment, &randomness)
    }
}
