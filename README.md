<p align="center" width="100%">
  <a href="https://ajuna.io" target="_blank">
    <img src="docs/ajuna-banner.jpeg" alt="Ajuna Network">
  </a>
</p>

[![Build](https://github.com/ajuna-network/ajuna-parachain/actions/workflows/check-pull-request.yml/badge.svg?branch=develop)](https://github.com/ajuna-network/ajuna-parachain/actions/workflows/check-pull-request.yml)
[![codecov](https://codecov.io/gh/ajuna-network/ajuna-parachain/branch/main/graph/badge.svg?token=V2Y88ZUD6C)](https://codecov.io/gh/ajuna-network/ajuna-parachain)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/ajuna/parachain-ajuna?label=ajuna%20network&logo=docker&sort=semver&style=plastic)](https://hub.docker.com/repository/docker/ajuna/parachain-ajuna/tags?page=1&ordering=last_updated)

A game platform [parachain](https://wiki.polkadot.network/docs/learn-parachains) built
with [Substrate](https://docs.substrate.io/).

## Chopsticks

Chopsticks can be used to create a local fork of a life network, copying all its storage values and override specific
storage values with a config file. The fork will create a block whenever an extrinsic is added to the tx-pool.
Otherwise, it will be stale, but exposes a fast-forward rpc to produce 'n' blocks, which is useful for scheduler
testing.

Chopsticks can be used to:

* Test runtime upgrades and functionality thereafter
* Setup XCM with multiple networks and allow for XCM testing
* Test execution of scheduled task, which is mostly execution of governance proposals.

**Preliminary Note**
Chopsticks will by default listen on the IPV6 localhost port 8000, which means we can either access it with:

* ws://localhost:8000
* ws://[::1]:8000

but ws://127.0.0.1:8000 doesn't work.

### Test Runtime Upgrades

Note: Currently, try-runtime seems to be a better solution to test the runtime upgrade itself, but chopsticks will be
good to test functionality after the runtime upgrade happened.

The following command overrides the runtime with the local build of the upgraded runtime and will run the migration
upon the next block, which means as soon as we send an arbitrary extrinsic.

```bash
nvm use 20
npx @acala-network/chopsticks@latest --config ./chopsticks/ajuna.yml  --wasm-override ./target/release/wbuild/ajuna-runtime/ajuna_runtime.compact.compressed.wasm
```

Then we can send an arbitrary extrinsic to trigger the next block and observe the logs of the runtime upgrade.

### Test XCM stuff

TBD when we actually start setting up XCM.

### Test Governance execution

Simply create an external proposal and fast-track it with a small voting period and delay, e.g. 10 blocks. Then, in
polkadot-js/apps, go to Developer -> JavaScript and execute the following chopsticks-dev-rpc:
`api.rpc('dev_newBlock', { count: 10 })`. After this you can observe the blocks being created one by one until the
proposal is executed.

### Creating new production network chainspecs.

We have parametrized chain-spec generation to simplify creating new production network chain-specs without manual
copy-pasting.
This helps to prevent human errors, and the code also serves as documentation at the same time. How to create a new
production
chain-spec:

1. Add a new `*-fresh` entry in the [node/src/command], e.g., `ajuna-westend-fresh`.
2. Ensure that the collator and the runtime is of the most recent build, and run the
   script: `scripts/dump_wasm_state_and_spec.sh ajuna-westend-fresh`.
3. Copy the `chain_dumps/ajuna-westend-fresh-raw.json`/`-.state`/`-.wasm` into `resources/ajuna-westend`.
4. Rename the files to remove the `-fresh` suffix.
5. Delete the unsorted chain-spec.
6. Add a new function into `node/src/chain_spec.rs`

```rust
pub fn ajuna_westend_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../resources/ajuna/ajuna-westend-raw.json")[..])
}
```

7. Refer to the new function in `node/src/command.rs`.
## Build

- Using `cargo`:

  ```bash
  # parachain with Ajuna runtime
  cargo build --release
  ```
- Using `Docker`:

  ```bash
  # parachain with Ajuna runtime
  docker build -f docker/Dockerfile -t ajuna/parachain-ajuna:latest . --build-arg bin=ajuna-node
  ```
- Ajuna specific parameters:
 RPC
  ```bash
          - --name=ajuna-rpc
          - --base-path=/data/ajuna
          - --chain=/ajuna/ajuna-raw.json
          - --port=30333
          - --unsafe-rpc-external
          - --rpc-port=9944
          - --rpc-external
          - --rpc-methods=Safe
          - --rpc-cors=all
          - --bootnodes=/ip4/178.62.47.42/tcp/30333/ws/p2p/12D3KooWQSuL51Ki3sydbCwPuvnFx6iNQthTbSjTts3zKMWt1bz8
          - --rpc-max-connections=50000
          - --prometheus-external
          - --telemetry-url={{ telemetry_url }}
  ```
