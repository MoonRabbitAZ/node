

//! Integration test that ensures that we can build and include parachain
//! blocks of the adder parachain.

const PUPPET_EXE: &str = env!("CARGO_BIN_EXE_adder_collator_puppet_worker");

// If this test is failing, make sure to run all tests with the `real-overseer` feature being enabled.
#[substrate_test_utils::test]
async fn collating_using_adder_collator(task_executor: sc_service::TaskExecutor) {
	use sp_keyring::AccountKeyring::*;
	use futures::join;
	use moonrabbit_primitives::v1::Id as ParaId;

	let mut builder = sc_cli::LoggerBuilder::new("");
	builder.with_colors(false);
	builder.init().expect("Set up logger");

	let para_id = ParaId::from(100);

	// start alice
	let alice = moonrabbit_test_service::run_validator_node(
		task_executor.clone(),
		Alice, || {},
		vec![],
		Some(PUPPET_EXE.into()),
	);

	// start bob
	let bob = moonrabbit_test_service::run_validator_node(
		task_executor.clone(),
		Bob,
		|| {},
		vec![alice.addr.clone()],
		Some(PUPPET_EXE.into()),
	);

	let collator = test_parachain_adder_collator::Collator::new();

	// register parachain
	alice
		.register_parachain(
			para_id,
			collator.validation_code().to_vec(),
			collator.genesis_head(),
		)
		.await
		.unwrap();

	// run the collator node
	let mut charlie = moonrabbit_test_service::run_collator_node(
		task_executor.clone(),
		Charlie,
		|| {},
		vec![alice.addr.clone(), bob.addr.clone()],
		collator.collator_key(),
	);

	charlie.register_collator(
		collator.collator_key(),
		para_id,
		collator.create_collation_function(charlie.task_manager.spawn_handle()),
	).await;

	// Wait until the parachain has 4 blocks produced.
	collator.wait_for_blocks(4).await;

	// Wait until the collator received `12` seconded statements for its collations.
	collator.wait_for_seconded_collations(12).await;

	join!(
		alice.task_manager.clean_shutdown(),
		bob.task_manager.clean_shutdown(),
		charlie.task_manager.clean_shutdown(),
	);
}
