//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
mod chain_spec_utils;
mod cli;
mod command;
mod fake_runtime_api;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
	command::run()
}
