

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

//! Execution part of the pipeline.
//!
//! The validation host [runs the queue][`start`] communicating with it by sending [`ToQueue`]
//! messages. The queue will spawn workers in new processes. Those processes should jump to
//! [`worker_entrypoint`].

mod queue;
mod worker;

pub use queue::{ToQueue, start};
pub use worker::worker_entrypoint;
