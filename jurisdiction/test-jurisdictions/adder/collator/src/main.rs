

//! Collator for the adder test parachain.

use moonrabbit_node_primitives::CollationGenerationConfig;
use moonrabbit_node_subsystem::messages::{CollationGenerationMessage, CollatorProtocolMessage};
use moonrabbit_primitives::v1::Id as ParaId;
use moonrabbit_cli::{Error, Result};
use sc_cli::{Error as SubstrateCliError, Role, SubstrateCli};
use sp_core::hexdisplay::HexDisplay;
use test_parachain_adder_collator::Collator;

/// The parachain ID to collate for in case it wasn't set explicitly through CLI.
const DEFAULT_PARA_ID: ParaId = ParaId::new(100);

mod cli;
use cli::Cli;

fn main() -> Result<()> {
	let cli = Cli::from_args();

	match cli.subcommand {
		Some(cli::Subcommand::ExportGenesisState(_params)) => {
			let collator = Collator::new();
			println!("0x{:?}", HexDisplay::from(&collator.genesis_head()));

			Ok::<_, Error>(())
		}
		Some(cli::Subcommand::ExportGenesisWasm(_params)) => {
			let collator = Collator::new();
			println!("0x{:?}", HexDisplay::from(&collator.validation_code()));

			Ok(())
		}
		None => {
			let runner = cli.create_runner(&cli.run.base)
				.map_err(|e| SubstrateCliError::Application(Box::new(e) as Box::<(dyn 'static + Send + Sync + std::error::Error)>))?;

			runner.run_node_until_exit(|config| async move {
				let role = config.role.clone();

				match role {
					Role::Light => Err("Light client not supported".into()),
					_ => {
						let collator = Collator::new();

						let full_node = moonrabbit_service::build_full(
							config,
							moonrabbit_service::IsCollator::Yes(collator.collator_key()),
							None,
							true,
							None,
							None,
						).map_err(|e| e.to_string())?;
						let mut overseer_handler = full_node
							.overseer_handler
							.expect("Overseer handler should be initialized for collators");

						let genesis_head_hex =
							format!("0x{:?}", HexDisplay::from(&collator.genesis_head()));
						let validation_code_hex =
							format!("0x{:?}", HexDisplay::from(&collator.validation_code()));

						let para_id = cli.run.parachain_id.map(ParaId::from).unwrap_or(DEFAULT_PARA_ID);

						log::info!("Running adder collator for parachain id: {}", para_id);
						log::info!("Genesis state: {}", genesis_head_hex);
						log::info!("Validation code: {}", validation_code_hex);

						let config = CollationGenerationConfig {
							key: collator.collator_key(),
							collator: collator.create_collation_function(full_node.task_manager.spawn_handle()),
							para_id,
						};
						overseer_handler
							.send_msg(CollationGenerationMessage::Initialize(config))
							.await;

						overseer_handler
							.send_msg(CollatorProtocolMessage::CollateOn(para_id))
							.await;

						Ok(full_node.task_manager)
					}
				}
			})
		}
	}?;
	Ok(())
}
