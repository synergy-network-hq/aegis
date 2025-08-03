#!/bin/sh
echo "Running tests with 9MB stack (RUST_MIN_STACK=9437184)..."
export RUST_MIN_STACK=9437184
cargo test "$@"
