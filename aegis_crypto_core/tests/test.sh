#!/bin/sh
echo "Running native PQC tests with 8MB stack..."
echo "Note: Classic McEliece is disabled by default. Use --features classicmceliece to enable it."
export RUST_MIN_STACK=8388608
cargo test \
  --test dilithium_native_tests \
  --test falcon_native_tests \
  --test hqc_native_tests \
  --test kyber_native_tests \
  --test sphincsplus_native_tests \
  "$@"
