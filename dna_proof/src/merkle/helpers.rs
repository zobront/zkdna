use super::keccak::KeccakHasher;
use tiny_merkle::{
    MerkleTree,
    proof::MerkleProof
};

pub fn bp_to_bits(bp: &u8) -> u8 {
    match bp {
        b'A' => 0b00,
        b'C' => 0b01,
        b'G' => 0b10,
        b'T' => 0b11,
        _ => panic!("Invalid DNA base"),
    }
}

pub fn bits_to_bp(bits: u8) -> char {
    match bits {
        0b00 => 'A',
        0b01 => 'C',
        0b10 => 'G',
        0b11 => 'T',
        _ => panic!("Invalid DNA bits"),
    }
}

pub fn get_root_from_proof(leaf: &[u8; 32], proof: &MerkleProof<KeccakHasher>) -> [u8; 32] {
    MerkleTree::<KeccakHasher>::get_root_from_proof_unsorted(leaf, proof).try_into().unwrap()
}
