//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use std::path::Path;
use dna_merkle_lib::{
    dna_encoder::{read_dna_sequence, encode_dna_sequence, bp_to_bits},
    merkle::{compute_merkle_root, generate_merkle_proof},
};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Goal: Prove that the base pair at index 52 is 'C'
    let index = 52;
    let target = bp_to_bits(b'C');

    // Define the path to the genome file
    let file_path = Path::new("../static/genome.txt");

    // Read the DNA sequence from the file
    let dna_sequence = read_dna_sequence(file_path).unwrap();

    // Encode the DNA sequence into 256-bit words
    let leaves = encode_dna_sequence(&dna_sequence);

    // Generate a Merkle proof for the target base pair
    let leaf_index = index / 128;
    let merkle_proof = generate_merkle_proof(&leaves, leaf_index);

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(&leaves[leaf_index]);
    stdin.write_slice(&merkle_proof.iter().flat_map(|array| array.iter().copied()).collect());
    stdin.write(&compute_merkle_root(&leaves));
    stdin.write(&index);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");

    // Read output.
    let root_public = proof.public_values.read::<u128>();
    let index_public = proof.public_values.read::<u128>();
    println!("root_public: {}", root_public);
    println!("index_public: {}", index_public);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
