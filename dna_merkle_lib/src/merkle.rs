use tiny_keccak::{Hasher, Keccak};

/// Computes the Keccak256 hash of the given data.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

/// Computes the Merkle root of the given encoded words.
pub fn compute_merkle_root(leaves: &[u128]) -> [u8; 32] {
    let mut current_level: Vec<[u8; 32]> = leaves.iter()
        .map(|&word| keccak256(&word.to_le_bytes()))
        .collect();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            let combined = if chunk.len() == 2 {
                let mut combined_data = [0u8; 64];
                combined_data[..32].copy_from_slice(&chunk[0]);
                combined_data[32..].copy_from_slice(&chunk[1]);
                combined_data
            } else {
                let mut combined_data = [0u8; 64];
                combined_data[..32].copy_from_slice(&chunk[0]);
                combined_data[32..].copy_from_slice(&chunk[0]);
                combined_data
            };

            let parent_hash = keccak256(&combined);
            next_level.push(parent_hash);
        }

        current_level = next_level;
    }

    current_level[0]
}

/// Generates a Merkle proof for the given index.
pub fn generate_merkle_proof(leaves: &[u128], index: usize) -> Vec<[u8; 32]> {
    let mut proof = Vec::new();
    let mut current_index = index;
    let mut current_level: Vec<[u8; 32]> = leaves.iter()
        .map(|&word| keccak256(&word.to_le_bytes()))
        .collect();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            let combined = if chunk.len() == 2 {
                let mut combined_data = [0u8; 64];
                combined_data[..32].copy_from_slice(&chunk[0]);
                combined_data[32..].copy_from_slice(&chunk[1]);
                combined_data
            } else {
                let mut combined_data = [0u8; 64];
                combined_data[..32].copy_from_slice(&chunk[0]);
                combined_data[32..].copy_from_slice(&chunk[0]);
                combined_data
            };

            let parent_hash = keccak256(&combined);
            next_level.push(parent_hash);

            if current_index % 2 == 0 && current_index + 1 < current_level.len() {
                proof.push(current_level[current_index + 1]);
            } else if current_index % 2 == 1 {
                proof.push(current_level[current_index - 1]);
            }

            current_index /= 2;
        }

        current_level = next_level;
    }

    proof
}

/// Verifies a Merkle proof.
pub fn verify_proof(root: [u8; 32], leaf: [u8; 32], proof: &[[u8; 32]], index: usize) -> bool {
    let mut computed_hash = leaf;
    let mut current_index = index;

    for sibling in proof {
        let combined = if current_index % 2 == 0 {
            let mut combined_data = [0u8; 64];
            combined_data[..32].copy_from_slice(&computed_hash);
            combined_data[32..].copy_from_slice(sibling);
            combined_data
        } else {
            let mut combined_data = [0u8; 64];
            combined_data[..32].copy_from_slice(sibling);
            combined_data[32..].copy_from_slice(&computed_hash);
            combined_data
        };

        computed_hash = keccak256(&combined);
        current_index /= 2;
    }

    computed_hash == root
}