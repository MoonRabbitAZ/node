

use sp_std::{result, convert::TryInto, marker::PhantomData};
use xcm::v0::{Error as XcmError, Result, MultiAsset, MultiLocation};
use sp_runtime::traits::{SaturatedConversion, CheckedSub};
use frame_support::traits::{ExistenceRequirement::AllowDeath, WithdrawReasons, Get};
use xcm_executor::traits::{MatchesFungible, Convert, TransactAsset};
use xcm_executor::Assets;

/// Asset transaction errors.
enum Error {
	/// Asset not found.
	AssetNotFound,
	/// `MultiLocation` to `AccountId` conversion failed.
	AccountIdConversionFailed,
	/// `u128` amount to currency `Balance` conversion failed.
	AmountToBalanceConversionFailed,
}

impl From<Error> for XcmError {
	fn from(e: Error) -> Self {
		match e {
			Error::AssetNotFound => XcmError::FailedToTransactAsset("AssetNotFound"),
			Error::AccountIdConversionFailed =>
				XcmError::FailedToTransactAsset("AccountIdConversionFailed"),
			Error::AmountToBalanceConversionFailed =>
				XcmError::FailedToTransactAsset("AmountToBalanceConversionFailed"),
		}
	}
}

pub struct CurrencyAdapter<Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount>(
	PhantomData<(Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount)>
);

impl<
	Matcher: MatchesFungible<Currency::Balance>,
	AccountIdConverter: Convert<MultiLocation, AccountId>,
	Currency: frame_support::traits::Currency<AccountId>,
	AccountId: Clone,	// can't get away without it since Currency is generic over it.
	CheckedAccount: Get<Option<AccountId>>,
> TransactAsset for CurrencyAdapter<Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount> {
	fn can_check_in(_origin: &MultiLocation, what: &MultiAsset) -> Result {
		// Check we handle this asset.
		let amount: Currency::Balance = Matcher::matches_fungible(what)
			.ok_or(Error::AssetNotFound)?;
		if let Some(checked_account) = CheckedAccount::get() {
			let new_balance = Currency::free_balance(&checked_account)
				.checked_sub(&amount)
				.ok_or(XcmError::NotWithdrawable)?;
			Currency::ensure_can_withdraw(&checked_account, amount, WithdrawReasons::TRANSFER, new_balance)
				.map_err(|_| XcmError::NotWithdrawable)?;
		}
		Ok(())
	}

	fn check_in(_origin: &MultiLocation, what: &MultiAsset) {
		if let Some(amount) = Matcher::matches_fungible(what) {
			if let Some(checked_account) = CheckedAccount::get() {
				let ok = Currency::withdraw(&checked_account, amount, WithdrawReasons::TRANSFER, AllowDeath).is_ok();
				debug_assert!(ok, "`can_check_in` must have returned `true` immediately prior; qed");
			}
		}
	}

	fn check_out(_dest: &MultiLocation, what: &MultiAsset) {
		if let Some(amount) = Matcher::matches_fungible(what) {
			if let Some(checked_account) = CheckedAccount::get() {
				Currency::deposit_creating(&checked_account, amount);
			}
		}
	}

	fn deposit_asset(what: &MultiAsset, who: &MultiLocation) -> Result {
		// Check we handle this asset.
		let amount: u128 = Matcher::matches_fungible(&what)
			.ok_or(Error::AssetNotFound)?
			.saturated_into();
		let who = AccountIdConverter::convert_ref(who)
			.map_err(|()| Error::AccountIdConversionFailed)?;
		let balance_amount = amount
			.try_into()
			.map_err(|_| Error::AmountToBalanceConversionFailed)?;
		let _imbalance = Currency::deposit_creating(&who, balance_amount);
		Ok(())
	}

	fn withdraw_asset(
		what: &MultiAsset,
		who: &MultiLocation
	) -> result::Result<Assets, XcmError> {
		// Check we handle this asset.
		let amount: u128 = Matcher::matches_fungible(what)
			.ok_or(Error::AssetNotFound)?
			.saturated_into();
		let who = AccountIdConverter::convert_ref(who)
			.map_err(|()| Error::AccountIdConversionFailed)?;
		let balance_amount = amount
			.try_into()
			.map_err(|_| Error::AmountToBalanceConversionFailed)?;
		Currency::withdraw(&who, balance_amount, WithdrawReasons::TRANSFER, AllowDeath)
			.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
		Ok(what.clone().into())
	}
}
