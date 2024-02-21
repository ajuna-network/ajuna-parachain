<p align="center" width="100%">
  <a href="https://ajuna.io" target="_blank">
    <img src="docs/ajuna-banner.jpeg" alt="Ajuna Network">
  </a>
</p>

[![Build](https://github.com/ajuna-network/ajuna-runtimes-ajuna/actions/workflows/check-pull-request.yml/badge.svg?branch=main)](https://github.com/ajuna-network/ajuna-runtimes-ajuna/actions/workflows/check-pull-request.yml)
[![codecov](https://codecov.io/gh/ajuna-network/ajuna-runtimes-ajuna/branch/main/graph/badge.svg?token=V2Y88ZUD6C)](https://codecov.io/gh/ajuna-network/ajuna-runtimes-ajuna)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/ajuna/parachain-bajun?label=bajun%20network&logo=docker&sort=semver&style=plastic)](https://hub.docker.com/repository/docker/ajuna/parachain-bajun/tags?page=1&ordering=last_updated)

A game platform [parachain](https://wiki.polkadot.network/docs/learn-parachains) built with [Substrate](https://docs.substrate.io/).

## Chopsticks
Chopsticks can be used to create a local fork of a life network, copying all its storage values and override specific 
storage values with a config file. The fork will create a block whenever an extrinsic is added to the tx-pool but
it will be stale otherwise, but exposes a fast-forward rpc to produce 'n' blocks, which is useful for scheduler testing.

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

### Test XCM stuf

TBD when we actually start setting up XCM.

### Test Governance execution 

TBD when we have governance