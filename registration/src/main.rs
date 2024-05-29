use std::path::Path;
use std::io;
use hex;

use dna_merkle_lib::{
    dna_encoder::{encode_dna_sequence, read_dna_sequence},
    merkle::compute_merkle_root,
};

fn main() -> io::Result<()> {
    // Define the path to the genome file
    let file_path = Path::new("../static/genome.txt");

    // Read the DNA sequence from the file
    let dna_sequence = read_dna_sequence(file_path)?;

    // Encode the DNA sequence into 256-bit words
    let leaves = encode_dna_sequence(&dna_sequence);

    // Compute the Merkle root from the encoded words
    let merkle_root = compute_merkle_root(&leaves);

    // Print the Merkle root
    println!("Merkle Root: 0x{}", hex::encode(merkle_root));

    Ok(())
}
