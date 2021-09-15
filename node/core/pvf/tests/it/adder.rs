

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

use super::TestHost;
use moonrabbit_parachain::{
	primitives::{
		RelayChainBlockNumber, BlockData as GenericBlockData, HeadData as GenericHeadData,
		ValidationParams,
	},
};
use moonrabbit_scale_codec::{Decode, Encode};
use adder::{HeadData, BlockData, hash_state};

#[async_std::test]
async fn execute_good_on_parent() {
	let parent_head = HeadData {
		number: 0,
		parent_hash: [0; 32],
		post_state: hash_state(0),
	};

	let block_data = BlockData { state: 0, add: 512 };

	let host = TestHost::new();

	let ret = host
		.validate_candidate(
			adder::wasm_binary_unwrap(),
			ValidationParams {
				parent_head: GenericHeadData(parent_head.encode()),
				block_data: GenericBlockData(block_data.encode()),
				relay_parent_number: 1,
				relay_parent_storage_root: Default::default(),
			},
		)
		.await
		.unwrap();

	let new_head = HeadData::decode(&mut &ret.head_data.0[..]).unwrap();

	assert_eq!(new_head.number, 1);
	assert_eq!(new_head.parent_hash, parent_head.hash());
	assert_eq!(new_head.post_state, hash_state(512));
}

#[async_std::test]
async fn execute_good_chain_on_parent() {
	let mut number = 0;
	let mut parent_hash = [0; 32];
	let mut last_state = 0;

	let host = TestHost::new();

	for add in 0..10 {
		let parent_head = HeadData {
			number,
			parent_hash,
			post_state: hash_state(last_state),
		};

		let block_data = BlockData {
			state: last_state,
			add,
		};

		let ret = host
			.validate_candidate(
				adder::wasm_binary_unwrap(),
				ValidationParams {
					parent_head: GenericHeadData(parent_head.encode()),
					block_data: GenericBlockData(block_data.encode()),
					relay_parent_number: number as RelayChainBlockNumber + 1,
					relay_parent_storage_root: Default::default(),
				},
			)
			.await
			.unwrap();

		let new_head = HeadData::decode(&mut &ret.head_data.0[..]).unwrap();

		assert_eq!(new_head.number, number + 1);
		assert_eq!(new_head.parent_hash, parent_head.hash());
		assert_eq!(new_head.post_state, hash_state(last_state + add));

		number += 1;
		parent_hash = new_head.hash();
		last_state += add;
	}
}

#[async_std::test]
async fn execute_bad_on_parent() {
	let parent_head = HeadData {
		number: 0,
		parent_hash: [0; 32],
		post_state: hash_state(0),
	};

	let block_data = BlockData {
		state: 256, // start state is wrong.
		add: 256,
	};

	let host = TestHost::new();

	let _ret = host
		.validate_candidate(
			adder::wasm_binary_unwrap(),
			ValidationParams {
				parent_head: GenericHeadData(parent_head.encode()),
				block_data: GenericBlockData(block_data.encode()),
				relay_parent_number: 1,
				relay_parent_storage_root: Default::default(),
			},
		)
		.await
		.unwrap_err();
}
