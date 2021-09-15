

use xcm::v0::{MultiAsset, MultiLocation};

pub trait FilterAssetLocation {
	/// A filter to distinguish between asset/location pairs.
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl FilterAssetLocation for Tuple {
	fn filter_asset_location(what: &MultiAsset, origin: &MultiLocation) -> bool {
		for_tuples!( #(
			if Tuple::filter_asset_location(what, origin) { return true }
		)* );
		false
	}
}
