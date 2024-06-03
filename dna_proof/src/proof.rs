pub use tiny_merkle::{
    MerkleTree,
    proof::MerkleProof
};
use serde::{Serialize, Deserialize};
use crate::merkle::keccak::KeccakHasher;

#[derive(Serialize, Deserialize)]
pub struct DNAProof {
    pub leaf: [u8; 32],
    // @todo this can be more efficiently as just a vec of hashes, as position can be derived based on index
    pub merkle_proof: MerkleProof<KeccakHasher>,
    pub root: [u8; 32],
    pub index: usize,
    pub target: u8,
}

impl DNAProof {
    pub fn new(leaf: [u8; 32], merkle_proof: MerkleProof<KeccakHasher>, root: [u8; 32], index: usize, target: u8) -> Self {
        Self {
            leaf,
            merkle_proof,
            root,
            index,
            target,
        }
    }
}
