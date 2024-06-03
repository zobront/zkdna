//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};

use dna_proof::{
    merkle::{
        DNAMerkleTree,
        helpers::bp_to_bits,
    },
    proof::DNAProof,
};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// Set mode to determine whether to prove or execute the program.
const PROVE: bool = false;

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
    let proof_data = DNAProof::new(leaf, merkle_proof, tree.root(), index, target);

    // Write the proof to the input data.
    let mut stdin = SP1Stdin::new();
    stdin.write(&proof_data);

    // Create client.
    let client = ProverClient::new();

    // Create a proof and verify it to ensure it worked.
    if PROVE {
        // Generate proof.
        let (pk, vk) = client.setup(ELF);
        let mut proof = client.prove(&pk, stdin).expect("proving failed");

        // Read & print the public values from the proof.
        let target = proof.public_values.read::<u8>();
        let root = proof.public_values.read::<[u8; 32]>();
        let index = proof.public_values.read::<usize>();

        println!("target: {}", target);
        println!("root: {}", hex::encode(root));
        println!("index: {}", index);

        // Verify proof.
        client.verify(&proof, &vk).expect("verification failed");

        // Save proof.
        proof
            .save("proof-with-io.json")
            .expect("saving proof failed");

        println!("successfully generated and verified proof for the program!")

    // Execute the program to simulate proof verification.
    } else {
        // Execute the program.
        let mut public_values = client.execute(ELF, stdin).expect("execution failed");

        // Read & print the public values from the proof.
        let target = public_values.read::<u8>();
        let root = public_values.read::<[u8; 32]>();
        let index = public_values.read::<usize>();

        println!("target: {}", target);
        println!("root: {}", hex::encode(root));
        println!("index: {}", index);
    }
}
