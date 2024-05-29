use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn bp_to_bits(bp: u8) -> u128 {
    match bp {
        b'A' => 0b00,
        b'C' => 0b01,
        b'G' => 0b10,
        b'T' => 0b11,
        _ => panic!("Invalid DNA base"),
    }
}

/// Reads the DNA sequence from a file located at the given path.
pub fn read_dna_sequence(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut sequence = String::new();
    file.read_to_string(&mut sequence)?;
    Ok(sequence)
}

/// Encodes a DNA sequence string into a vector of 256-bit words.
pub fn encode_dna_sequence(dna_sequence: &str) -> Vec<u128> {
    let mut encoded_words = Vec::new();

    // Filter out any whitespace characters
    let cleaned_sequence: String = dna_sequence.chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    // Iterate over the cleaned DNA sequence in chunks of 128 characters
    for chunk in cleaned_sequence.as_bytes().chunks(128) {
        let mut encoded_word: u128 = 0;

        for &base in chunk {
            let value = bp_to_bits(base);
            encoded_word = (encoded_word << 2) | value;
        }

        encoded_words.push(encoded_word);
    }

    encoded_words
}
