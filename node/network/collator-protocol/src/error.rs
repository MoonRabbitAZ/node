

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.
//

//! Error handling related code and Error/Result definitions.

use moonrabbit_node_primitives::UncheckedSignedFullStatement;
use moonrabbit_subsystem::SubsystemError;
use thiserror::Error;

use moonrabbit_node_subsystem_util::{Fault, runtime, unwrap_non_fatal};

use crate::LOG_TARGET;

/// General result.
pub type Result<T> = std::result::Result<T, Error>;

/// Result for fatal only failures.
pub type FatalResult<T> = std::result::Result<T, Fatal>;

/// Errors for statement distribution.
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

/// Fatal runtime errors.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Receiving subsystem message from overseer failed.
	#[error("Receiving message from overseer failed")]
	SubsystemReceive(#[source] SubsystemError),

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::Fatal),
}

/// Errors for fetching of runtime information.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// Signature was invalid on received statement.
	#[error("CollationSeconded contained statement with invalid signature.")]
	InvalidStatementSignature(UncheckedSignedFullStatement),

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::NonFatal),
}

/// Utility for eating top level errors and log them.
///
/// We basically always want to try and continue on error. This utility function is meant to
/// consume top-level errors by simply logging them.
pub fn log_error(result: Result<()>, ctx: &'static str)
	-> FatalResult<()>
{
	if let Some(error) = unwrap_non_fatal(result.map_err(|e| e.0))? {
		tracing::warn!(target: LOG_TARGET, error = ?error, ctx)
	}
	Ok(())
}
