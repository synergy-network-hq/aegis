
# Aegis Crypto Core

> **Unified, post-quantum cryptography for Rust, WASM, JavaScript/TypeScript, and Node.js.**
>
> PQC, NIST-compliant. Ready for wallets, blockchains, secure messaging, web apps, and more.

[![CI](https://github.com/synergy-network-hq/aegis/actions/workflows/CI.yml/badge.svg)](https://github.com/synergy-network-hq/aegis/actions)
[![Crates.io](https://img.shields.io/crates/v/aegis_crypto_core.svg)](https://crates.io/crates/aegis_crypto_core)
[![npm](https://img.shields.io/npm/v/aegis-crypto-core.svg)](https://www.npmjs.com/package/aegis-crypto-core)
[![Docs](https://docs.rs/aegis_crypto_core/badge.svg)](https://docs.rs/aegis_crypto_core)

---

## Overview

Aegis delivers a **modular, plug-n-play quantum-safe cryptography core** for any application—web, Node.js, Rust, or WASM. It unifies every major NIST PQC standard and provides developer-friendly APIs for web, blockchain, messaging, and embedded security.

- **NIST PQC standards:** Kyber (ML-KEM), Dilithium (ML-DSA), Falcon, SPHINCS+, HQC, Classic McEliece
- **Installable via Rust (`cargo`), npm (`npm install`), or as a WASM package**
- **All-in-one:** Keygen, sign/verify, encaps/decaps (KEM), zeroize, TypeScript types

---

## Installation

### Rust

```sh
cargo add aegis_crypto_core
```

### npm (Node.js, Browser, WASM)

```sh
npm install aegis-crypto-core
# or yarn add aegis-crypto-core
```

> **Note:** Requires Node.js 18+ or any modern browser with WASM support.

---

## Supported Algorithms

| Algorithm          | PQC Standard | Rust | WASM | npm | Notes                                    |
|--------------------|--------------|------|------|-----|------------------------------------------|
| Kyber (ML-KEM)     | FIPS 203     | ✅    | ✅    | ✅   | All parameter sets                       |
| Dilithium (ML-DSA) | FIPS 204     | ✅    | ✅    | ✅   | All parameter sets                       |
| SPHINCS+           | FIPS 205     | ✅    | ✅    | ✅   | All parameter sets                       |
| Falcon             | FIPS 206     | ✅    | ✅    | ✅   | All parameter sets                       |
| HQC                | FIPS 207     | ✅    | ✅    | ✅   | *Optional; feature-gated*                |
| Classic McEliece   | FIPS 208     | ✅    | ✅*   | ✅*  | *Feature-gated, may be excluded in WASM* |

---

## Usage

### Rust Example

```rust
use aegis_crypto_core::{Algorithm, keypair, sign, verify};

let (pk, sk) = keypair(Algorithm::Dilithium3);
let message = b"hello quantum world!";
let signature = sign(&sk, message).unwrap();
assert!(verify(&pk, message, &signature));
```

### JavaScript / TypeScript Examples (Browser or Node)

#### ML-KEM (Key Encapsulation)
```js
import init, { mlkem768_keygen, mlkem768_encaps, mlkem768_decaps } from "aegis-crypto-core";
await init();

// Generate key pair
const { publicKey, secretKey } = mlkem768_keygen();

// Encapsulate shared secret
const { ciphertext, sharedSecret } = mlkem768_encaps(publicKey);

// Decapsulate shared secret
const decryptedSecret = mlkem768_decaps(ciphertext, secretKey);
console.log('Shared secrets match:', Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret)));
```

#### ML-DSA (Digital Signatures)
```js
import init, { mldsa65_keygen, mldsa65_sign, mldsa65_verify } from "aegis-crypto-core";
await init();

const { publicKey, secretKey } = mldsa65_keygen();
const message = new TextEncoder().encode("hello quantum world!");
const signature = mldsa65_sign(secretKey, message);
console.log(mldsa65_verify(publicKey, message, signature)); // true
```

#### Falcon (Compact Signatures)
```js
import init, { falcon512_keygen, falcon512_sign, falcon512_verify } from "aegis-crypto-core";
await init();

const { publicKey, secretKey } = falcon512_keygen();
const message = new TextEncoder().encode("compact quantum signature");
const signature = falcon512_sign(secretKey, message);
console.log(falcon512_verify(publicKey, message, signature)); // true
console.log('Signature size:', signature.length, 'bytes'); // Very compact!
```

#### SPHINCS+ (Hash-based Signatures)
```js
import init, { slhdsa_sha2_128f_keygen, slhdsa_sha2_128f_sign, slhdsa_sha2_128f_verify } from "aegis-crypto-core";
await init();

const { publicKey, secretKey } = slhdsa_sha2_128f_keygen();
const message = new TextEncoder().encode("long-term quantum security");
const signature = slhdsa_sha2_128f_sign(secretKey, message);
console.log(slhdsa_sha2_128f_verify(publicKey, message, signature)); // true
```

#### HQC-KEM (Hamming Quasi-Cyclic)
```js
import init, { hqc128_keygen, hqc128_encaps, hqc128_decaps } from "aegis-crypto-core";
await init();

const { publicKey, secretKey } = hqc128_keygen();
const { ciphertext, sharedSecret } = hqc128_encaps(publicKey);
const decryptedSecret = hqc128_decaps(ciphertext, secretKey);
console.log('HQC shared secrets match:', Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret)));
```

> **TypeScript users:** All functions have full type declarations out-of-the-box.

### WASM Usage (Direct)

If you need raw WASM output for embedding, use:
```sh
wasm-pack build --target web
```
and import the generated JS/WASM as usual.

---

## Features

- **Unified PQC API:** Switch algorithms on-the-fly (Kyber, Dilithium, Falcon, SPHINCS+, HQC, Classic McEliece)
- **Safe-by-default:** Constant-time, zeroized, no_std and WASM compatible
- **npm & browser ready:** TypeScript types, Node.js and browser demo apps included
- **Secure defaults:** Only trusted, audited pqcrypto backend code

---

## Documentation

- [Quickstart](docs/quickstart.md)
- [Cookbook](docs/cookbook.md)
- [Wallet Integration](docs/wallet_integration.md)
- [Secure Messaging](docs/secure_messaging.md)
- [API Reference (Rust)](https://docs.rs/aegis_crypto_core)

---

## Examples

See the `/examples` directory for:
- **Basic Usage** (`basic-usage.js`) - Complete examples for all 5 algorithms
- **Performance Benchmark** (`performance-benchmark.js`) - Comprehensive performance testing
- **WASM Usage Guide** (`WASM_USAGE_GUIDE.md`) - Detailed WASM documentation
- Browser SPA demo
- Node.js CLI demo
- Secure messaging API example
- Wallet integration quickstart

---

## Contributing

Pull requests welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

Dual-licensed under MIT or Apache-2.0.
Copyright (c) Justin Hutzler.
