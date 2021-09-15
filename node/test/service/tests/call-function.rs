

use moonrabbit_test_service::*;
use service::TaskExecutor;
use sp_keyring::Sr25519Keyring::{Alice, Bob};

#[substrate_test_utils::test]
async fn call_function_actually_work(task_executor: TaskExecutor) {
	let alice = run_validator_node(task_executor, Alice, || {}, Vec::new(), None);

	let function = moonrabbit_test_runtime::Call::Balances(pallet_balances::Call::transfer(
		Default::default(),
		1,
	));
	let output = alice.send_extrinsic(function, Bob).await.unwrap();

	let res = output.result.expect("return value expected");
	let json = serde_json::from_str::<serde_json::Value>(res.as_str()).expect("valid JSON");
	let object = json.as_object().expect("JSON is an object");
	assert!(object.contains_key("jsonrpc"), "key jsonrpc exists");
	let result = object.get("result");
	let result = result.expect("key result exists");
	assert_eq!(
		result.as_str().map(|x| x.starts_with("0x")),
		Some(true),
		"result starts with 0x",
	);

	alice.task_manager.clean_shutdown().await;
}
