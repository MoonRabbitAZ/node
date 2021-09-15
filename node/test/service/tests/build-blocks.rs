

use futures::{future, pin_mut, select};
use moonrabbit_test_service::*;
use service::TaskExecutor;
use sp_keyring::Sr25519Keyring;

#[substrate_test_utils::test]
async fn ensure_test_service_build_blocks(task_executor: TaskExecutor) {
	let mut builder = sc_cli::LoggerBuilder::new("");
	builder.with_colors(false);
	builder.init().expect("Sets up logger");

	let mut alice = run_validator_node(
		task_executor.clone(),
		Sr25519Keyring::Alice,
		|| {},
		Vec::new(),
		None,
	);
	let mut bob = run_validator_node(
		task_executor.clone(),
		Sr25519Keyring::Bob,
		|| {},
		vec![alice.addr.clone()],
		None,
	);

	{
		let t1 = future::join(alice.wait_for_blocks(3), bob.wait_for_blocks(3)).fuse();
		let t2 = alice.task_manager.future().fuse();
		let t3 = bob.task_manager.future().fuse();

		pin_mut!(t1, t2, t3);

		select! {
			_ = t1 => {},
			_ = t2 => panic!("service Alice failed"),
			_ = t3 => panic!("service Bob failed"),
		}
	}

	alice.task_manager.clean_shutdown().await;
	bob.task_manager.clean_shutdown().await;
}
