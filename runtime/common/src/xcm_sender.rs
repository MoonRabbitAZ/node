

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

//! Xcm sender for relay chain.

use moonrabbit_scale_codec::Encode;
use sp_std::marker::PhantomData;
use xcm::opaque::{VersionedXcm, v0::{SendXcm, MultiLocation, Junction, Xcm, Result, Error}};
use runtime_parachains::{configuration, dmp};

/// Xcm sender for relay chain. It only sends downward message.
pub struct ChildParachainRouter<T>(PhantomData<T>);

impl<T: configuration::Config + dmp::Config> SendXcm for ChildParachainRouter<T> {
	fn send_xcm(dest: MultiLocation, msg: Xcm) -> Result {
		match dest {
			MultiLocation::X1(Junction::Parachain(id)) => {
				// Downward message passing.
				let config = <configuration::Module<T>>::config();
				<dmp::Module<T>>::queue_downward_message(
					&config,
					id.into(),
					VersionedXcm::from(msg).encode(),
				).map_err(Into::<Error>::into)?;
				Ok(())
			}
			d => Err(Error::CannotReachDestination(d, msg)),
		}
	}
}
