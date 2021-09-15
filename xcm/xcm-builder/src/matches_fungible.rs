

use sp_std::{marker::PhantomData, convert::TryFrom};
use sp_runtime::traits::CheckedConversion;
use xcm::v0::{MultiAsset, MultiLocation};
use frame_support::traits::Get;
use xcm_executor::traits::MatchesFungible;

pub struct IsConcrete<T>(PhantomData<T>);
impl<T: Get<MultiLocation>, B: TryFrom<u128>> MatchesFungible<B> for IsConcrete<T> {
	fn matches_fungible(a: &MultiAsset) -> Option<B> {
		match a {
			MultiAsset::ConcreteFungible { id, amount } if id == &T::get() =>
				CheckedConversion::checked_from(*amount),
			_ => None,
		}
	}
}
pub struct IsAbstract<T>(PhantomData<T>);
impl<T: Get<&'static [u8]>, B: TryFrom<u128>> MatchesFungible<B> for IsAbstract<T> {
	fn matches_fungible(a: &MultiAsset) -> Option<B> {
		match a {
			MultiAsset::AbstractFungible { id, amount } if &id[..] == T::get() =>
				CheckedConversion::checked_from(*amount),
			_ => None,
		}
	}
}
