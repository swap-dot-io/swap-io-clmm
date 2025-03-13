#!/usr/bin/bash
cargo build-bpf --manifest-path Cargo.toml
mkdir -p target
mkdir target/idl
mkdir target/types
anchor idl parse --file programs/amm/src/lib.rs -o target/idl/swap-io-clmm-idl.json -t target/types/swap-io-clmm-idl.ts
