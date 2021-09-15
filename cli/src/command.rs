

use log::info;
use service::{IdentifyVariant, self};
use sc_cli::{SubstrateCli, RuntimeVersion, Role};
use crate::cli::{Cli, Subcommand};
use futures::future::TryFutureExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	moonrabbitService(#[from] service::Error),

	#[error(transparent)]
	SubstrateCli(#[from] sc_cli::Error),

	#[error(transparent)]
	SubstrateService(#[from] sc_service::Error),

	#[error("Other: {0}")]
	Other(String),
}

impl std::convert::From<String> for Error {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

type Result<T> = std::result::Result<T, Error>;

fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl SubstrateCli for Cli {
	fn impl_name() -> String { "moonrabbit moonrabbit".into() }

	fn impl_version() -> String { env!("SUBSTRATE_CLI_IMPL_VERSION").into() }

	fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

	fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

	fn support_url() -> String { "https://github.com/moonRabbitAZ/moonrabbit/issues/new".into() }

	fn copyright_start_year() -> i32 { 2017 }

	fn executable_name() -> String { "moonrabbit".into() }

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id == "" {
			let n = get_exec_name().unwrap_or_default();
			["moonrabbit", "moonrabbit"].iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("moonrabbit")
		} else { id };
		Ok(match id {
			"moonrabbit" => Box::new(service::chain_spec::moonrabbit_config()?),
			"moonrabbit-dev" | "dev" => Box::new(service::chain_spec::moonrabbit_development_config()?),
			"moonrabbit-local" => Box::new(service::chain_spec::moonrabbit_local_testnet_config()?),
			"moonrabbit-staging" => Box::new(service::chain_spec::moonrabbit_staging_testnet_config()?),
			"moonrabbit" => Box::new(service::chain_spec::moonrabbit_config()?),
			"moonrabbit-dev" | "dev" => Box::new(service::chain_spec::moonrabbit_development_config()?),
			"moonrabbit-local" => Box::new(service::chain_spec::moonrabbit_local_testnet_config()?),
			"moonrabbit-staging" => Box::new(service::chain_spec::moonrabbit_staging_testnet_config()?),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name().map(|f| f.to_str().map(|s| s.starts_with(&prefix))).flatten().unwrap_or(false)
				};

				// When `force_*` is given or the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if self.run.force_moonrabbit || starts_with("moonrabbit")  {
					Box::new(service::MoonrabbitChainSpec::from_json_file(path)?)
				} else {
					Box::new(service::moonrabbitChainSpec::from_json_file(path)?)
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_moonrabbit() {
			&service::moonrabbit_runtime::VERSION
		} else {
			&service::moonrabbit_runtime::VERSION
		}
	}
}

fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
	use sp_core::crypto::Ss58AddressFormat;

	let ss58_version = if spec.is_kusama() {
		Ss58AddressFormat::KusamaAccount
	} else if spec.is_westend() {
		Ss58AddressFormat::SubstrateAccount
	} else {
		Ss58AddressFormat::moonrabbitAccount
	};

	sp_core::crypto::set_default_ss58_version(ss58_version);
}

const DEV_ONLY_ERROR_PATTERN: &'static str =
	"can only use subcommand with --chain [moonrabbit-dev, kusama-dev, westend-dev, rococo-dev, wococo-dev], got ";

fn ensure_dev(spec: &Box<dyn service::ChainSpec>) -> std::result::Result<(), String> {
	if spec.is_dev() {
		Ok(())
	} else {
		Err(format!("{}{}", DEV_ONLY_ERROR_PATTERN, spec.id()))
	}
}

/// Parses moonrabbit specific CLI arguments and run the service.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run.base)
				.map_err(Error::from)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
				None
			} else {
				Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
			};

			if chain_spec.is_kusama() {
				info!("----------------------------");
				info!("This chain is not in any way");
				info!("      endorsed by the       ");
				info!("     KUSAMA FOUNDATION      ");
				info!("----------------------------");
			}

			let jaeger_agent = cli.run.jaeger_agent;

			runner.run_node_until_exit(move |config| async move {
				let role = config.role.clone();

				let task_manager = match role {
					Role::Light => service::build_light(config).map(|(task_manager, _)| task_manager),
					_ => service::build_full(
						config,
						service::IsCollator::No,
						grandpa_pause,
						cli.run.no_beefy,
						jaeger_agent,
						None,
					).map(|full| full.task_manager)
				}?;
				Ok::<_, Error>(task_manager)
			})
		},
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| {
				cmd.run(config.chain_spec, config.network)
			})?)
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)
				.map_err(Error::SubstrateCli)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::SubstrateCli), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config, None)
					.map_err(Error::moonrabbitService)?;
				Ok((cmd.run(client, config.database).map_err(Error::SubstrateCli), task_manager))
			})?)
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, config.chain_spec).map_err(Error::SubstrateCli), task_manager))
			})?)
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::SubstrateCli), task_manager))
			})?)
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| cmd.run(config.database))?)
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, backend).map_err(Error::SubstrateCli), task_manager))
			})?)
		},
		Some(Subcommand::PvfPrepareWorker(cmd)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_colors(false);
			let _ = builder.init();

			#[cfg(any(target_os = "android", feature = "browser"))]
			{
				return Err(
					sc_cli::Error::Input("PVF preparation workers are not supported under this platform".into()).into()
				);
			}

			#[cfg(not(any(target_os = "android", feature = "browser")))]
			{
				moonrabbit_node_core_pvf::prepare_worker_entrypoint(&cmd.socket_path);
				Ok(())
			}
		},
		Some(Subcommand::PvfExecuteWorker(cmd)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_colors(false);
			let _ = builder.init();

			#[cfg(any(target_os = "android", feature = "browser"))]
			{
				return Err(
					sc_cli::Error::Input("PVF execution workers are not supported under this platform".into()).into()
				);
			}

			#[cfg(not(any(target_os = "android", feature = "browser")))]
			{
				moonrabbit_node_core_pvf::execute_worker_entrypoint(&cmd.socket_path);
				Ok(())
			}
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			ensure_dev(chain_spec).map_err(Error::Other)?;
			if chain_spec.is_kusama() {
				Ok(runner.sync_run(|config| {
					cmd.run::<service::kusama_runtime::Block, service::KusamaExecutor>(config)
						.map_err(|e| Error::SubstrateCli(e))
				})?)
			} else if chain_spec.is_westend() {
				Ok(runner.sync_run(|config| {
					cmd.run::<service::westend_runtime::Block, service::WestendExecutor>(config)
						.map_err(|e| Error::SubstrateCli(e))
				})?)
			} else {
				// else we assume it is moonrabbit.
				Ok(runner.sync_run(|config| {
					cmd.run::<service::moonrabbit_runtime::Block, service::moonrabbitExecutor>(config)
						.map_err(|e| Error::SubstrateCli(e))
				})?)
			}
		},
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			use sc_service::TaskManager;
			let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager = TaskManager::new(
				runner.config().task_executor.clone(),
				*registry,
			).map_err(|e| Error::SubstrateService(sc_service::Error::Prometheus(e)))?;

			ensure_dev(chain_spec).map_err(Error::Other)?;
			if chain_spec.is_moonrabbit() {
				runner.async_run(|config| {
					Ok((cmd.run::<
						service::moonrabbit_runtime::Block,
						service::MoonrabbitExecutor,
					>(config).map_err(Error::SubstrateCli), task_manager))
				})
			} else {
				// else we assume it is moonrabbit.
				runner.async_run(|config| {
					Ok((cmd.run::<
						service::moonrabbit_runtime::Block,
						service::moonrabbitExecutor,
					>(config).map_err(Error::SubstrateCli), task_manager))
				})
			}
		}
	}?;
	Ok(())
}
