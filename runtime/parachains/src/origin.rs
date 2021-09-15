

//! Declaration of the parachain specific origin and a pallet that hosts it.

use sp_std::result;
use sp_runtime::traits::BadOrigin;
use primitives::v1::Id as ParaId;
use moonrabbit_scale_codec::{Decode, Encode};

/// Origin for the parachains.
#[derive(PartialEq, Eq, Clone, Encode, Decode, sp_core::RuntimeDebug)]
pub enum Origin {
	/// It comes from a parachain.
	Parachain(ParaId),
}

/// Ensure that the origin `o` represents a parachain.
/// Returns `Ok` with the parachain ID that effected the extrinsic or an `Err` otherwise.
pub fn ensure_parachain<OuterOrigin>(o: OuterOrigin) -> result::Result<ParaId, BadOrigin>
	where OuterOrigin: Into<result::Result<Origin, OuterOrigin>>
{
	match o.into() {
		Ok(Origin::Parachain(id)) => Ok(id),
		_ => Err(BadOrigin),
	}
}

/// The origin module.
pub trait Config: frame_system::Config {}

frame_support::decl_module! {
	/// There is no way to register an origin type in `construct_runtime` without a pallet the origin
	/// belongs to.
	///
	/// This module fulfills only the single purpose of housing the `Origin` in `construct_runtime`.
	///
	// ideally, though, the `construct_runtime` should support a free-standing origin.
	pub struct Module<T: Config> for enum Call where origin: <T as frame_system::Config>::Origin {}
}

impl From<u32> for Origin {
	fn from(id: u32) -> Origin {
		Origin::Parachain(id.into())
	}
}
