use serde::Serialize;
use sha2::{Sha256, Digest};

#[derive(Clone, Serialize, Debug)]
pub struct CommitmentInfo([u64; 2]);

impl CommitmentInfo {
    pub fn new(c0: u64, c1: u64) -> Self {
        CommitmentInfo([c0, c1])
    }

    pub fn get_commitment(&self) -> [u64; 2] {
        self.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedInfo {
    pub seed: u64,
    pub commitment: CommitmentInfo,
}

impl SeedInfo {
    // Admin generates a random seed and its commitment
    pub fn generate_seed_commitment() -> Self {
        // In production, use a secure random number generator
        let seed = 12345u64; // Example fixed seed for demo
        
        // Calculate commitment = hash(seed)
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        let result = hasher.finalize();
        
        // Convert first 16 bytes of hash into two u64 values
        let c0 = u64::from_le_bytes(result[0..8].try_into().unwrap());
        let c1 = u64::from_le_bytes(result[8..16].try_into().unwrap());
        
        SeedInfo {
            seed,
            commitment: CommitmentInfo::new(c0, c1)
        }
    }

    // Verify that a revealed seed matches its commitment
    pub fn verify_seed(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.seed.to_le_bytes());
        let result = hasher.finalize();
        
        let c0 = u64::from_le_bytes(result[0..8].try_into().unwrap());
        let c1 = u64::from_le_bytes(result[8..16].try_into().unwrap());
        
        let commitment = self.commitment.get_commitment();
        c0 == commitment[0] && c1 == commitment[1]
        
    }

    // Generate final random number using seed and player signature
    pub fn generate_random(&self, player_signature: u64) -> u64 {
        self.seed ^ player_signature
    }

    // 合并 reveal、verify 和 random generation 为一个函数
    pub fn reveal_verify_and_generate_random(&self, player_signature: u64) -> Result<u64, &'static str> {
        // 1. Verify the seed matches commitment
        if !self.verify_seed() {
            return Err("Invalid seed: commitment verification failed");
        }

        // 2. Generate final random number
        let random = self.seed ^ player_signature;

        Ok(random)
    }
}

