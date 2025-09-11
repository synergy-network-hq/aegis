# Aegis Crypto Core - Project Structure

```
.
├── benches                                    {Benchmarking test files for performance evaluation of cryptographic algorithms}
│   ├── classicmceliece_benchmarks.rs          {Performance benchmarks for Classic McEliece post-quantum encryption}
│   ├── performance_benchmarks.rs              {General performance benchmarking suite for all cryptographic algorithms}
│   └── simple_bench.rs                        {Simple benchmarking utilities for basic cryptographic operations}
│
├── .cargo                                     {Cargo configuration directory for Rust build system}
│   └── config.toml                            {Cargo build configuration with target-specific settings and dependencies}
│
├── docs                                       {Documentation files including guides, specifications, and API documentation}
│   ├── cookbook.md                            {Practical examples and recipes for common cryptographic operations}
│   ├── document_signing_identity.md           {Guide for implementing document signing with digital identities}
│   ├── general_purpose_pq_crypto.md           {Overview of general-purpose post-quantum cryptography features}
│   ├── hardware_embedded_integration.md       {Integration guide for hardware and embedded systems}
│   ├── on_chain_smart_contract_use.md         {Documentation for blockchain and smart contract integration}
│   ├── overview.md                            {High-level overview of the Aegis cryptographic library}
│   ├── quickstart.md                          {Quick start guide for getting started with the library}
│   ├── README.md                              {Main documentation index and navigation}
│   ├── secure_messaging.md                    {Guide for implementing secure messaging protocols}
│   ├── tech_spec.md                           {Technical specifications and implementation details}
│   ├── update_document.md                     {Documentation update procedures and guidelines}
│   └── wallet_integration.md                  {Guide for integrating with cryptocurrency wallets}
│
├── examples                                   {Example code demonstrating usage of the cryptographic library}
│   ├── browser_demo.html                      {HTML demo page for browser-based cryptographic operations}
│   ├── browser_demo.js                        {JavaScript code for browser demonstration of crypto features}
│   ├── node_demo.js                           {Node.js example demonstrating server-side cryptographic operations}
│   ├── secure_messaging_api.js                {API example for secure messaging implementation}
│   ├── secure_messaging_browser_demo.html     {Browser demo for secure messaging features}
│   ├── secure_messaging_demo.js               {JavaScript demo for secure messaging functionality}
│   └── wallet_integration_demo.html           {Demo page for wallet integration features}
│
├── fuzz                                       {Fuzzing test files for security testing and bug detection}
│   ├── fuzz_targets                           {Directory containing fuzzing test targets for security validation}
│   │   ├── fuzz_target_1.rs                   {Primary fuzzing target for general cryptographic functions}
│   │   └── kyber_fuzz.rs                      {Specific fuzzing tests for Kyber key encapsulation mechanism}
│   ├── Cargo.toml                             {Fuzzing-specific Cargo configuration and dependencies}
│   └── .gitignore
│
├── .github                                    {GitHub-specific configuration and CI/CD workflows}
│   └── workflows                              {GitHub Actions workflow definitions}
│       └── ci.yml                             {Continuous integration workflow for automated testing and building}
│
├── scripts                                    {Build and utility scripts for development and deployment}
│   ├── build-wasi.sh                          {Shell script for building WebAssembly System Interface targets}
│   └── build-wasm.sh                          {Shell script for building standard WebAssembly targets}
│
├── src                                        {Main source code directory containing all cryptographic implementations}
│   ├── bin                                    {Binary executables and command-line tools}
│   │
│   ├── classicmceliece                        {Classic McEliece post-quantum encryption implementation}
│   │   ├── core.rs                            {Core Classic McEliece encryption and decryption algorithms}
│   │   ├── mod.rs                             {Module definition and public API for Classic McEliece}
│   │   ├── utils.rs                           {Utility functions for Classic McEliece operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for Classic McEliece}
│   │
│   ├── dilithium                              {Dilithium post-quantum digital signature implementation}
│   │   ├── core.rs                            {Core Dilithium signature generation and verification}
│   │   ├── mod.rs                             {Module definition and public API for Dilithium}
│   │   ├── utils.rs                           {Utility functions for Dilithium operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for Dilithium}
│   │
│   ├── falcon                                 {FALCON post-quantum digital signature implementation}
│   │   ├── core.rs                            {Core FALCON signature generation and verification}
│   │   ├── mod.rs                             {Module definition and public API for FALCON}
│   │   ├── utils.rs                           {Utility functions for FALCON operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for FALCON}
│   │
│   ├── hqc                                    {HQC post-quantum encryption implementation}
│   │   ├── core.rs                            {Core HQC encryption and decryption algorithms}
│   │   ├── mod.rs                             {Module definition and public API for HQC}
│   │   ├── utils.rs                           {Utility functions for HQC operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for HQC}
│   │
│   ├── kyber                                  {Kyber post-quantum key encapsulation mechanism}
│   │   ├── core.rs                            {Core Kyber key generation, encapsulation, and decapsulation}
│   │   ├── mod.rs                             {Module definition and public API for Kyber}
│   │   ├── traits.rs                          {Trait definitions for Kyber operations}
│   │   ├── utils.rs                           {Utility functions for Kyber operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for Kyber}
│   │
│   ├── sphincsplus                            {SPHINCS+ post-quantum digital signature implementation}
│   │   ├── core.rs                            {Core SPHINCS+ signature generation and verification}
│   │   ├── mod.rs                             {Module definition and public API for SPHINCS+}
│   │   ├── utils.rs                           {Utility functions for SPHINCS+ operations}
│   │   └── wasm_bindings.rs                   {WebAssembly bindings for SPHINCS+}
│   │
│   ├── blockchain.rs                          {Blockchain-specific cryptographic utilities and integrations}
│   ├── hash.rs                                {Hash function implementations and utilities}
│   ├── js_bindings.rs                         {JavaScript/TypeScript bindings for the cryptographic library}
│   ├── lib.rs                                 {Main library entry point and public API definitions}
│   ├── performance.rs                         {Performance optimization utilities and benchmarks}
│   ├── traits.rs                              {Common trait definitions for cryptographic operations}
│   └── utils.rs                               {General utility functions used across the library}
│
├── tests                                      {Test files for various cryptographic algorithms and implementations}
│   ├── kyber_browser_tests.rs
│   ├── kyber_kat_tests.rs
│   ├── kyber_native_tests.rs
│   ├── kyber_wasm_tests.rs
│   │
│   ├── dilithium_browser_tests.rs
│   ├── dilithium_kat_tests.rs
│   ├── dilithium_native_tests.rs
│   ├── dilithium_wasm_tests.rs
│   │
│   ├── sphincsplus_browser_tests.rs
│   ├── sphincsplus_kat_tests.rs
│   ├── sphincsplus_native_tests.rs
│   ├── sphincsplus_wasm_tests.rs
│   │
│   ├── falcon_browser_tests.rs
│   ├── falcon_kat_tests.rs
│   ├── falcon_native_tests.rs
│   ├── falcon_wasm_tests.rs
│   │
│   ├── hqc_browser_tests.rs
│   ├── hqc_kat_tests.rs
│   ├── hqc_native_tests.rs
│   ├── hqc_wasm_tests.rs
│   │
│   ├── classicmceliece_browser_tests.rs
│   ├── classicmceliece_native_tests.rs
│   ├── classicmceliece_wasm_tests.rs
│   ├── test_classicmceliece.sh                {Test script specifically for Classic McEliece algorithm}
│   ├── test_wasm_classicmceliece.sh           {WebAssembly test script for Classic McEliece}
│   │
│   ├── kat_tests.rs                           {Known Answer Tests for cryptographic algorithm validation}
│   ├── comprehensive_kat_tests.rs             {Comprehensive Known Answer Tests for all algorithms}
│   │
│   ├── test.sh                                {Main test execution script for all algorithms}
│   │
│   ├── README_BROWSER_TESTS.md                {Documentation for browser-based testing procedures}
│   ├── complete_browser_suite_remaining.rs    {Remaining browser test suite for comprehensive coverage}
│   ├── run_browser_tests.sh                   {Shell script to execute browser-based tests}
│   ├── run_wasi_tests.sh                      {Shell script to execute WASI-based tests}
│   ├── test_wasm.sh                           {WebAssembly test execution script}
│   │
│   ├── performance_tests.rs                   {Performance testing suite for algorithm benchmarks}
│   └── trait_tests.rs                         {Tests for trait implementations and generic functionality}
│
├── wasi-sdk-20.0                              {WebAssembly System Interface SDK for cross-platform compilation}
│       └── [. . .]                            {WebAssembly toolchain - compilers, linkers, and system libraries for WASI target}
│
├── Cargo.lock                                 {Rust dependency lock file with exact versions}
├── Cargo.toml                                 {Rust project configuration and dependency specifications}
├── package.json                               {Node.js package configuration for JavaScript/TypeScript bindings}
├── pyproject.toml                             {Python project configuration for Python bindings}
├── README.md                                  {Main project documentation and overview}
└── WASM_BUILD_STATUS.md                       {Documentation of WebAssembly build status and issues}

91 directories, 1458 files (files in the wasi-sdk-20.0 folder omitted for brevity)
