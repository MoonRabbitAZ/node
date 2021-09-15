

use assert_cmd::cargo::cargo_bin;
use std::process::Command;
use tempfile::tempdir;

#[test]
#[cfg(unix)]
fn invalid_order_arguments() {
	let tmpdir = tempdir().expect("could not create temp dir");

	let status = Command::new(cargo_bin("moonrabbit"))
		.args(&["--dev", "invalid_order_arguments", "-d"])
		.arg(tmpdir.path())
		.arg("-y")
		.status()
		.unwrap();
	assert!(!status.success());
}
