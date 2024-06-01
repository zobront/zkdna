pub use tiny_merkle::{
    MerkleTree,
    proof::MerkleProof
};
use serde::{Serialize, Deserialize};
use crate::keccak::KeccakHasher;

#[derive(Serialize, Deserialize)]
struct DNAProofInputData {
    leaf: [u8; 32],
    // @todo merkle lib doesn't have serde support, fork merkle tree lib and fix it
    proof: MerkleProof<KeccakHasher>,
    root: [u8; 32],
    index: u32,
}

impl DNAProofInputData {
    fn new(leaf: [u8; 32], proof: MerkleProof<KeccakHasher>, root: [u8; 32], index: u32) -> Self {
        Self {
            leaf,
            proof,
            root,
            index,
        }
    }

    pub fn verify_proof(&self) -> bool {
        let tree = MerkleTree::<KeccakHasher>::from_leaves(vec![], None);
        tree.verify(self.leaf, self.root, &self.proof)
    }
}
