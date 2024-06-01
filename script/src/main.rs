//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use std::path::Path;
use dna_merkle_lib::{
    dna_encoder::{read_dna_sequence, encode_dna_sequence, bp_to_bits},
    merkle::{compute_merkle_root, generate_merkle_proof},
};
use program::DNAProofInputData;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

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

    // Input data.
    let input_data = DNAProofInputData {
        leaf: target_leaf,
        proof: merkle_proof,
        root: merkle_root,
        index: index as u32,
    };

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(input_data);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");

    // Read output.
    let valid_proof = proof.public_values.read::<bool>();
    let bit_at_index = proof.public_values.read::<u8>();

    println!("root_public: {}", valid_proof);
    println!("index_public: {}", bit_at_index);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    assert!(valid_proof);
    assert!(bit_at_index == target);

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
