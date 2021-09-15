

use sp_std::result::Result;
use xcm::v0::{Xcm, MultiLocation};
use frame_support::weights::Weight;

/// Trait to determine whether the execution engine should actually execute a given XCM.
pub trait ShouldExecute {
	/// Returns `true` if the given `message` may be executed.
	///
	/// - `origin`: The origin (sender) of the message.
	/// - `top_level`: `true`` indicates the initial XCM coming from the `origin`, `false` indicates an embedded
	///   XCM executed internally as part of another message or an `Order`.
	/// - `message`: The message itself.
	/// - `shallow_weight`: The weight of the non-negotiable execution of the message. This does not include any
	///   embedded XCMs sat behind mechanisms like `BuyExecution` which would need to answer for their own weight.
	/// - `weight_credit`: The pre-established amount of weight that the system has determined this message
	///   may utilise in its execution. Typically non-zero only because of prior fee payment, but could
	///   in principle be due to other factors.
	fn should_execute<Call>(
		origin: &MultiLocation,
		top_level: bool,
		message: &Xcm<Call>,
		shallow_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl ShouldExecute for Tuple {
	fn should_execute<Call>(
		origin: &MultiLocation,
		top_level: bool,
		message: &Xcm<Call>,
		shallow_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		for_tuples!( #(
			match Tuple::should_execute(origin, top_level, message, shallow_weight, weight_credit) {
				o @ Ok(()) => return o,
				_ => (),
			}
		)* );
		Err(())
	}
}
