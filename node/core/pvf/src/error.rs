

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

/// A error raised during validation of the candidate.
#[derive(Debug, Clone)]
pub enum ValidationError {
	/// The error was raised because the candidate is invalid.
	InvalidCandidate(InvalidCandidate),
	/// This error is raised due to inability to serve the request.
	InternalError(String),
}

/// A description of an error raised during executing a PVF and can be attributed to the combination
/// of the candidate [`moonrabbit_parachain::primitives::ValidationParams`] and the PVF.
#[derive(Debug, Clone)]
pub enum InvalidCandidate {
	/// The failure is reported by the worker. The string contains the error message.
	///
	/// This also includes the errors reported by the preparation pipeline.
	WorkerReportedError(String),
	/// The worker has died during validation of a candidate. That may fall in one of the following
	/// categories, which we cannot distinguish programmatically:
	///
	/// (a) Some sort of transient glitch caused the worker process to abort. An example would be that
	///     the host machine ran out of free memory and the OOM killer started killing the processes,
	///     and in order to save the parent it will "sacrifice child" first.
	///
	/// (b) The candidate triggered a code path that has lead to the process death. For example,
	///     the PVF found a way to consume unbounded amount of resources and then it either exceeded
	///     an rlimit (if set) or, again, invited OOM killer. Another possibility is a bug in
	///     wasmtime allowed the PVF to gain control over the execution worker.
	///
	/// We attribute such an event to an invalid candidate in either case.
	///
	/// The rationale for this is that a glitch may lead to unfair rejecting candidate by a single
	/// validator. If the glitch is somewhat more persistant the validator will reject all candidate
	/// thrown at it and hopefully the operator notices it by decreased reward performance of the
	/// validator. On the other hand, if the worker died because of (b) we would have better chances
	/// to stop the attack.
	AmbigiousWorkerDeath,
	/// PVF execution (compilation is not included) took more time than was allotted.
	HardTimeout,
}
