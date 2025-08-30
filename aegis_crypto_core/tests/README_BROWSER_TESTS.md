Browser WASM tests for Aegis Crypto Core

Overview
This project now includes browser-based WebAssembly tests using wasm-bindgen-test on the wasm32-unknown-unknown target. Tests run headlessly in Chrome or Firefox and cover all six algorithms: Classic McEliece, Kyber, Dilithium, Falcon, SPHINCS+, and HQC.

Files added
- tests/*_browser_tests.rs: Browser-focused test suites per algorithm
  - classicmceliece_browser_tests.rs
  - kyber_browser_tests.rs
  - dilithium_browser_tests.rs
  - falcon_browser_tests.rs
  - sphincsplus_browser_tests.rs
  - hqc_browser_tests.rs
- tests/run_browser_tests.sh: Helper script to run all browser tests headlessly
- .cargo/config.toml alias:
  - test-wasm-browser = test -p aegis_crypto_core --target wasm32-unknown-unknown --features wasm,js-bindings -- --test-threads=1

Prerequisites
1) Install target and dev tools
   - rustup target add wasm32-unknown-unknown
   - wasm-bindgen-test is already declared under [dev-dependencies] in Cargo.toml

2) Ensure a headless browser is available in PATH
   - Chrome/Chromium: one of chromium, chromium-browser, google-chrome
   - Firefox: firefox
   - Optional: export WASM_BINDGEN_TEST_BROWSER=chrome or firefox to force a specific browser

How to run
Option A: Via cargo alias
  cargo test-wasm-browser

Option B: Using the helper script
  bash aegis_crypto_core/tests/run_browser_tests.sh

What runs
- Only tests compiled for the wasm32-unknown-unknown target with js-bindings and annotated with wasm_bindgen_test in the *_browser_tests.rs files.
- These tests are separate from the WASI suite (run with wasmtime) and from native tests.

Troubleshooting
- No browser found:
  Install chromium or firefox and ensure it is on PATH, or set WASM_BINDGEN_TEST_BROWSER to a browser that is installed.
- Tests hang or flake:
  The command is already single-threaded; ensure your browser supports headless mode (most do). Try switching browsers via WASM_BINDGEN_TEST_BROWSER.
- Build errors referencing missing wasm-bindgen/web-sys:
  Ensure you passed features "wasm,js-bindings" and are targeting wasm32-unknown-unknown.
  Example:
    cargo test -p aegis_crypto_core --target wasm32-unknown-unknown --features "wasm,js-bindings" -- --test-threads=1

CI considerations
- Install a headless browser in CI (e.g., apt-get install chromium or firefox)
- rustup target add wasm32-unknown-unknown
- cargo test -p aegis_crypto_core --target wasm32-unknown-unknown --features "wasm,js-bindings" -- --test-threads=1

Notes
- WASI tests remain runnable separately via tests/run_wasi_tests.sh and the test-wasm alias.
- Browser tests purposefully exclude the WASI target using cfg(all(target_arch = "wasm32", not(target_os = "wasi"))).
