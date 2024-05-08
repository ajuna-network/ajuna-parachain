use crate::chain_spec_utils::{AjunaKeys, GenesisKeys, RelayChain, TestnetDevKeys, WellKnownKeys};
use ajuna_runtime::{AccountId, AuraId, EXISTENTIAL_DEPOSIT};
use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

const SS58_FORMAT: u32 = 1328;
/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = staging_xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> ajuna_runtime::SessionKeys {
	ajuna_runtime::SessionKeys { aura: keys }
}

pub fn ajuna_chain_spec(
	para_id: ParaId,
	genesis_keys: GenesisKeys,
	relay_chain: RelayChain,
) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "AJUN".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	let (root, endowed, invulnerables, gov) = match genesis_keys {
		GenesisKeys::Ajuna => (
			AjunaKeys::root(),
			vec![AjunaKeys::root()],
			AjunaKeys::invulnerables(),
			AjunaKeys::governance(),
		),
		GenesisKeys::TestnetDev => (
			TestnetDevKeys::root(),
			vec![TestnetDevKeys::root()],
			TestnetDevKeys::invulnerables(),
			TestnetDevKeys::governance(),
		),
		GenesisKeys::WellKnown => (
			WellKnownKeys::root(),
			WellKnownKeys::endowed(),
			WellKnownKeys::invulnerables(),
			WellKnownKeys::governance(),
		),
	};

	#[allow(deprecated)]
	ChainSpec::builder(
		ajuna_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: relay_chain.to_string(), para_id: para_id.into() },
	)
	.with_name("Ajuna")
	.with_id(&format!("ajuna-{}", relay_chain.to_string()))
	.with_protocol_id(relay_chain.protocol_id())
	.with_chain_type(relay_chain.chain_type())
	.with_properties(properties)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		invulnerables,
		endowed,
		root,
		gov,
		para_id,
	))
	.build()
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
	governance_accounts: Vec<AccountId>,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"council": {
			"members": governance_accounts
		},
		"technicalCommittee": {
			"members": governance_accounts
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
			.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": { "key": Some(root) }
	})
}

pub fn ajuna_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../resources/ajuna/ajuna-raw.json")[..])
}

pub fn ajuna_westend_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(
		&include_bytes!("../../resources/ajuna/westend/ajuna-westend-raw.json")[..],
	)
}

pub fn ajuna_paseo_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(
		&include_bytes!("../../resources/ajuna/paseo/ajuna-paseo-raw.json")[..],
	)
}
