#!/bin/bash
set -e

RUNTIME="ajuna"

# common pallets shared by both networks
PALLETS=(
  "cumulus-pallet-xcmp-queue"
  "frame-system"
  "pallet-balances"
  "pallet-collator-selection"
  "pallet-collective"
  "pallet-identity"
  "pallet-membership"
  "pallet-multisig"
  "pallet-nfts"
  "pallet-preimage"
  "pallet-proxy"
  "pallet-scheduler"
  "pallet-session"
  "pallet-timestamp"
  "pallet-treasury"
  "pallet-utility"
)

cd "$(git rev-parse --show-toplevel)" || exit
cargo build-"${RUNTIME}"-benchmarks --features "experimental"

for PALLET in "${PALLETS[@]}"; do
  ./target/release/"${RUNTIME}"-para benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    --pallet="${PALLET}" \
    --extrinsic="*" \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --header="./HEADER-AGPL" \
    --output="./runtime/${RUNTIME}/src/weights/${PALLET//-/_}.rs"
done

# custom pallets for ajuna network
[ "${RUNTIME}" != "ajuna" ] && exit 0
CUSTOM_PALLETS=(
  "pallet-ajuna-awesome-avatars"
)
for PALLET in "${CUSTOM_PALLETS[@]}"; do
  ./target/release/"${RUNTIME}"-para benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    --pallet="${PALLET}" \
    --extrinsic="*" \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --template="./.maintain/frame-weight-template.hbs" \
    --output="./runtime/${RUNTIME}/src/weights/${PALLET//-/_}.rs"
done
