

use xcm::v0::MultiAsset;

pub trait MatchesFungible<Balance> {
	fn matches_fungible(a: &MultiAsset) -> Option<Balance>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl<Balance> MatchesFungible<Balance> for Tuple {
	fn matches_fungible(a: &MultiAsset) -> Option<Balance> {
		for_tuples!( #(
			match Tuple::matches_fungible(a) { o @ Some(_) => return o, _ => () }
		)* );
		None
	}
}
