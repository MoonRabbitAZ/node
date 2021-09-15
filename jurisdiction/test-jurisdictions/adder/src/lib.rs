

//! Basic parachain that adds a number as part of its state.

#![no_std]

#![cfg_attr(not(feature = "std"), feature(core_intrinsics, lang_items, core_panic_info, alloc_error_handler))]

use moonrabbit_scale_codec::{Encode, Decode};
use tiny_keccak::{Hasher as _, Keccak};

#[cfg(not(feature = "std"))]
mod wasm_validation;

#[cfg(not(feature = "std"))]
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

fn keccak256(input: &[u8]) -> [u8; 32] {
	let mut out = [0u8; 32];
	let mut keccak256 = Keccak::v256();
	keccak256.update(input);
	keccak256.finalize(&mut out);
	out
}

/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect("Development wasm binary is not available. Testing is only \
						supported with the flag disabled.")
}

/// Head data for this parachain.
#[derive(Default, Clone, Hash, Eq, PartialEq, Encode, Decode, Debug)]
pub struct HeadData {
	/// Block number
	pub number: u64,
	/// parent block keccak256
	pub parent_hash: [u8; 32],
	/// hash of post-execution state.
	pub post_state: [u8; 32],
}

impl HeadData {
	pub fn hash(&self) -> [u8; 32] {
		keccak256(&self.encode())
	}
}

/// Block data for this parachain.
#[derive(Default, Clone, Encode, Decode, Debug)]
pub struct BlockData {
	/// State to begin from.
	pub state: u64,
	/// Amount to add (wrapping)
	pub add: u64,
}

pub fn hash_state(state: u64) -> [u8; 32] {
	keccak256(state.encode().as_slice())
}

/// Start state mismatched with parent header's state hash.
#[derive(Debug)]
pub struct StateMismatch;

/// Execute a block body on top of given parent head, producing new parent head
/// if valid.
pub fn execute(
	parent_hash: [u8; 32],
	parent_head: HeadData,
	block_data: &BlockData,
) -> Result<HeadData, StateMismatch> {
	assert_eq!(parent_hash, parent_head.hash());

	if hash_state(block_data.state) != parent_head.post_state {
		return Err(StateMismatch);
	}

	let new_state = block_data.state.wrapping_add(block_data.add);

	Ok(HeadData {
		number: parent_head.number + 1,
		parent_hash,
		post_state: hash_state(new_state),
	})
}
