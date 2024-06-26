use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use super::helpers::bp_to_bits;

/// Reads the DNA sequence from a file located at the given path.
pub fn read_dna_sequence(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut genotypes = String::new();

    for line in reader.lines() {
        let line = line?;
        // Skip header lines or comments
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            println!("Invalid line: {}", line);
            continue;
        }

        // Extract the genotype
        let genotype = parts[3];

        // Check if the genotype is a valid two-letter string containing only A, G, T, or C
        if genotype.len() == 2 && genotype.chars().all(|c| matches!(c, 'A' | 'G' | 'T' | 'C')) {
            genotypes.push_str(genotype);
        }
    }

    Ok(genotypes)
}

/// Encodes a DNA sequence string into a vector of 256-bit words represented as [u8; 32].
pub fn encode_dna_sequence(dna_sequence: &str) -> Vec<[u8; 32]> {
    let mut encoded_words = Vec::new();

    // Filter out any whitespace characters
    let cleaned_sequence: String = dna_sequence.chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    // Iterate over the cleaned DNA sequence in chunks of 128 characters
    for chunk in cleaned_sequence.as_bytes().chunks(128) {
        let mut encoded_word = [0u8; 32];
        let mut bit_index = 0;

        for &base in chunk {
            let value = bp_to_bits(&base);

            let byte_index = bit_index / 8;
            let bit_offset = bit_index % 8;
            encoded_word[byte_index] |= value << (6 - bit_offset);
            bit_index += 2;
        }

        encoded_words.push(encoded_word);
    }

    encoded_words
}
