//! Helper module to build a genesis configuration for the super-runtime

use super::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, SessionConfig, StakingConfig, StakerStatus, MainnetCollectiveConfig,
	TechnicalCollectiveConfig, PBTCCollectiveConfig
};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::{Perbill};
use crate::Balance;
use crate::opaque::SessionKeys;

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId,AccountId,AccountId) {
	(
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		account_id_from_seed::<sr25519::Public>(seed)
	)
}
pub fn dev_genesis() -> GenesisConfig {
	testnet_genesis(
		// Initial Authorities
		vec![authority_keys_from_seed("Alice")],
		// Root Key
		account_id_from_seed::<sr25519::Public>("Alice"),
		// Endowed Accounts
		vec![
			account_id_from_seed::<sr25519::Public>("Alice"),
			account_id_from_seed::<sr25519::Public>("Bob"),
			account_id_from_seed::<sr25519::Public>("Alice//stash"),
			account_id_from_seed::<sr25519::Public>("Bob//stash"),
		],
	)
}

/// Helper function to build a genesis configuration
pub fn testnet_genesis(
	initial_authorities: Vec<(BabeId, GrandpaId,AccountId,AccountId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	const STASH: Balance = 100;
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		sudo: Some(SudoConfig { key: root_key }),
		babe: Some(BabeConfig {
			// authorities: initial_authorities
			// 	.iter()
			// 	.map(|x| (x.0.clone(), 1))
			// 	.collect(),
			authorities : vec![]
		}),
		grandpa: Some(GrandpaConfig {
			// authorities: initial_authorities
			// 	.iter()
			// 	.map(|x| (x.1.clone(), 1))
			// 	.collect(),
			authorities : vec![]
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.2.clone(), x.2.clone(), session_keys(
					x.1.clone(),
					x.0.clone()
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.2.clone(), x.3.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.2.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_collective_Instance1: Some(MainnetCollectiveConfig{
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(TechnicalCollectiveConfig{
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance3: Some(PBTCCollectiveConfig{
			members: vec![],
			phantom: Default::default(),
		}),
	}
}
fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
) -> SessionKeys {
	SessionKeys { grandpa, babe  }
}
