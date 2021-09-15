

//! Utilities that don't belong to any particular module but may draw
//! on all modules.

use primitives::v1::{Id as ParaId, PersistedValidationData, ValidatorIndex};
use sp_std::vec::Vec;

use crate::{configuration, paras, hrmp};

/// Make the persisted validation data for a particular parachain, a specified relay-parent and it's
/// storage root.
///
/// This ties together the storage of several modules.
pub fn make_persisted_validation_data<T: paras::Config + hrmp::Config>(
	para_id: ParaId,
	relay_parent_number: T::BlockNumber,
	relay_parent_storage_root: T::Hash,
) -> Option<PersistedValidationData<T::Hash, T::BlockNumber>> {
	let config = <configuration::Module<T>>::config();

	Some(PersistedValidationData {
		parent_head: <paras::Module<T>>::para_head(&para_id)?,
		relay_parent_number,
		relay_parent_storage_root,
		max_pov_size: config.max_pov_size,
	})
}

/// Take the active subset of a set containing all validators.
pub fn take_active_subset<T: Clone>(active_validators: &[ValidatorIndex], set: &[T]) -> Vec<T> {
	let subset: Vec<_> = active_validators.iter()
		.filter_map(|i| set.get(i.0 as usize))
		.cloned()
		.collect();

	if subset.len() != active_validators.len() {
		log::warn!(
			target: "runtime::parachains",
			"Took active validators from set with wrong size",
		);
	}

	subset
}
