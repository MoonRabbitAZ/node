

use xcm::v0::SendXcm;
use frame_support::dispatch::{Dispatchable, Parameter};
use frame_support::weights::{PostDispatchInfo, GetDispatchInfo};
use crate::traits::{
	TransactAsset, ConvertOrigin, FilterAssetLocation, InvertLocation, ShouldExecute, WeightTrader, WeightBounds,
	OnResponse,
};

/// The trait to parametrize the `XcmExecutor`.
pub trait Config {
	/// The outer call dispatch type.
	type Call: Parameter + Dispatchable<PostInfo=PostDispatchInfo> + GetDispatchInfo;

	/// How to send an onward XCM message.
	type XcmSender: SendXcm;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// How to get a call origin from a `OriginKind` value.
	type OriginConverter: ConvertOrigin<<Self::Call as Dispatchable>::Origin>;

	/// Combinations of (Location, Asset) pairs which we unilateral trust as reserves.
	type IsReserve: FilterAssetLocation;

	/// Combinations of (Location, Asset) pairs which we bilateral trust as teleporters.
	type IsTeleporter: FilterAssetLocation;

	/// Means of inverting a location.
	type LocationInverter: InvertLocation;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// The means of determining an XCM message's weight.
	type Weigher: WeightBounds<Self::Call>;

	/// The means of purchasing weight credit for XCM execution.
	type Trader: WeightTrader;

	/// What to do when a response of a query is found.
	type ResponseHandler: OnResponse;
}
