use crate::keccak_hasher::KeccakHasher;
use tiny_merkle::{
    MerkleTree,
    proof::MerkleProof
};

pub struct DNAMerkleTree {
    leaves: Vec<[u8; 32]>,
    tree: MerkleTree<KeccakHasher>,
}

impl DNAMerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>) -> Self {
        let tree = MerkleTree::<KeccakHasher>::from_leaves(leaves.clone(), None);
        DNAMerkleTree {
            leaves,
            tree,
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.tree.root()
    }

    pub fn proof(&self, index: usize) -> MerkleProof<KeccakHasher> {
        self.tree.proof(&self.leaves[index]).unwrap()
    }
}
