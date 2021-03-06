

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

//! Various things for testing other crates.
//!
//! N.B. This is not guarded with some feature flag. Overexposing items here may affect the final
//!      artifact even for production builds.

pub mod worker_common {
	pub use crate::worker_common::{spawn_with_program_path, SpawnErr};
}

/// A function that emulates the stitches together behaviors of the preparation and the execution
/// worker in a single synchronous function.
pub fn validate_candidate(
	code: &[u8],
	params: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	use crate::executor_intf::{prevalidate, prepare, execute, TaskExecutor};

	let blob = prevalidate(code)?;
	let artifact = prepare(blob)?;
	let executor = TaskExecutor::new()?;
	let result = execute(&artifact, params, executor)?;

	Ok(result)
}

/// Use this macro to declare a `fn main() {}` that will check the arguments and dispatch them to
/// the appropriate worker, making the executable that can be used for spawning workers.
#[macro_export]
macro_rules! decl_puppet_worker_main {
	() => {
		fn main() {
			let args = std::env::args().collect::<Vec<_>>();
			if args.len() < 2 {
				panic!("wrong number of arguments");
			}

			let subcommand = &args[1];
			match subcommand.as_ref() {
				"sleep" => {
					std::thread::sleep(std::time::Duration::from_secs(5));
				}
				"prepare-worker" => {
					let socket_path = &args[2];
					$crate::prepare_worker_entrypoint(socket_path);
				}
				"execute-worker" => {
					let socket_path = &args[2];
					$crate::execute_worker_entrypoint(socket_path);
				}
				other => panic!("unknown subcommand: {}", other),
			}
		}
	};
}
