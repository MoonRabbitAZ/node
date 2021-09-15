

//! moonrabbit CLI library.

use sc_cli::{RuntimeVersion, SubstrateCli};
use structopt::StructOpt;

/// Sub-commands supported by the collator.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Export the genesis state of the parachain.
	#[structopt(name = "export-genesis-state")]
	ExportGenesisState(ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	#[structopt(name = "export-genesis-wasm")]
	ExportGenesisWasm(ExportGenesisWasmCommand),
}

/// Command for exporting the genesis state of the parachain
#[derive(Debug, StructOpt)]
pub struct ExportGenesisStateCommand {}

/// Command for exporting the genesis wasm file.
#[derive(Debug, StructOpt)]
pub struct ExportGenesisWasmCommand {}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub base: sc_cli::RunCmd,

	/// Id of the parachain this collator collates for.
	#[structopt(long)]
	pub parachain_id: Option<u32>,
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"moonrabbit moonrabbit".into()
	}

	fn impl_version() -> String {
		"0.0.0".into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/moonRabbitAZ/moonrabbit/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> String {
		"moonrabbit".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id.is_empty() { "rococo" } else { id };
		Ok(match id {
			"rococo-staging" => {
				Box::new(moonrabbit_service::chain_spec::rococo_staging_testnet_config()?)
			}
			"rococo-local" => {
				Box::new(moonrabbit_service::chain_spec::rococo_local_testnet_config()?)
			}
			"rococo" => Box::new(moonrabbit_service::chain_spec::rococo_config()?),
			path => {
				let path = std::path::PathBuf::from(path);
				Box::new(moonrabbit_service::RococoChainSpec::from_json_file(path)?)
			}
		})
	}

	fn native_runtime_version(
		_spec: &Box<dyn moonrabbit_service::ChainSpec>,
	) -> &'static RuntimeVersion {
		&moonrabbit_service::rococo_runtime::VERSION
	}
}
