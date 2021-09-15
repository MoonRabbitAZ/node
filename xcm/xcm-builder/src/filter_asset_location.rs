

use sp_std::marker::PhantomData;
use xcm::v0::{MultiAsset, MultiLocation};
use frame_support::traits::Get;
use xcm_executor::traits::FilterAssetLocation;

pub struct NativeAsset;
impl FilterAssetLocation for NativeAsset {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		matches!(asset, MultiAsset::ConcreteFungible { ref id, .. } if id == origin)
	}
}

pub struct Case<T>(PhantomData<T>);
impl<T: Get<(MultiAsset, MultiLocation)>> FilterAssetLocation for Case<T> {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let (a, o) = T::get();
		a.contains(asset) && &o == origin
	}
}
