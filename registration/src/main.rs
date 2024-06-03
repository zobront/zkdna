use std::io;
use hex;

use dna_proof::merkle::DNAMerkleTree;

fn main() -> io::Result<()> {
    // Define the path to the genome file
    let file_path = "../static/genome.txt";

    // Read the DNA sequence from the file into a DNA Merkle Tree
    let tree = DNAMerkleTree::new(file_path);

    // Compute the Merkle root from the encoded words
    let root = tree.root();

    // Print the Merkle root
    println!("Merkle Root: 0x{}", hex::encode(root));

    Ok(())
}
