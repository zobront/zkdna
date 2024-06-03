#![no_main]
sp1_zkvm::entrypoint!(main);

use dna_proof::{
    proof::DNAProof,
    merkle::{
        DNAMerkleTree,
        helpers::{bp_to_bits, get_root_from_proof},
    },
};

pub fn main() {
    // Read the input data passed to the zkVM.
    let data = sp1_zkvm::io::read<DNAProof>();

    // Verify the merkle proof that the leaf is valid.
    let root_from_proof = get_root_from_proof(&data.leaf, &data.proof);
    assert!(data.root == root_from_proof);

    // Prove the base pair within the leaf.
    let index_within_leaf = proof.index % 128;
    let byte = proof.leaf[index_within_leaf / 8];
    let bits = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    assert!(data.target == bits);

    // Commit to the target base pair, the index, and the root.
    sp1_zkvm::io::commit(&data.target);
    sp1_zkvm::io::commit(&data.root);
    sp1_zkvm::io::commit(&data.index);
}
