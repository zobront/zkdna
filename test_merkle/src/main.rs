use std::path::Path;
use dna_merkle_lib::{
    dna_encoder::{read_dna_sequence, encode_dna_sequence, bp_to_bits},
    merkle::{compute_merkle_root, generate_merkle_proof, verify_proof},
};


fn main() {
    // Goal: Prove that the base pair at index 52 is 'C'
    let index = 52;
    let target = bp_to_bits(&b'C');

    // Define the path to the genome file
    let file_path = Path::new("../static/genome.txt");

    // Read the DNA sequence from the file
    let dna_sequence = read_dna_sequence(file_path).unwrap();

    // Encode the DNA sequence into 256-bit words
    let leaves = encode_dna_sequence(&dna_sequence);

    // Compute the Merkle root of the DNA sequence
    let merkle_root = compute_merkle_root(&leaves);

    // Generate a Merkle proof for the target base pair
    let leaf_index = index / 128;
    let merkle_proof = generate_merkle_proof(&leaves, 4);

    // Confirm if the proof is valid.
    let result = verify_proof(merkle_root, leaves[leaf_index], &merkle_proof, 4);
    println!("Proof is valid: {}", result);
}
