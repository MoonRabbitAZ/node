

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Subsystem(#[from] moonrabbit_node_subsystem::SubsystemError),
	#[error(transparent)]
	OneshotRecv(#[from] futures::channel::oneshot::Canceled),
	#[error(transparent)]
	Runtime(#[from] moonrabbit_node_subsystem::errors::RuntimeApiError),
	#[error(transparent)]
	Util(#[from] moonrabbit_node_subsystem_util::Error),
	#[error(transparent)]
	Erasure(#[from] moonrabbit_erasure_coding::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
