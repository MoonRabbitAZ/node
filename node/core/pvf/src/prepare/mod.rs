

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

//! Preparation part of pipeline
//!
//! The validation host spins up two processes: the queue (by running [`start_queue`]) and the pool
//! (by running [`start_pool`]).
//!
//! The pool will spawn workers in new processes and those should execute pass control to
//! [`worker_entrypoint`].

mod pool;
mod queue;
mod worker;

pub use queue::{ToQueue, FromQueue, start as start_queue};
pub use pool::start as start_pool;
pub use worker::worker_entrypoint;
