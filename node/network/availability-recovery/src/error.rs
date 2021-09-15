

//! The `Error` and `Result` types used by the subsystem.

use futures::channel::oneshot;
use thiserror::Error;

/// Error type used by the Availability Recovery subsystem.
#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Subsystem(#[from] moonrabbit_subsystem::SubsystemError),

	#[error("failed to query full data from store")]
	CanceledQueryFullData(#[source] oneshot::Canceled),

	#[error("failed to query session info")]
	CanceledSessionInfo(#[source] oneshot::Canceled),

	#[error("failed to send response")]
	CanceledResponseSender,

	#[error(transparent)]
	Runtime(#[from] moonrabbit_subsystem::errors::RuntimeApiError),

	#[error(transparent)]
	Erasure(#[from] moonrabbit_erasure_coding::Error),

	#[error(transparent)]
	Util(#[from] moonrabbit_node_subsystem_util::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
