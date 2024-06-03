mod merkle;
mod proof;

use merkle::{
    DNAMerkleTree,
    helpers::{bp_to_bits, get_root_from_proof},
};
use proof::DNAProof;

fn main() {
    // Goal: Prove that the base pair at index 52 is 'T'
    let index = 52;
    let target = bp_to_bits(&b'T');
    assert!(index % 2 == 0);

    // Generate a Merkle tree from the source data
    let file_path = "../static/genome.txt";
    let tree = DNAMerkleTree::new(file_path);

    // Generate the proof.
    let (leaf, merkle_proof) = tree.leaf_and_proof_from_gene_index(index);
    let proof = DNAProof::new(leaf, merkle_proof, tree.root(), index, target);

    // // ////////////////////////////////////
    // // //// ABOVE THIS LINE IS SCRIPT /////
    // // //// BELOW THIS LINE IS PROGRAM ////
    // // ////////////////////////////////////

    // // Verify the merkle proof that the leaf is valid.
    let root_from_merkle_proof = get_root_from_proof(&proof.leaf, &proof.merkle_proof);
    println!("Leaf is valid: {}", proof.root == root_from_merkle_proof);

    // Prove the base pair within the leaf.
    let index_within_leaf = proof.index % 128;
    let byte = proof.leaf[index_within_leaf / 8];
    let bits = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    println!("BP at Index: {}", proof.target == bits);
}
