use ajuna_runtime::{AccountId, AuraId};
use sc_chain_spec::ChainType;
use sp_core::{crypto::Ss58Codec, sr25519, Public};
use sp_keyring::AccountKeyring::{Alice, Bob, Charlie, Dave, Eve, Ferdie};
use std::str::FromStr;

pub fn pub_sr25519(ss58: &str) -> sr25519::Public {
	public_from_ss58::<sr25519::Public>(ss58)
}

pub fn public_from_ss58<TPublic: Public + FromStr>(ss58: &str) -> TPublic
where
	<TPublic as FromStr>::Err: std::fmt::Debug,
{
	TPublic::from_ss58check(ss58).expect("supply valid ss58!")
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GenesisKeys {
	/// Use Ajuna production keys.
	Ajuna,
	/// Keys intended for testnets like westend, or paseo.
	TestnetDev,
	/// Use keys from the keyring for a test setup.
	WellKnown,
}

pub struct WellKnownKeys;

impl WellKnownKeys {
	pub fn root() -> AccountId {
		Alice.to_account_id()
	}

	pub fn endowed() -> Vec<AccountId> {
		vec![
			Alice.to_account_id(),
			Bob.to_account_id(),
			Charlie.to_account_id(),
			Dave.to_account_id(),
			Eve.to_account_id(),
			Ferdie.to_account_id(),
		]
	}

	pub fn governance() -> Vec<AccountId> {
		vec![Alice.to_account_id(), Bob.to_account_id(), Charlie.to_account_id()]
	}

	pub fn invulnerables() -> Vec<(AccountId, AuraId)> {
		vec![
			(Alice.public().into(), Alice.public().into()),
			(Bob.public().into(), Bob.public().into()),
		]
	}
}

/// Ajuna and Bajun share the same set of keys here.
pub struct TestnetDevKeys;

impl TestnetDevKeys {
	pub fn root() -> AccountId {
		pub_sr25519("5H6WjuXTrFTpiAmr2Pohzbuj7EvBHuDNht7PSSUCCDv9u4ec").into()
	}
	pub fn invulnerables() -> Vec<(AccountId, AuraId)> {
		vec![
			// Col1-BUBBLEBOBBLE
			(
				pub_sr25519("5FAJtuxRspz76JGHCERLQR8iMoA66kCbJv4G8SSgHJpdmyDG").into(),
				pub_sr25519("5FAJtuxRspz76JGHCERLQR8iMoA66kCbJv4G8SSgHJpdmyDG").into(),
			),
			// Col2-LEMMINGS
			(
				pub_sr25519("5GWyisa8R3h3vmfkv1HVWi3n49xotnNhdCPnfBcn9QD2jwP1").into(),
				pub_sr25519("5GWyisa8R3h3vmfkv1HVWi3n49xotnNhdCPnfBcn9QD2jwP1").into(),
			),
		]
	}
	pub fn governance() -> Vec<AccountId> {
		vec![Self::root()]
	}
}

pub struct AjunaKeys;

impl AjunaKeys {
	pub fn root() -> AccountId {
		pub_sr25519("13EnS5vY7i1rvF7NqMniipYpqtbbUp3burkeYEEp6RvLMwGz").into()
	}
	pub fn invulnerables() -> Vec<(AccountId, AuraId)> {
		vec![
			(
				pub_sr25519("14NC15QdLuHwYFPUM1x87vsh15EsvPDtLhZhvb5vExHkCHPa").into(),
				pub_sr25519("14NC15QdLuHwYFPUM1x87vsh15EsvPDtLhZhvb5vExHkCHPa").into(),
			),
			(
				pub_sr25519("13GbXQAWfLGx9bAuRVN3zdHwL2vs94PKzbTLfqyUfoaB88Lo").into(),
				pub_sr25519("13GbXQAWfLGx9bAuRVN3zdHwL2vs94PKzbTLfqyUfoaB88Lo").into(),
			),
		]
	}
	pub fn governance() -> Vec<AccountId> {
		vec![Self::root()]
	}
}

pub enum RelayChain {
	Kusama,
	Paseo,
	Westend,
	KusamaLocal,
	PaseoLocal,
	RococoLocal,
	WestendLocal,
}

impl RelayChain {
	pub fn id(&self) -> &'static str {
		match self {
			RelayChain::Kusama => "kusama",
			RelayChain::Paseo => "paseo",
			RelayChain::Westend => "westend",
			RelayChain::KusamaLocal => "kusama-local",
			RelayChain::PaseoLocal => "paseo-local",
			RelayChain::RococoLocal => "rococo-local",
			RelayChain::WestendLocal => "westend-local",
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			RelayChain::Kusama => "Kusama",
			RelayChain::Paseo => "Paseo",
			RelayChain::Westend => "Westend",
			RelayChain::KusamaLocal => "Kusama-local",
			RelayChain::PaseoLocal => "Paseo-local",
			RelayChain::RococoLocal => "Rococo-local",
			RelayChain::WestendLocal => "Westend-local",
		}
	}

	pub(crate) fn chain_type(&self) -> ChainType {
		match self {
			RelayChain::Kusama => ChainType::Live,
			RelayChain::Paseo => ChainType::Live,
			RelayChain::Westend => ChainType::Live,
			RelayChain::KusamaLocal => ChainType::Local,
			RelayChain::PaseoLocal => ChainType::Local,
			RelayChain::RococoLocal => ChainType::Local,
			RelayChain::WestendLocal => ChainType::Local,
		}
	}
	pub(crate) fn protocol_id(&self) -> &str {
		match self {
			RelayChain::Kusama => "ajuna-k",
			RelayChain::Paseo => "ajuna-pas",
			RelayChain::Westend => "ajuna-w",
			RelayChain::KusamaLocal => "ajuna-kl",
			RelayChain::PaseoLocal => "ajuna-pasl",
			RelayChain::RococoLocal => "ajuna-rl",
			RelayChain::WestendLocal => "ajuna-wl",
		}
	}
}
