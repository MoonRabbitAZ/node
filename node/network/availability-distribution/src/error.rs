

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.
//

//! Error handling related code and Error/Result definitions.

use moonrabbit_node_network_protocol::request_response::request::RequestError;
use thiserror::Error;

use futures::channel::oneshot;

use moonrabbit_node_subsystem_util::{Fault, runtime, unwrap_non_fatal};
use moonrabbit_subsystem::SubsystemError;

use crate::LOG_TARGET;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub Fault<NonFatal, Fatal>);

impl From<NonFatal> for Error {
	fn from(e: NonFatal) -> Self {
		Self(Fault::from_non_fatal(e))
	}
}

impl From<Fatal> for Error {
	fn from(f: Fatal) -> Self {
		Self(Fault::from_fatal(f))
	}
}

impl From<runtime::Error> for Error {
	fn from(o: runtime::Error) -> Self {
		Self(Fault::from_other(o))
	}
}

/// Fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Spawning a running task failed.
	#[error("Spawning subsystem task failed")]
	SpawnTask(#[source] SubsystemError),

	/// Requester stream exhausted.
	#[error("Erasure chunk requester stream exhausted")]
	RequesterExhausted,

	#[error("Receive channel closed")]
	IncomingMessageChannel(#[source] SubsystemError),

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::Fatal),
}

/// Non fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// av-store will drop the sender on any error that happens.
	#[error("Response channel to obtain chunk failed")]
	QueryChunkResponseChannel(#[source] oneshot::Canceled),

	/// av-store will drop the sender on any error that happens.
	#[error("Response channel to obtain available data failed")]
	QueryAvailableDataResponseChannel(#[source] oneshot::Canceled),

	/// We tried accessing a session that was not cached.
	#[error("Session is not cached.")]
	NoSuchCachedSession,

	/// Sending request response failed (Can happen on timeouts for example).
	#[error("Sending a request's response failed.")]
	SendResponse,

	/// Fetching PoV failed with `RequestError`.
	#[error("FetchPoV request error")]
	FetchPoV(#[source] RequestError),

	/// Fetching PoV failed as the received PoV did not match the expected hash.
	#[error("Fetched PoV does not match expected hash")]
	UnexpectedPoV,

	#[error("Remote responded with `NoSuchPoV`")]
	NoSuchPoV,

	/// No validator with the index could be found in current session.
	#[error("Given validator index could not be found")]
	InvalidValidatorIndex,

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::NonFatal),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Utility for eating top level errors and log them.
///
/// We basically always want to try and continue on error. This utility function is meant to
/// consume top-level errors by simply logging them
pub fn log_error(result: Result<()>, ctx: &'static str)
	-> std::result::Result<(), Fatal>
{
	if let Some(error) = unwrap_non_fatal(result.map_err(|e| e.0))? {
		tracing::warn!(target: LOG_TARGET, error = ?error, ctx);
	}
	Ok(())
}
