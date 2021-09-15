

//! Basic parachain that executes forever.

#![no_std]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics, lang_items, core_panic_info, alloc_error_handler))]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(feature = "std")]
/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect("Development wasm binary is not available. Testing is only \
						supported with the flag disabled.")
}

#[cfg(not(feature = "std"))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
	unsafe {
		core::intrinsics::abort()
	}
}

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
#[no_mangle]
pub fn oom(_: core::alloc::Layout) -> ! {
	unsafe {
		core::intrinsics::abort();
	}
}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern fn validate_block(params: *const u8, len: usize) -> u64 {
	loop {}
}
