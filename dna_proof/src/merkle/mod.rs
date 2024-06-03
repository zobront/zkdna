pub mod keccak;
pub mod encoder;
pub mod helpers;

use std::path::Path;
use tiny_merkle::{
    MerkleTree,
    proof::MerkleProof
};
use keccak::KeccakHasher;
use encoder::{read_dna_sequence, encode_dna_sequence};

pub struct DNAMerkleTree {
    leaves: Vec<[u8; 32]>,
    tree: MerkleTree<KeccakHasher>,
}

impl DNAMerkleTree {
    pub fn new(genome_path: &str) -> Self {
        let file_path = Path::new(genome_path);
        let dna_sequence = read_dna_sequence(file_path).unwrap();

        let leaves = encode_dna_sequence(&dna_sequence);
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
        self.tree.make_proof(index)
    }

    pub fn leaf_and_proof_from_gene_index(&self, index: usize) -> ([u8; 32], MerkleProof<KeccakHasher>) {
        let leaf_index = index / 128;
        (self.leaves[leaf_index], self.proof(leaf_index))
    }
}
