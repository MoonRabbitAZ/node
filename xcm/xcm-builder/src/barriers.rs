

use sp_std::{result::Result, marker::PhantomData};
use xcm::v0::{Xcm, Order, MultiLocation, Junction};
use frame_support::{ensure, traits::Contains, weights::Weight};
use xcm_executor::traits::{OnResponse, ShouldExecute};
use moonrabbit_parachain::primitives::IsSystem;

pub struct TakeWeightCredit;
impl ShouldExecute for TakeWeightCredit {
	fn should_execute<Call>(
		_origin: &MultiLocation,
		_top_level: bool,
		_message: &Xcm<Call>,
		shallow_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		*weight_credit = weight_credit.checked_sub(shallow_weight).ok_or(())?;
		Ok(())
	}
}

pub struct AllowTopLevelPaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowTopLevelPaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		top_level: bool,
		message: &Xcm<Call>,
		shallow_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		ensure!(T::contains(origin), ());
		ensure!(top_level, ());
		match message {
			Xcm::TeleportAsset { effects, .. }
			| Xcm::WithdrawAsset { effects, ..}
			| Xcm::ReserveAssetDeposit { effects, ..}
			if matches!(
					effects.first(),
					Some(Order::BuyExecution { debt, ..}) if *debt >= shallow_weight
				)
			=> Ok(()),
			_ => Err(()),
		}
	}
}

pub struct AllowUnpaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowUnpaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		_top_level: bool,
		_message: &Xcm<Call>,
		_shallow_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		ensure!(T::contains(origin), ());
		Ok(())
	}
}

pub struct IsChildSystemParachain<ParaId>(PhantomData<ParaId>);
impl<
	ParaId: IsSystem + From<u32>,
> Contains<MultiLocation> for IsChildSystemParachain<ParaId> {
	fn contains(l: &MultiLocation) -> bool {
		matches!(l, MultiLocation::X1(Junction::Parachain(id)) if ParaId::from(*id).is_system())
	}
}

pub struct AllowKnownQueryResponses<ResponseHandler>(PhantomData<ResponseHandler>);
impl<ResponseHandler: OnResponse> ShouldExecute for AllowKnownQueryResponses<ResponseHandler> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		_top_level: bool,
		message: &Xcm<Call>,
		_shallow_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		match message {
			Xcm::QueryResponse { query_id, .. } if ResponseHandler::expecting_response(origin, *query_id)
			=> Ok(()),
			_ => Err(()),
		}
	}
}
