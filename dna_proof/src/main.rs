mod merkle;
mod proof;

use merkle::{
    DNAMerkleTree,
    helpers::{bp_to_bits, bits_to_bp, get_root_from_proof},
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
    if proof.root == root_from_merkle_proof {
        println!("Merkle Proof is valid: 0x{}", hex::encode(proof.root));
    } else {
        println!("Merkle Proof is invalid.");
    }

    // Prove the base pair within the leaf.
    let index_within_leaf = proof.index % 128;
    let byte = proof.leaf[index_within_leaf / 8];
    let bits = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    if proof.target == bits {
        println!("Base Pair is valid: {}", bits_to_bp(proof.target));
    } else {
        println!("Base Pair is invalid.");
    }
}
