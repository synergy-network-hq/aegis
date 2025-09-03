# Test Execution Guide

This document describes how to run all native and WASM tests in this repository, along with the environment setup required for each.

## Prerequisites

1) Rust targets
- Native tests run on the host toolchain (no extra target needed).
- WASM tests require:
  - rustup target add wasm32-wasip1

2) WASI SDK (for C code used by pqcrypto-internals)
- Install WASI SDK 22.0 (or compatible) and set:
  - export WASI_SDK_DIR="$HOME/wasi-sdk-22.0"
  - export WASI_SYSROOT="$WASI_SDK_DIR/share/wasi-sysroot"
- Sanity check:
  - test -x "$WASI_SDK_DIR/bin/clang"
  - test -d "$WASI_SYSROOT/include"

3) Wasmtime (WASI runtime)
- Install a local Wasmtime (no sudo):
  - mkdir -p "$HOME/.wasmtime/bin"
  - Download prebuilt (example):
    - curl -fL https://github.com/bytecodealliance/wasmtime/releases/download/v19.0.2/wasmtime-v19.0.2-x86_64-linux.tar.xz -o /tmp/wasmtime.tar.xz
    - tar -xJf /tmp/wasmtime.tar.xz -C /tmp
    - cp /tmp/wasmtime-*/wasmtime "$HOME/.wasmtime/bin/wasmtime"
    - chmod +x "$HOME/.wasmtime/bin/wasmtime"
  - export PATH="$HOME/.wasmtime/bin:$PATH"
  - wasmtime --version

4) Project aliases (optional but recommended)
- The following aliases are already configured in [.cargo/config.toml](.cargo/config.toml):
  - cargo test-wasm
  - cargo list-wasm
  - cargo test-native-sphincs

## Running All Native Tests

Native tests use the host runner. There are two useful modes:

A) Default features (runs most native tests; SPHINCS+ native tests are excluded when wasm/js-bindings are enabled by default)
- Command:
  - cargo test -p aegis_crypto_core --tests

B) SPHINCS+ native tests without wasm/js-bindings (prevents wasm-bindgen from being used on native)
- Command:
  - cargo test -p aegis_crypto_core --tests --no-default-features --features "sphincsplus"
- Alias:
  - cargo test-native-sphincs

Notes:
- classicmceliece native tests require the "classicmceliece" feature to be enabled. In this project, that feature is wired for dev builds via [aegis_crypto_core/Cargo.toml](aegis_crypto_core/Cargo.toml). If needed manually:
  - cargo test -p aegis_crypto_core --tests --no-default-features --features "classicmceliece"

## Running All WASM Tests

WASM tests are compiled to wasm32-wasip1 and must be executed with a WASI runtime (Wasmtime). The test files intended for WASM are named *_wasm_tests.rs and gated with #[cfg(target_arch = "wasm32")]. Native test artifacts should not be executed under WASI.

There are two steps: build and run.

A) Build WASM test artifacts
- Ensure environment:
  - export WASI_SDK_DIR="$HOME/wasi-sdk-22.0"
  - export WASI_SYSROOT="$WASI_SDK_DIR/share/wasi-sysroot"
- Build tests:
  - cargo test -p aegis_crypto_core --no-run --target wasm32-wasip1 --features wasm
- Alias to list tests (optional):
  - cargo list-wasm

B) Run only WASM test artifacts using Wasmtime
- Use the helper script:
  - export PATH="$HOME/.wasmtime/bin:$PATH"
  - export WASI_SDK_DIR="$HOME/wasi-sdk-22.0"
  - bash aegis_crypto_core/tests/run_wasi_tests.sh
- What the script does:
  - Verifies wasm32-wasip1 target is installed
  - Verifies WASI_SDK_DIR and Wasmtime are available
  - Builds tests for the wasm target with feature “wasm”
  - Runs only artifacts matching:
    - target/wasm32-wasip1/debug/deps/*wasm_tests-*.wasm
  - Skips *_native_tests-*.wasm to avoid WASI-incompatible threading/stack manipulations

## Why Some Tests Don’t Run in Certain Modes

- Files like dilithium_wasm_tests.rs, falcon_wasm_tests.rs, sphincsplus_wasm_tests.rs are gated with #[cfg(target_arch = "wasm32")]. They won’t run under plain cargo test on the host.
- Kyber wasm tests have been updated to match this pattern and no longer run on native by accident.
- SPHINCS+ native tests are guarded to avoid wasm-bindgen on non-wasm targets. If default features include "wasm", those native tests are filtered out; run them with:
  - cargo test -p aegis_crypto_core --tests --no-default-features --features "sphincsplus"

## Troubleshooting

- Error: missing environment variable: WASI_SDK_DIR
  - Set: export WASI_SDK_DIR="$HOME/wasi-sdk-22.0"
  - Ensure clang exists at $WASI_SDK_DIR/bin/clang and sysroot at $WASI_SDK_DIR/share/wasi-sysroot

- Exec format error when running cargo test --target wasm32-wasip1
  - Use Wasmtime (or another WASI runtime) to execute the resulting .wasm test artifacts.
  - Prefer: bash aegis_crypto_core/tests/run_wasi_tests.sh

- SPHINCS+ panics on native
  - Ensure wasm/js-bindings features are not enabled for native SPHINCS+ tests:
    - cargo test -p aegis_crypto_core --tests --no-default-features --features "sphincsplus"

## Quick Commands Summary

Native
- All default native tests:
  - cargo test -p aegis_crypto_core --tests
- SPHINCS+ native only (no wasm/js-bindings):
  - cargo test -p aegis_crypto_core --tests --no-default-features --features "sphincsplus"
  - or: cargo test-native-sphincs

WASM
- Build artifacts:
  - cargo test-wasm --no-run
  - or: cargo test -p aegis_crypto_core --no-run --target wasm32-wasip1 --features wasm
- Run artifacts with Wasmtime:
  - bash aegis_crypto_core/tests/run_wasi_tests.sh
