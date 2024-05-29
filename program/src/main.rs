//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use dna_merkle_lib::merkle::verify_proof;

pub fn main() {
    let leaf = sp1_zkvm::io::read<u32>();
    let proof = sp1_zkvm::io::read_vec();

    // todo: implement custom type for vector of vectors?
    let root = sp1_zkvm::io::read<u32>();
    sp1_zkvm::io::commit_slice(&root);

    let index = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit::<u32>(&index);

    let result = verify_proof(&root, &leaf, &proof, index as usize);

    sp1_zkvm::io::commit(&result);
}
