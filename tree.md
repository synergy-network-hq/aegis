.
├── benches
│   ├── classicmceliece_benchmarks.rs
│   ├── performance_benchmarks.rs
│   └── simple_bench.rs
├── Cargo.lock
├── Cargo.toml
├── config
│   ├── package.json
│   ├── pyproject.toml
│   └── wasm-pack.toml
├── demos
│   ├── advanced-messaging
│   │   ├── advanced-messaging.html
│   │   └── advanced-messaging.js
│   ├── blockchain-wallet
│   │   ├── blockchain-wallet.html
│   │   └── blockchain-wallet.js
│   ├── blockchain_wallet_example.md
│   ├── demo-docs
│   ├── demo-scripts
│   │   ├── README.md
│   │   └── start_portal.sh
│   ├── digital-identity
│   │   ├── digital-identity.html
│   │   └── digital-identity.js
│   ├── document-signing
│   │   ├── document-signing.html
│   │   └── document-signing.js
│   ├── FINAL_REORGANIZATION_SUMMARY.md
│   ├── financial-security
│   │   ├── financial-security.html
│   │   └── financial-security.js
│   ├── government-communications
│   │   ├── government-communications.html
│   │   └── government-communications.js
│   ├── healthcare-data-protection
│   │   ├── healthcare-data-protection.html
│   │   └── healthcare-data-protection.js
│   ├── interactive-learning
│   │   ├── interactive-learning.html
│   │   └── interactive-learning.js
│   ├── iot-security
│   │   ├── iot-security.html
│   │   └── iot-security.js
│   ├── ml-model-protection
│   │   ├── ml-model-protection.html
│   │   └── ml-model-protection.js
│   ├── nist_wasm_demo.html
│   ├── nist_wasm_demo.js
│   ├── portal
│   │   ├── portal.css
│   │   ├── portal.html
│   │   └── portal.js
│   ├── post-quantum-blockchain
│   │   ├── post-quantum-blockchain.html
│   │   └── post-quantum-blockchain.js
│   ├── post-quantum-cloud-storage
│   │   ├── post-quantum-cloud-storage.html
│   │   └── post-quantum-cloud-storage.js
│   ├── post-quantum-database
│   │   ├── post-quantum-database.html
│   │   └── post-quantum-database.js
│   ├── quantum-key-distribution
│   │   ├── quantum-key-distribution.html
│   │   └── quantum-key-distribution.js
│   ├── quantum-resistant-iot
│   │   ├── quantum-resistant-iot.html
│   │   └── quantum-resistant-iot.js
│   ├── quantum-resistant-vpn
│   │   ├── quantum-resistant-vpn.html
│   │   └── quantum-resistant-vpn.js
│   ├── README.md
│   ├── real-time-crypto
│   │   ├── real-time-crypto.html
│   │   └── real-time-crypto.js
│   ├── REORGANIZATION_SUMMARY.md
│   ├── sample_programs_guide.md
│   ├── secure-messaging
│   │   ├── secure-messaging.html
│   │   └── secure-messaging.js
│   ├── secure_messaging_example.md
│   ├── secure-voting-system
│   │   ├── secure-voting-system.html
│   │   └── secure-voting-system.js
│   ├── smart-contract-security
│   │   ├── smart-contract-security.html
│   │   └── smart-contract-security.js
│   ├── supply-chain-security
│   │   ├── supply-chain-security.html
│   │   └── supply-chain-security.js
│   └── working_examples_summary.md
├── fuzz
│   ├── Cargo.toml
│   └── fuzz_targets
│       ├── fuzz_target_1.rs
│       └── kyber_fuzz.rs
├── pkg
│   ├── aegis_crypto_core_bg_original.wasm
│   ├── aegis_crypto_core_bg.wasm
│   ├── aegis_crypto_core.js
│   ├── package.json
│   └── README.md
├── README.md
├── src
│   ├── bin
│   │   ├── blockchain_wallet.rs
│   │   ├── digital_identity.rs
│   │   ├── document_signing.rs
│   │   ├── financial_security.rs
│   │   ├── iot_security.rs
│   │   ├── secure_messaging.rs
│   │   └── web_api_server.rs
│   ├── blockchain.rs
│   ├── classicmceliece
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── dilithium
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── falcon
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── hash.rs
│   ├── hqc
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── js_bindings.rs
│   ├── kyber
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── lib.rs
│   ├── nist_wasm_mldsa.rs
│   ├── nist_wasm_mlkem.rs
│   ├── performance.rs
│   ├── rustpqc_dilithium
│   │   └── mod.rs
│   ├── rustpqc_kyber
│   │   └── mod.rs
│   ├── sphincsplus
│   │   ├── core.rs
│   │   ├── mod.rs
│   │   ├── utils.rs
│   │   └── wasm_bindings.rs
│   ├── traits.rs
│   ├── utils.rs
│   └── wasm_loader.rs
├── tests
│   ├── classicmceliece_browser_tests.rs
│   ├── classicmceliece_native_tests.rs
│   ├── classicmceliece_wasm_tests.rs
│   ├── complete_browser_suite_remaining.rs
│   ├── comprehensive_kat_tests.rs
│   ├── dilithium_browser_tests.rs
│   ├── dilithium_kat_tests.rs
│   ├── dilithium_native_tests.rs
│   ├── dilithium_wasm_tests.rs
│   ├── falcon_browser_tests.rs
│   ├── falcon_kat_tests.rs
│   ├── falcon_native_tests.rs
│   ├── falcon_wasm_tests.rs
│   ├── hqc_browser_tests.rs
│   ├── hqc_kat_tests.rs
│   ├── hqc_native_tests.rs
│   ├── hqc_wasm_tests.rs
│   ├── kat_tests.rs
│   ├── kyber_browser_tests.rs
│   ├── kyber_kat_tests.rs
│   ├── kyber_native_tests.rs
│   ├── kyber_wasm_tests.rs
│   ├── performance_tests.rs
│   ├── README_BROWSER_TESTS.md
│   ├── run_browser_tests.sh
│   ├── run_wasi_tests.sh
│   ├── sphincsplus_browser_tests.rs
│   ├── sphincsplus_kat_tests.rs
│   ├── sphincsplus_native_tests.rs
│   ├── sphincsplus_wasm_tests.rs
│   ├── test_classicmceliece.sh
│   ├── test_nist_wasm.rs
│   ├── test_rustpqc_integration.rs
│   ├── test.sh
│   ├── test_wasm_classicmceliece.sh
│   ├── test_wasm.sh
│   └── trait_tests.rs
└── tools
    └── wasi-sdk-20.0
        ├── bin
        │   ├── ar -> llvm-ar
        │   ├── c++filt -> llvm-cxxfilt
        │   ├── clang -> clang-16
        │   ├── clang++ -> clang
        │   ├── clang-16
        │   ├── clang-apply-replacements
        │   ├── clang-cl -> clang
        │   ├── clang-cpp -> clang
        │   ├── clang-format
        │   ├── clang-tidy
        │   ├── git-clang-format
        │   ├── ld64.lld -> lld
        │   ├── ld.lld -> lld
        │   ├── lld
        │   ├── lld-link -> lld
        │   ├── llvm-ar
        │   ├── llvm-cxxfilt
        │   ├── llvm-dwarfdump
        │   ├── llvm-mc
        │   ├── llvm-nm
        │   ├── llvm-objcopy
        │   ├── llvm-objdump
        │   ├── llvm-ranlib -> llvm-ar
        │   ├── llvm-size
        │   ├── llvm-strings
        │   ├── llvm-strip -> llvm-objcopy
        │   ├── nm -> llvm-nm
        │   ├── objcopy -> llvm-objcopy
        │   ├── objdump -> llvm-objdump
        │   ├── ranlib -> llvm-ar
        │   ├── run-clang-tidy
        │   ├── size -> llvm-size
        │   ├── strings -> llvm-strings
        │   ├── strip -> llvm-objcopy
        │   └── wasm-ld -> lld
        ├── lib
        │   └── clang
        └── share
            ├── clang
            ├── cmake
            ├── misc
            └── wasi-sysroot
