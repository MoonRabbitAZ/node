

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.
//

//! Error handling related code and Error/Result definitions.

use thiserror::Error;
use futures::channel::oneshot;

use moonrabbit_node_subsystem::errors::RuntimeApiError;
use moonrabbit_primitives::v1::SessionIndex;

use crate::Fault;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors for `Runtime` cache.
pub type Error = Fault<NonFatal, Fatal>;

impl From<NonFatal> for Error {
	fn from(e: NonFatal) -> Self {
		Self::from_non_fatal(e)
	}
}

impl From<Fatal> for Error {
	fn from(f: Fatal) -> Self {
		Self::from_fatal(f)
	}
}

/// Fatal runtime errors.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Runtime API subsystem is down, which means we're shutting down.
	#[error("Runtime request got canceled")]
	RuntimeRequestCanceled(oneshot::Canceled),
}

/// Errors for fetching of runtime information.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// Some request to the runtime failed.
	/// For example if we prune a block we're requesting info about.
	#[error("Runtime API error")]
	RuntimeRequest(RuntimeApiError),

	/// We tried fetching a session info which was not available.
	#[error("There was no session with the given index")]
	NoSuchSession(SessionIndex),
}

/// Receive a response from a runtime request and convert errors.
pub(crate) async fn recv_runtime<V>(
	r: oneshot::Receiver<std::result::Result<V, RuntimeApiError>>,
) -> Result<V> {
	let result = r.await
		.map_err(Fatal::RuntimeRequestCanceled)?
		.map_err(NonFatal::RuntimeRequest)?;
	Ok(result)
}
