#![no_main]
sp1_zkvm::entrypoint!(main);

use dna_merkle_lib::merkle::{verify_proof, MerkleProof};

pub fn main() {
    let data = sp1_zkvm::io::read<DNAProofInputData>();

    // Verify the merkle proof that the leaf is valid.
    let valid = verify_proof(&data.leaf, &data.root, &data.proof);
    sp1_zkvm::io::commit(&valid);

    // Prove the base pair within the leaf.
    let index_within_leaf = index % 128;
    let byte = data.leaf[index_within_leaf / 8];
    let bits: u8 = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    sp1_zkvm::io::commit(&bits);
}
