use std::path::Path;
use dna_merkle_lib::{
    dna_encoder::{read_dna_sequence, encode_dna_sequence, bp_to_bits},
    merkle::{compute_merkle_root, generate_merkle_proof, verify_proof},
};


fn main() {
    // Goal: Prove that the base pair at index 52 is 'A'
    let index = 52;
    let target = bp_to_bits(&b'A');
    assert!(index % 2 == 0);

    // Define the path to the genome file
    let file_path = Path::new("../static/genome.txt");

    // Read the DNA sequence from the file
    let dna_sequence = read_dna_sequence(file_path).unwrap();

    // Encode the DNA sequence into 256-bit words
    let leaves = encode_dna_sequence(&dna_sequence);

    // Compute the Merkle root of the DNA sequence
    let merkle_root = compute_merkle_root(&leaves);

    // Prove the leaf.
    let leaf_index = index / 128;
    let target_leaf = leaves[leaf_index];
    let overall_index = leaf_index + leaves.len() + leaves.len() % 2;
    let merkle_proof = generate_merkle_proof(&leaves, overall_index);
    let result = verify_proof(merkle_root, target_leaf, &merkle_proof, overall_index);
    println!("Leaf is valid: {}", result);

    // Prove the base pair within the leaf.
    let index_within_leaf = index % 128;
    let byte = target_leaf[index_within_leaf / 8];
    let bits = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    println!("BP at Index: {}", target == bits);
}
