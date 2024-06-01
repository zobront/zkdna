use tiny_keccak::{Hasher, Keccak};
use tiny_merkle;

#[derive(Clone, Debug)]
pub struct KeccakHasher;

impl tiny_merkle::Hasher for KeccakHasher {
	type Hash = [u8; 32];

	fn hash(value: &[u8]) -> Self::Hash {
		keccak256(value)
	}
}

fn keccak256(data: &[u8]) -> [u8; 32] {
	let mut hasher = Keccak::v256();
	let mut hash = [0_u8; 32];
	hasher.update(data);
	hasher.finalize(&mut hash);
	hash
}
