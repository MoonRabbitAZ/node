

//! Utilities for writing parachain WASM.

/// Load the validation params from memory when implementing a Rust parachain.
///
/// Offset and length must have been provided by the validation
/// function's entry point.
#[cfg(not(feature = "std"))]
pub unsafe fn load_params(params: *const u8, len: usize)
	-> crate::primitives::ValidationParams
{
	let mut slice = sp_std::slice::from_raw_parts(params, len);

	moonrabbit_scale_codec::Decode::decode(&mut slice).expect("Invalid input data")
}

/// Allocate the validation result in memory, getting the return-pointer back.
///
/// As described in the crate docs, this is a pointer to the appended length
/// of the vector.
#[cfg(not(feature = "std"))]
pub fn write_result(result: &crate::primitives::ValidationResult) -> u64 {
	sp_core::to_substrate_wasm_fn_return_value(&result)
}
