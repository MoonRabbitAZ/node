
//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-26, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("moonrabbit-dev"), DB CACHE: 128

// Executed Command:
// target/release/moonrabbit
// benchmark
// --chain=moonrabbit-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096



#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_identity.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	fn add_registrar(r: u32, ) -> Weight {
		(21_283_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((303_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(53_184_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((255_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((1_055_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32, ) -> Weight {
		(41_073_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((6_793_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32, ) -> Weight {
		(42_051_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_309_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(49_465_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((205_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((2_317_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 2_000
			.saturating_add((716_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(54_855_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((310_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_368_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(49_913_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((215_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_347_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32, ) -> Weight {
		(7_665_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((300_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32, ) -> Weight {
		(8_757_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((306_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32, ) -> Weight {
		(7_661_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((302_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(34_837_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((284_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_366_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(66_368_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((107_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_314_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32, ) -> Weight {
		(55_474_000 as Weight)
			// Standard Error: 0
			.saturating_add((221_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32, ) -> Weight {
		(16_929_000 as Weight)
			// Standard Error: 0
			.saturating_add((25_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32, ) -> Weight {
		(56_833_000 as Weight)
			// Standard Error: 0
			.saturating_add((193_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32, ) -> Weight {
		(34_255_000 as Weight)
			// Standard Error: 0
			.saturating_add((187_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
