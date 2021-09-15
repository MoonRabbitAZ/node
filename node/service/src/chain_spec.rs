

//! moonrabbit chain configurations.

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use beefy_primitives::ecdsa::AuthorityId as BeefyId;
use grandpa::AuthorityId as GrandpaId;
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use moonrabbit::constants::currency::UNITS as DOT;
use moonrabbit_node_primitives::MAX_POV_SIZE;
use moonrabbit_primitives::v1::{AccountId, AccountPublic, AssignmentId, ValidatorId};
use moonrabbit_runtime as moonrabbit;
use moonrabbit_runtime as moonrabbit;
use moonrabbit_runtime::constants::currency::UNITS as AAA;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;

const moonrabbit_STAGING_TELEMETRY_URL: &str = "wss://telemetry.moonrabbit.io/submit/";
const MOONRABBIT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.moonrabbit.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<moonrabbit_primitives::v1::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<moonrabbit_primitives::v1::Block>,
}

/// The `ChainSpec` parametrised for the moonrabbit runtime.
pub type moonrabbitChainSpec = service::GenericChainSpec<moonrabbit::GenesisConfig, Extensions>;

pub type MoonrabbitChainSpec = service::GenericChainSpec<moonrabbit::GenesisConfig, Extensions>;


/// Extension for the Rococo genesis config to support a custom changes to the genesis state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RococoGenesisExt {
	/// The runtime genesis config.
	/// The session length in blocks.
	///
	/// If `None` is supplied, the default value is used.
	session_length_in_blocks: Option<u32>,
}

pub fn moonrabbit_config() -> Result<moonrabbitChainSpec, String> {
	moonrabbitChainSpec::from_json_bytes(&include_bytes!("../res/moonrabbit.json")[..])
}

fn moonrabbit_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> moonrabbit::SessionKeys {
	moonrabbit::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

fn moonrabbit_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> moonrabbit::SessionKeys {
	moonrabbit::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

fn moonrabbit_staging_testnet_config_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![];

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	moonrabbit::GenesisConfig {
		frame_system: moonrabbit::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: moonrabbit::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		pallet_indices: moonrabbit::IndicesConfig { indices: vec![] },
		pallet_session: moonrabbit::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						moonrabbit_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		pallet_staking: moonrabbit::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						moonrabbit::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		pallet_elections_phragmen: Default::default(),
		pallet_democracy: Default::default(),
		pallet_collective_Instance1: moonrabbit::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: moonrabbit::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_membership_Instance1: Default::default(),
		pallet_babe: moonrabbit::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(moonrabbit::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_grandpa: Default::default(),
		pallet_im_online: Default::default(),
		pallet_authority_discovery: moonrabbit::AuthorityDiscoveryConfig { keys: vec![] },
		claims: moonrabbit::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		},
		pallet_vesting: moonrabbit::VestingConfig { vesting: vec![] },
		pallet_treasury: Default::default(),
	}
}

fn moonrabbit_staging_testnet_config_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![];

	const ENDOWMENT: u128 = 1_000_000 * AAA;
	const STASH: u128 = 100 * AAA;

	moonrabbit::GenesisConfig {
		frame_system: moonrabbit::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: moonrabbit::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		pallet_indices: moonrabbit::IndicesConfig { indices: vec![] },
		pallet_session: moonrabbit::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						moonrabbit_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		pallet_staking: moonrabbit::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						moonrabbit::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		pallet_elections_phragmen: Default::default(),
		pallet_democracy: Default::default(),
		pallet_collective_Instance1: moonrabbit::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: moonrabbit::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_membership_Instance1: Default::default(),
		pallet_babe: moonrabbit::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(moonrabbit::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_grandpa: Default::default(),
		pallet_im_online: Default::default(),
		pallet_authority_discovery: moonrabbit::AuthorityDiscoveryConfig { keys: vec![] },
		claims: moonrabbit::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		},
		pallet_vesting: moonrabbit::VestingConfig { vesting: vec![] },
		pallet_treasury: Default::default(),
	}
}

/// moonrabbit staging testnet config.
pub fn moonrabbit_staging_testnet_config() -> Result<moonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("moonrabbit development wasm not available")?;
	let boot_nodes = vec![];

	Ok(moonrabbitChainSpec::from_genesis(
		"moonrabbit Staging Testnet",
		"moonrabbit_staging_testnet",
		ChainType::Live,
		move || moonrabbit_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(moonrabbit_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("moonrabbit Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

pub fn moonrabbit_staging_testnet_config() -> Result<MoonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("Test development wasm not available")?;
	let boot_nodes = vec![];

	Ok(MoonrabbitChainSpec::from_genesis(
		"Moonrabbit Staging Testnet",
		"Test_staging_testnet",
		ChainType::Live,
		move || moonrabbit_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(MOONRABBIT_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Test Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(
		keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed)
	)
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
}

/// Helper function to create moonrabbit GenesisConfig for testing
pub fn moonrabbit_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> moonrabbit::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	moonrabbit::GenesisConfig {
		frame_system: moonrabbit::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_indices: moonrabbit::IndicesConfig { indices: vec![] },
		pallet_balances: moonrabbit::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		},
		pallet_session: moonrabbit::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						moonrabbit_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		pallet_staking: moonrabbit::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						moonrabbit::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		pallet_elections_phragmen: Default::default(),
		pallet_democracy: moonrabbit::DemocracyConfig::default(),
		pallet_collective_Instance1: moonrabbit::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: moonrabbit::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_membership_Instance1: Default::default(),
		pallet_babe: moonrabbit::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(moonrabbit::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_grandpa: Default::default(),
		pallet_im_online: Default::default(),
		pallet_authority_discovery: moonrabbit::AuthorityDiscoveryConfig { keys: vec![] },
		claims: moonrabbit::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		},
		pallet_vesting: moonrabbit::VestingConfig { vesting: vec![] },
		pallet_treasury: Default::default(),
	}
}

pub fn moonrabbit_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> moonrabbit::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * AAA;
	const STASH: u128 = 100 * AAA;

	moonrabbit::GenesisConfig {
		frame_system: moonrabbit::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_indices: moonrabbit::IndicesConfig { indices: vec![] },
		pallet_balances: moonrabbit::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k| (k.clone(), ENDOWMENT))
				.collect(),
		},
		pallet_session: moonrabbit::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						moonrabbit_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		pallet_staking: moonrabbit::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						moonrabbit::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		pallet_elections_phragmen: Default::default(),
		pallet_democracy: moonrabbit::DemocracyConfig::default(),
		pallet_collective_Instance1: moonrabbit::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_collective_Instance2: moonrabbit::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_membership_Instance1: Default::default(),
		pallet_babe: moonrabbit::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(moonrabbit::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_grandpa: Default::default(),
		pallet_im_online: Default::default(),
		pallet_authority_discovery: moonrabbit::AuthorityDiscoveryConfig { keys: vec![] },
		claims: moonrabbit::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		},
		pallet_vesting: moonrabbit::VestingConfig { vesting: vec![] },
		pallet_treasury: Default::default(),
	}
}

fn moonrabbit_development_config_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	moonrabbit_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

fn moonrabbit_development_config_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	moonrabbit_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}


/// moonrabbit development config (single validator Alice)
pub fn moonrabbit_development_config() -> Result<moonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("moonrabbit development wasm not available")?;

	Ok(moonrabbitChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || moonrabbit_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

pub fn moonrabbit_development_config() -> Result<MoonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("Test development wasm not available")?;

	Ok(MoonrabbitChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || moonrabbit_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn moonrabbit_local_testnet_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	moonrabbit_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// moonrabbit local testnet config (multivalidator Alice + Bob)
pub fn moonrabbit_local_testnet_config() -> Result<moonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("moonrabbit development wasm not available")?;

	Ok(moonrabbitChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || moonrabbit_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

fn moonrabbit_local_testnet_genesis(wasm_binary: &[u8]) -> moonrabbit::GenesisConfig {
	moonrabbit_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
			get_authority_keys_from_seed_no_beefy("Charlie"),
			get_authority_keys_from_seed_no_beefy("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

pub fn moonrabbit_local_testnet_config() -> Result<MoonrabbitChainSpec, String> {
	let wasm_binary = moonrabbit::WASM_BINARY.ok_or("Test development wasm not available")?;

	Ok(MoonrabbitChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || moonrabbit_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}
