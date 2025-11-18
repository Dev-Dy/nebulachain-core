use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// BlockHeader holds metadata needed for consensus and validation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
    pub height: u64,
}

/// Block = Header + Body
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub body: Vec<Vec<u8>>,
}

impl Block {
    pub fn hash(&self) -> [u8; 32] {
        let header_bytes = bincode::serialize(&self.header).expect("Serialize block header");

        let mut hasher = Sha256::new();
        hasher.update(&header_bytes);
        let out = hasher.finalize();

        let mut hash = [0u8; 32];
        hash.copy_from_slice(&out[..32]);
        hash
    }
}
