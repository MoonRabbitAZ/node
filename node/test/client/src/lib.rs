

//! A moonrabbit test client.
//!
//! This test client is using the moonrabbit test runtime.

mod block_builder;

use moonrabbit_primitives::v1::Block;
use sc_service::client;
use sp_core::storage::Storage;
use sp_runtime::BuildStorage;

pub use block_builder::*;
pub use substrate_test_client::*;
pub use moonrabbit_test_service::{
	Client, construct_extrinsic, construct_transfer_extrinsic, moonrabbitTestExecutor, FullBackend,
};
pub use moonrabbit_test_runtime as runtime;

/// Test client executor.
pub type Executor = client::LocalCallExecutor<FullBackend, sc_executor::NativeExecutor<moonrabbitTestExecutor>>;

/// Test client builder for moonrabbit.
pub type TestClientBuilder = substrate_test_client::TestClientBuilder<Block, Executor, FullBackend, GenesisParameters>;

/// LongestChain type for the test runtime/client.
pub type LongestChain = sc_consensus::LongestChain<FullBackend, Block>;

/// Parameters of test-client builder with test-runtime.
#[derive(Default)]
pub struct GenesisParameters;

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		moonrabbit_test_service::chain_spec::moonrabbit_local_testnet_genesis()
			.build_storage()
			.expect("Builds test runtime genesis storage")
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Build the test client.
	fn build(self) -> Client {
		self.build_with_longest_chain().0
	}

	/// Build the test client and longest chain selector.
	fn build_with_longest_chain(self) -> (Client, LongestChain);
}

impl TestClientBuilderExt for TestClientBuilder {
	fn build_with_longest_chain(self) -> (Client, LongestChain) {
		self.build_with_native_executor(None)
	}
}

/// A `TestClientBuilder` with default backend and executor.
pub trait DefaultTestClientBuilderExt: Sized {
	/// Create new `TestClientBuilder`
	fn new() -> Self;
}

impl DefaultTestClientBuilderExt for TestClientBuilder {
	fn new() -> Self {
		Self::with_default_backend()
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use sp_consensus::BlockOrigin;

	#[test]
	fn ensure_test_client_can_build_and_import_block() {
		let mut client = TestClientBuilder::new().build();

		let block_builder = client.init_moonrabbit_block_builder();
		let block = block_builder.build().expect("Finalizes the block").block;

		futures::executor::block_on(client.import(BlockOrigin::Own, block)).expect("Imports the block");
	}

	#[test]
	fn ensure_test_client_can_push_extrinsic() {
		let mut client = TestClientBuilder::new().build();

		let transfer = construct_transfer_extrinsic(
			&client,
			sp_keyring::Sr25519Keyring::Alice,
			sp_keyring::Sr25519Keyring::Bob,
			1000,
		);
		let mut block_builder = client.init_moonrabbit_block_builder();
		block_builder.push_moonrabbit_extrinsic(transfer).expect("Pushes extrinsic");

		let block = block_builder.build().expect("Finalizes the block").block;

		futures::executor::block_on(client.import(BlockOrigin::Own, block)).expect("Imports the block");
	}
}
