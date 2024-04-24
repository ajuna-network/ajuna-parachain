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
	/// Use Bajun production keys.
	Ajuna,
	/// Use Bajun Dev keys, intended for test networks.
	AjunaDev,
	/// Use Keys from the keyring for a test setup
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

// Todo: add ajuna-westend-keys
pub struct BajunDevKeys;

impl BajunDevKeys {
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

pub struct BajunKeys;

impl BajunKeys {
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
	Westend,
	KusamaLocal,
	RococoLocal,
	WestendLocal,
}

impl ToString for RelayChain {
	fn to_string(&self) -> String {
		match self {
			RelayChain::Kusama => "kusama".into(),
			RelayChain::Westend => "ajuna-westend".into(),
			RelayChain::KusamaLocal => "kusama-local".into(),
			RelayChain::RococoLocal => "rococo-local".into(),
			RelayChain::WestendLocal => "ajuna-westend-local".into(),
		}
	}
}

impl RelayChain {
	pub(crate) fn chain_type(&self) -> ChainType {
		match self {
			RelayChain::Kusama => ChainType::Live,
			RelayChain::Westend => ChainType::Live,
			RelayChain::KusamaLocal => ChainType::Local,
			RelayChain::RococoLocal => ChainType::Local,
			RelayChain::WestendLocal => ChainType::Local,
		}
	}
	pub(crate) fn protocol_id(&self) -> &str {
		match self {
			RelayChain::Kusama => "ajuna-k",
			RelayChain::Westend => "ajuna-w",
			RelayChain::KusamaLocal => "ajuna-kl",
			RelayChain::RococoLocal => "ajuna-rl",
			RelayChain::WestendLocal => "ajuna-wl",
		}
	}
}
