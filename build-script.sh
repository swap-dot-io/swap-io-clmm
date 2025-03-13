#!/usr/bin/bash
# This builds a verifieable solana program. To be run within docker.

cargo build-bpf --manifest-path Cargo.toml
mkdir -p docker-target
mkdir -p docker-target/idl
mkdir -p docker-target/types
anchor idl parse --file programs/amm/src/lib.rs -o target/idl/swap-io-clmm-idl.json -t target/types/swap-io-clmm-idl.ts
