// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use ajuna::{ajuna_chain_spec, ajuna_config, ajuna_paseo_config, ajuna_westend_config};
use chain_spec_utils::{GenesisKeys, RelayChain};
use polkadot_omni_node_lib::{
	chain_spec::{GenericChainSpec, LoadSpec},
	runtime::{
		AuraConsensusId, BlockNumber, Consensus, Runtime, RuntimeResolver as RuntimeResolverT,
	},
};
use sc_cli::ChainSpec;

mod ajuna;
mod chain_spec_utils;

const POLKADOT_PARA_ID: u32 = 2051;
const PASEO_PARA_ID: u32 = 2051;
const WESTEND_PARA_ID: u32 = 2051;
const LOCAL_PARA_ID: u32 = 2051;

#[derive(Debug)]
pub(crate) struct ChainSpecLoader;

impl LoadSpec for ChainSpecLoader {
	// If we don't skip here, each cmd expands to 5 lines. I think we have better overview like
	// this.
    #[rustfmt::skip]
	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		Ok(match id {
			// live configs
			"ajuna-polkadot" => Box::new(ajuna_config()?),
			"ajuna-paseo" => Box::new(ajuna_paseo_config()?),
			"ajuna-westend" => Box::new(ajuna_westend_config()?),

			// fresh production/testnet chain-specs based on the current rust code
			"ajuna-polkadot-fresh" => Box::new(ajuna_chain_spec(POLKADOT_PARA_ID.into(), GenesisKeys::Ajuna, RelayChain::Polkadot)),
			"ajuna-paseo-fresh" => Box::new(ajuna_chain_spec(PASEO_PARA_ID.into(), GenesisKeys::TestnetDev, RelayChain::Paseo)),
			"ajuna-westend-fresh" => Box::new(ajuna_chain_spec(WESTEND_PARA_ID.into(), GenesisKeys::TestnetDev, RelayChain::Westend)),

			// rust code based configs for a local setup
			"" | "ajuna-rococo-local" => Box::new(ajuna_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::RococoLocal)),

			path => Box::new(GenericChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		})
	}
}

#[derive(Debug)]
pub(crate) struct RuntimeResolver;

impl RuntimeResolverT for RuntimeResolver {
	fn runtime(&self, _: &dyn ChainSpec) -> sc_cli::Result<Runtime> {
		Ok(Runtime::Omni(BlockNumber::U32, Consensus::Aura(AuraConsensusId::Sr25519)))
	}
}
