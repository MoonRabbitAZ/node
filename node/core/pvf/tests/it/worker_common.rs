

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

use crate::PUPPET_EXE;
use moonrabbit_node_core_pvf::testing::worker_common::{spawn_with_program_path, SpawnErr};
use std::time::Duration;

#[async_std::test]
async fn spawn_timeout() {
	let result = spawn_with_program_path(
		"integration-test",
		PUPPET_EXE,
		&["sleep"],
		Duration::from_secs(2),
	)
	.await;
	assert!(matches!(result, Err(SpawnErr::AcceptTimeout)));
}

#[async_std::test]
async fn should_connect() {
	let _ = spawn_with_program_path(
		"integration-test",
		PUPPET_EXE,
		&["prepare-worker"],
		Duration::from_secs(2),
	)
	.await
	.unwrap();
}
