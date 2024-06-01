use std::path::Path;
use dna_merkle_lib::{
    dna_encoder::{read_dna_sequence, encode_dna_sequence, bp_to_bits},
    merkle::{compute_merkle_root, generate_merkle_proof, verify_proof},
};
use hex;

fn main() {
    // Goal: Prove that the base pair at index 52 is 'T'
    let index = 52;
    let target = bp_to_bits(&b'T');
    assert!(index % 2 == 0);

    // Define the path to the genome file
    let file_path = Path::new("../static/genome.txt");

    // Read the DNA sequence from the file
    let dna_sequence = read_dna_sequence(file_path).unwrap();

    // Encode the DNA sequence into 256-bit words
    let leaves = encode_dna_sequence(&dna_sequence);

    // Compute the Merkle root of the DNA sequence
    let merkle_root = compute_merkle_root(&leaves);

    // Generate the proof.
    let leaf_index = index / 128;
    let target_leaf = leaves[leaf_index];
    let merkle_proof = generate_merkle_proof(&leaves, leaf_index);

    // ////////////////////////////////////
    // //// ABOVE THIS LINE IS SCRIPT /////
    // //// BELOW THIS LINE IS PROGRAM ////
    // ////////////////////////////////////

    // // Verify the merkle proof that the leaf is valid.
    let valid_proof = verify_proof(&target_leaf, &merkle_root, &merkle_proof);
    println!("Leaf is valid: {}", valid_proof);

    // @todo prove it's on the correct side all the way up to prove index?

    // Prove the base pair within the leaf.
    let index_within_leaf = index % 128;
    let byte = target_leaf[index_within_leaf / 8];
    let bits = (byte >> (6 - index_within_leaf % 8)) & 0b11;
    println!("BP at Index: {}", target == bits);
}
