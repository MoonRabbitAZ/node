

//! Runtime modules for parachains code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod configuration;
pub mod shared;
pub mod inclusion;
pub mod initializer;
pub mod paras;
pub mod paras_inherent;
pub mod scheduler;
pub mod session_info;
pub mod origin;
pub mod dmp;
pub mod ump;
pub mod hrmp;
pub mod reward_points;

pub mod runtime_api_impl;

mod util;

#[cfg(test)]
mod mock;

pub use origin::{Origin, ensure_parachain};
use primitives::v1::Id as ParaId;
pub use paras::ParaLifecycle;

/// Schedule a para to be initialized at the start of the next session with the given genesis data.
pub fn schedule_para_initialize<T: paras::Config>(
	id: ParaId,
	genesis: paras::ParaGenesisArgs,
) -> Result<(), ()> {
	<paras::Module<T>>::schedule_para_initialize(id, genesis).map_err(|_| ())
}

/// Schedule a para to be cleaned up at the start of the next session.
pub fn schedule_para_cleanup<T: paras::Config>(id: primitives::v1::Id) -> Result<(), ()> {
	<paras::Module<T>>::schedule_para_cleanup(id).map_err(|_| ())
}

/// Schedule a parathread to be upgraded to a parachain.
pub fn schedule_parathread_upgrade<T: paras::Config>(id: ParaId) -> Result<(), ()> {
	paras::Module::<T>::schedule_parathread_upgrade(id).map_err(|_| ())
}

/// Schedule a parachain to be downgraded to a parathread.
pub fn schedule_parachain_downgrade<T: paras::Config>(id: ParaId) -> Result<(), ()> {
	paras::Module::<T>::schedule_parachain_downgrade(id).map_err(|_| ())
}
