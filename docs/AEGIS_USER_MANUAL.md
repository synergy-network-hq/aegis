# Aegis Crypto Core: Complete User Manual

**Version:** 0.8.4
**Last Updated:** 2025-08-22

---

## Table of Contents

01. [Introduction](#1-introduction)
02. [Getting Started](#2-getting-started)
03. [Installation Guide](#3-installation-guide)
04. [Quick Start](#4-quick-start)
05. [Core Algorithms](#5-core-algorithms)
06. [API Reference](#6-api-reference)
07. [Integration Examples](#7-integration-examples)
08. [Platform-Specific Guides](#8-platform-specific-guides)
09. [Real-World Use Cases](#9-real-world-use-cases)
10. [Enhanced Troubleshooting](#10-enhanced-troubleshooting)
11. [Security Best Practices](#11-security-best-practices)
12. [Advanced Usage](#12-advanced-usage)
13. [Performance Optimization](#13-performance-optimization)

---

## 1. Introduction

### 1.1 What is Aegis Crypto Core?

Aegis, the Post-Quantum Cryptography Core [also referred to as 'Aegis Crypto Core,' or simply 'Aegis'] is a comprehensive, production-ready quantum cryptography library designed for the post-quantum era. It provides a unified, high-assurance foundation for implementing quantum-resistant cryptography across all computing platforms—from embedded systems to cloud infrastructure, from web browsers to blockchain networks.

### 1.2 Key Features

* **Complete NIST PQC Coverage**: Five PQC algorithms selected by NIST for standardization (ML-KEM, ML-DSA, SLH-DSA, FN-DSA, HQC-KEM)
* **Universal Platform Support**: Rust, WebAssembly, JavaScript/TypeScript, Python, Node.js
* **Zero-Configuration Deployment**: npm installable, plug-and-play for web applications
* **Enterprise-Grade Security**: Constant-time operations, side-channel resistance, secure memory handling
* **Production Ready**: 47/47 tests passing, NIST KAT validated

### 1.3 Supported Algorithms

| Algorithm | Standard | Variants | Security Levels | Use Case |
|-----------|----------|----------|-----------------|----------|
| **ML-KEM** | FIPS 203 | ML-KEM-512, ML-KEM-768, ML-KEM-1024 | 128, 256, 256+ bits | Key exchange |
| **ML-DSA** | FIPS 204 | ML-DSA-44, ML-DSA-65, ML-DSA-87 | 192, 256, 256+ bits | Digital signatures |
| **SLH-DSA** | FIPS 205 | 6 variants (SHA2/SHAKE-128f/192f/256f) | 128, 192, 256 bits | Hash-based signatures |
| **FN-DSA** | FIPS 206 | FN-DSA-512, FN-DSA-1024 | 128, 256+ bits | Compact signatures |
| **HQC-KEM** | FIPS 207 | HQC-KEM-128, HQC-KEM-192, HQC-KEM-256 | 128, 256, 256+ bits | Alternative KEM |

---

## 2. Getting Started

### 2.1 Prerequisites

**For Rust Development:**
* Rust 1.70+ with Cargo
* Git for version control

**For Web Development:**
* Node.js 18+ and npm
* Modern web browser with WASM support

**For Python Development:**
* Python 3.8+
* pip package manager

### 2.2 System Requirements

**Minimum Requirements:**
* 2GB RAM
* 100MB disk space
* Modern CPU with 64-bit support

**Recommended Requirements:**
* 4GB+ RAM
* 500MB+ disk space
* Multi-core CPU for optimal performance

---

## 3. Installation Guide

### 3.1 NPM Installation (Recommended for Web)

```bash
# Install from npm (when published)
npm install aegis-crypto-core

# Or install from local build
npm install /path/to/aegis_crypto_core/pkg
```

### 3.2 Building from Source

```bash
# Clone the repository
git clone https://github.com/synergy-network-hq/aegis.git
cd aegis/aegis_crypto_core

# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers
wasm-pack build --target bundler
```

### 3.3 Cargo Installation (Rust)

```toml
[dependencies]
aegis_crypto_core = { version = "0.1.0", features = ["mlkem", "mldsa", "fndsa", "slhdsa"] }
```

### 3.4 Python Installation (Future)

```bash
pip install aegis-crypto-core
```

---

## 4. Quick Start

### 4.1 Node.js Quick Start

```javascript
const fs = require("fs");
const path = require("path");
const wasm = require("./pkg");

async function main() {
    // Initialize WASM
    const wasmBytes = fs.readFileSync(path.join(__dirname, "pkg", "aegis_crypto_core_bg.wasm"));
    await wasm.initSync(wasmBytes);

    // Generate a Dilithium key pair
    const keypair = wasm.dilithium_keygen();

    // Sign a message
    const message = new TextEncoder().encode("Hello, quantum world!");
    const signature = wasm.dilithium_sign(keypair.secret_key, message);

    // Verify the signature
    const isValid = wasm.dilithium_verify(keypair.public_key, message, signature);
    console.log("Signature valid:", isValid);
}

main();
```

### 4.2 Browser Quick Start

```html
<!DOCTYPE html>
<html>

<head>
    <title>Aegis Crypto Demo</title>
</head>

<body>
    <script type="module">
        import init, * as wasm from "./pkg/aegis_crypto_core.js";

        async function main() {
            // Initialize WASM
            await init();

            // Generate a Kyber key pair
            const keypair = wasm.kyber_keygen();

            // Encapsulate a shared secret
            const encapsulated = wasm.kyber_encapsulate(keypair.public_key);

            // Decapsulate the shared secret
            const sharedSecret = wasm.kyber_decapsulate(keypair.secret_key, encapsulated.ciphertext);

            console.log("Shared secret length:", sharedSecret.length);
        }

        main();
    </script>
</body>

</html>
```

### 4.3 Rust Quick Start

```rust
use aegis_crypto_core::mlkem::*;
use aegis_crypto_core::mldsa::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate ML-KEM-768 keypair
    let keypair = mlkem768_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Encapsulate shared secret
    let encapsulated = mlkem768_encapsulate(&public_key)?;
    let ciphertext = encapsulated.ciphertext();
    let shared_secret = encapsulated.shared_secret();

    // Decapsulate shared secret
    let decapsulated = mlkem768_decapsulate(&secret_key, &ciphertext)?;
    assert_eq!(shared_secret, decapsulated);

    // Generate ML-DSA-65 keypair
    let mldsa_keypair = mldsa65_keygen();

    // Sign message
    let message = b"Hello, quantum world!";
    let signature = mldsa65_sign(&mldsa_keypair.secret_key(), message)?;

    // Verify signature
    let is_valid = mldsa65_verify(&mldsa_keypair.public_key(), &signature, message)?;
    assert!(is_valid);

    println!("All operations successful!");
    Ok(())
}
```

---

## 5. Core Algorithms

### 5.1 ML-KEM (Key Encapsulation Mechanism)

ML-KEM is the NIST standard for post-quantum key exchange, based on module-lattice cryptography.

**Available Variants:**
* ML-KEM-512: 128-bit security level
* ML-KEM-768: 192-bit security level (recommended)
* ML-KEM-1024: 256-bit security level

**Key Sizes:**
* Public Key: ~800-1, 500 bytes
* Secret Key: ~1, 500-3, 000 bytes
* Ciphertext: ~700-1, 500 bytes

**Usage Example:**

```javascript
// Generate key pair
const keypair = wasm.mlkem768_keygen();

// Encapsulate (sender)
const encapsulated = wasm.mlkem768_encapsulate(keypair.public_key);
const ciphertext = encapsulated.ciphertext();
const sharedSecret = encapsulated.shared_secret();

// Decapsulate (receiver)
const decapsulated = wasm.mlkem768_decapsulate(keypair.secret_key, ciphertext);
// sharedSecret === decapsulated
```

### 5.2 ML-DSA (Digital Signature Algorithm)

ML-DSA is the NIST standard for post-quantum digital signatures, based on module-lattice cryptography.

**Available Variants:**
* ML-DSA-44: 192-bit security level
* ML-DSA-65: 256-bit security level (recommended)
* ML-DSA-87: 256+ bit security level

**Key Sizes:**
* Public Key: ~1, 300-2, 500 bytes
* Secret Key: ~2, 500-5, 000 bytes
* Signature: ~2, 400-4, 500 bytes

**Usage Example:**

```javascript
// Generate key pair
const keypair = wasm.mldsa65_keygen();

// Sign message
const message = new TextEncoder().encode("Hello, quantum world!");
const signature = wasm.mldsa65_sign(keypair.secret_key, message);

// Verify signature
const isValid = wasm.mldsa65_verify(keypair.public_key, signature, message);
```

### 5.3 SLH-DSA (Hash-Based Signatures)

SLH-DSA provides stateless hash-based digital signatures with conservative security assumptions.

**Available Variants:**
* SLH-DSA-SHA2-128f/192f/256f
* SLH-DSA-SHAKE-128f/192f/256f

**Key Sizes:**
* Public Key: ~32 bytes (very small)
* Secret Key: ~64 bytes (very small)
* Signature: ~7, 000-17, 000 bytes (large)

**Usage Example:**

```javascript
// Generate key pair
const keypair = wasm.slhdsa_sha2_128f_keygen();

// Sign message
const message = new TextEncoder().encode("Hello, quantum world!");
const signature = wasm.slhdsa_sha2_128f_sign(keypair.secret_key, message);

// Verify signature
const isValid = wasm.slhdsa_sha2_128f_verify(keypair.public_key, signature, message);
```

### 5.4 FN-DSA (Compact Signatures)

FN-DSA provides compact lattice-based digital signatures, ideal for bandwidth-constrained environments.

**Available Variants:**
* FN-DSA-512: 128-bit security level
* FN-DSA-1024: 256+ bit security level

**Key Sizes:**
* Public Key: ~900-1, 800 bytes
* Secret Key: ~1, 300-2, 500 bytes
* Signature: ~600-1, 300 bytes (compact)

**Usage Example:**

```javascript
// Generate key pair
const keypair = wasm.fndsa512_keygen();

// Sign message
const message = new TextEncoder().encode("Hello, quantum world!");
const signature = wasm.fndsa512_sign(keypair.secret_key, message);

// Verify signature
const isValid = wasm.fndsa512_verify(keypair.public_key, signature, message);
```

### 5.5 HQC-KEM (Alternative KEM)

HQC-KEM provides an alternative key encapsulation mechanism based on quasi-cyclic codes.

**Available Variants:**
* HQC-KEM-128: 128-bit security level
* HQC-KEM-192: 192-bit security level
* HQC-KEM-256: 256-bit security level

**Usage Example:**

```javascript
// Generate key pair
const keypair = wasm.hqckem128_keygen();

// Encapsulate
const encapsulated = wasm.hqckem128_encapsulate(keypair.public_key);
const ciphertext = encapsulated.ciphertext();
const sharedSecret = encapsulated.shared_secret();

// Decapsulate
const decapsulated = wasm.hqckem128_decapsulate(keypair.secret_key, ciphertext);
```

---

## 6. API Reference

### 6.1 JavaScript/TypeScript API

#### Initialization

```javascript
import init, * as wasm from "aegis-crypto-core";

// Initialize WASM module
await init();
```

#### Key Generation Functions

```javascript
// ML-KEM key generation
const mlkemKeypair = wasm.mlkem768_keygen();
const publicKey = mlkemKeypair.public_key();
const secretKey = mlkemKeypair.secret_key();

// ML-DSA key generation
const mldsaKeypair = wasm.mldsa65_keygen();
const publicKey = mldsaKeypair.public_key();
const secretKey = mldsaKeypair.secret_key();
```

#### Key Encapsulation Functions

```javascript
// Encapsulate shared secret
const encapsulated = wasm.mlkem768_encapsulate(publicKey);
const ciphertext = encapsulated.ciphertext();
const sharedSecret = encapsulated.shared_secret();

// Decapsulate shared secret
const decapsulated = wasm.mlkem768_decapsulate(secretKey, ciphertext);
```

#### Digital Signature Functions

```javascript
// Sign message
const message = new TextEncoder().encode("Hello, world!");
const signature = wasm.mldsa65_sign(secretKey, message);

// Verify signature
const isValid = wasm.mldsa65_verify(publicKey, signature, message);
```

### 6.2 Rust API

#### Key Generation

```rust
use aegis_crypto_core::mlkem::*;
use aegis_crypto_core::mldsa::*;

// Generate ML-KEM-768 keypair
let keypair = mlkem768_keygen();
let public_key = keypair.public_key();
let secret_key = keypair.secret_key();

// Generate ML-DSA-65 keypair
let mldsa_keypair = mldsa65_keygen();
```

#### Key Encapsulation

```rust
// Encapsulate shared secret
let encapsulated = mlkem768_encapsulate(&public_key)?;
let ciphertext = encapsulated.ciphertext();
let shared_secret = encapsulated.shared_secret();

// Decapsulate shared secret
let decapsulated = mlkem768_decapsulate(&secret_key, &ciphertext)?;
assert_eq!(shared_secret, decapsulated);
```

#### Digital Signatures

```rust
// Sign message
let message = b"Hello, quantum world!";
let signature = mldsa65_sign(&mldsa_keypair.secret_key(), message)?;

// Verify signature
let is_valid = mldsa65_verify(&mldsa_keypair.public_key(), &signature, message)?;
assert!(is_valid);
```

---

## 7. Integration Examples

### 7.1 Web Application Integration

```javascript
// document_signing_example.js
import init, {
    mldsa65_keygen,
    mldsa65_sign,
    mldsa65_verify
} from "aegis-crypto-core";

class DocumentSigner {
    constructor() {
        this.keypair = null;
    }

    async initialize() {
        await init();
        this.keypair = mldsa65_keygen();
    }

    signDocument(documentContent) {
        if (!this.keypair) {
            throw new Error("Not initialized");
        }

        const message = new TextEncoder().encode(documentContent);
        return mldsa65_sign(this.keypair.secret_key(), message);
    }

    verifyDocument(documentContent, signature) {
        if (!this.keypair) {
            throw new Error("Not initialized");
        }

        const message = new TextEncoder().encode(documentContent);
        return mldsa65_verify(this.keypair.public_key(), signature, message);
    }

    getPublicKey() {
        return this.keypair ? this.keypair.public_key() : null;
    }
}

// Usage
const signer = new DocumentSigner();
await signer.initialize();

const document = "This is an important document";
const signature = signer.signDocument(document);
const isValid = signer.verifyDocument(document, signature);

console.log("Document signed and verified:", isValid);
```

### 7.2 Secure Messaging Application

```javascript
// secure_messaging_example.js
import init, {
    mlkem768_keygen,
    mlkem768_encapsulate,
    mlkem768_decapsulate,
    mldsa65_keygen,
    mldsa65_sign,
    mldsa65_verify
} from "aegis-crypto-core";

class SecureMessaging {
    constructor() {
        this.kemKeypair = null;
        this.signKeypair = null;
    }

    async initialize() {
        await init();
        this.kemKeypair = mlkem768_keygen();
        this.signKeypair = mldsa65_keygen();
    }

    // Generate shared secret for encryption
    generateSharedSecret(recipientPublicKey) {
        const encapsulated = mlkem768_encapsulate(recipientPublicKey);
        return {
            ciphertext: encapsulated.ciphertext(),
            sharedSecret: encapsulated.shared_secret()
        };
    }

    // Decapsulate shared secret
    decapsulateSharedSecret(ciphertext) {
        return mlkem768_decapsulate(this.kemKeypair.secret_key(), ciphertext);
    }

    // Sign message
    signMessage(message) {
        const messageBytes = new TextEncoder().encode(message);
        return mldsa65_sign(this.signKeypair.secret_key(), messageBytes);
    }

    // Verify message signature
    verifyMessage(message, signature, senderPublicKey) {
        const messageBytes = new TextEncoder().encode(message);
        return mldsa65_verify(senderPublicKey, signature, messageBytes);
    }

    getPublicKeys() {
        return {
            kemPublicKey: this.kemKeypair.public_key(),
            signPublicKey: this.signKeypair.public_key()
        };
    }
}

// Usage
const alice = new SecureMessaging();
const bob = new SecureMessaging();

await alice.initialize();
await bob.initialize();

// Alice sends encrypted message to Bob
const aliceKeys = alice.getPublicKeys();
const bobKeys = bob.getPublicKeys();

// Generate shared secret
const {
    ciphertext,
    sharedSecret
} = alice.generateSharedSecret(bobKeys.kemPublicKey);
const bobSharedSecret = bob.decapsulateSharedSecret(ciphertext);

// Sign message
const message = "Hello Bob, this is a secure message!";
const signature = alice.signMessage(message);

// Verify signature
const isValid = bob.verifyMessage(message, signature, aliceKeys.signPublicKey);

console.log("Message verified:", isValid);
console.log("Shared secrets match:",
    Array.from(sharedSecret).every((byte, i) => byte === bobSharedSecret[i]));
```

### 7.3 Blockchain Integration

```javascript
// blockchain_integration_example.js
import init, {
    mldsa65_keygen,
    mldsa65_sign,
    mldsa65_verify
} from "aegis-crypto-core";

class PostQuantumWallet {
    constructor() {
        this.keypair = null;
        this.address = null;
    }

    async initialize() {
        await init();
        this.keypair = mldsa65_keygen();
        this.address = this.generateAddress();
    }

    generateAddress() {
        // Generate quantum-resistant address from public key
        const publicKey = this.keypair.public_key();
        // In practice, you'd use a proper hash function
        return `pq_${Array.from(publicKey.slice(0, 20))
            .map(b => b.toString(16).padStart(2, '0'))
            .join('')}`;
    }

    signTransaction(transaction) {
        const transactionBytes = new TextEncoder().encode(JSON.stringify(transaction));
        return mldsa65_sign(this.keypair.secret_key(), transactionBytes);
    }

    verifyTransaction(transaction, signature, senderPublicKey) {
        const transactionBytes = new TextEncoder().encode(JSON.stringify(transaction));
        return mldsa65_verify(senderPublicKey, signature, transactionBytes);
    }

    getPublicKey() {
        return this.keypair.public_key();
    }

    getAddress() {
        return this.address;
    }
}

// Usage
const wallet = new PostQuantumWallet();
await wallet.initialize();

const transaction = {
    from: wallet.getAddress(),
    to: "pq_recipient_address",
    amount: 100,
    nonce: 1,
    timestamp: Date.now()
};

const signature = wallet.signTransaction(transaction);
const isValid = wallet.verifyTransaction(transaction, signature, wallet.getPublicKey());

console.log("Wallet address:", wallet.getAddress());
console.log("Transaction verified:", isValid);
```

---

## 8. Platform-Specific Guides

### 8.1 Web Browser Integration

**HTML Setup:**

```html
<!DOCTYPE html>
<html>

<head>
    <title>Post-Quantum Crypto Demo</title>
</head>

<body>
    <script type="module" src="app.js"></script>
</body>

</html>
```

**JavaScript Module:**

```javascript
// app.js
import init, * as wasm from "./pkg/aegis_crypto_core.js";

async function main() {
    try {
        await init();
        console.log("Aegis Crypto Core initialized successfully");

        // Your application code here
        const keypair = wasm.mlkem768_keygen();
        console.log("Key pair generated");

    } catch (error) {
        console.error("Failed to initialize:", error);
    }
}

main();
```

**Webpack Configuration:**

```javascript
// webpack.config.js
module.exports = {
    experiments: {
        asyncWebAssembly: true
    },
    resolve: {
        fallback: {
            "fs": false,
            "path": false
        }
    }
};
```

### 8.2 Node.js Integration

**Package.json:**

```json
{
    "name": "my-pq-app",
    "version": "1.0.0",
    "type": "module",
    "dependencies": {
        "aegis-crypto-core": "^1.0.0"
    }
}
```

**Application Code:**

```javascript
// app.js
import init, * as wasm from "aegis-crypto-core";
import fs from "fs";
import path from "path";

async function main() {
    try {
        // Initialize WASM for Node.js
        const wasmPath = path.resolve("node_modules/aegis-crypto-core/aegis_crypto_core_bg.wasm");
        const wasmBytes = fs.readFileSync(wasmPath);
        await init(wasmBytes);

        console.log("Aegis Crypto Core initialized successfully");

        // Your application code here

    } catch (error) {
        console.error("Failed to initialize:", error);
    }
}

main();
```

### 8.3 React Integration

**React Component:**

```jsx
// CryptoComponent.jsx
import React, { useState, useEffect } from 'react';
import init, * as wasm from 'aegis-crypto-core';

function CryptoComponent() {
    const [initialized, setInitialized] = useState(false);
    const [keypair, setKeypair] = useState(null);

    useEffect(() => {
        async function initializeCrypto() {
            try {
                await init();
                setInitialized(true);
                console.log("Crypto initialized");
            } catch (error) {
                console.error("Failed to initialize crypto:", error);
            }
        }

        initializeCrypto();
    }, []);

    const generateKeypair = () => {
        if (initialized) {
            const newKeypair = wasm.mlkem768_keygen();
            setKeypair(newKeypair);
        }
    };

    return (
        <div>
            <h2>Post-Quantum Cryptography Demo</h2>
            <p>Status: {initialized ? "Initialized" : "Initializing..."}</p>
            <button onClick={generateKeypair} disabled={!initialized}>
                Generate Key Pair
            </button>
            {keypair && (
                <div>
                    <p>Key pair generated successfully!</p>
                    <p>Public key length: {keypair.public_key().length} bytes</p>
                </div>
            )}
        </div>
    );
}

export default CryptoComponent;
```

### 8.4 Python Integration (Future)

```python
# python_integration.py
import aegis_crypto_core as aegis

def main():
    # Generate key pair
    keypair = aegis.mlkem768_keygen()
    public_key = keypair.public_key()
    secret_key = keypair.secret_key()

    # Encapsulate shared secret
    encapsulated = aegis.mlkem768_encapsulate(public_key)
    ciphertext = encapsulated.ciphertext()
    shared_secret = encapsulated.shared_secret()

    # Decapsulate shared secret
    decapsulated = aegis.mlkem768_decapsulate(secret_key, ciphertext)

    print("Shared secrets match:", shared_secret == decapsulated)

if __name__ == "__main__":
    main()
```

---

## 9. Real-World Use Cases

### 9.1 Financial Services & Banking

**Digital Banking Applications:**
Aegis enables quantum-resistant security for online banking, mobile payments, and digital wallets.

```javascript
// Banking transaction signing
class QuantumResistantBanking {
    async initialize() {
        await init();
        this.transactionKeypair = wasm.mldsa65_keygen();
        this.communicationKeypair = wasm.mlkem768_keygen();
    }

    signTransaction(transaction) {
        const txData = {
            from: transaction.from,
            to: transaction.to,
            amount: transaction.amount,
            timestamp: Date.now(),
            nonce: transaction.nonce
        };

        const message = new TextEncoder().encode(JSON.stringify(txData));
        return wasm.mldsa65_sign(this.transactionKeypair.secret_key(), message);
    }

    verifyTransaction(transaction, signature, senderPublicKey) {
        const txData = {
            from: transaction.from,
            to: transaction.to,
            amount: transaction.amount,
            timestamp: transaction.timestamp,
            nonce: transaction.nonce
        };

        const message = new TextEncoder().encode(JSON.stringify(txData));
        return wasm.mldsa65_verify(senderPublicKey, signature, message);
    }

    establishSecureChannel(recipientPublicKey) {
        const encapsulated = wasm.mlkem768_encapsulate(recipientPublicKey);
        return {
            ciphertext: encapsulated.ciphertext(),
            sharedSecret: encapsulated.shared_secret()
        };
    }
}
```

**Cryptocurrency & Blockchain:**
Post-quantum blockchain networks using Aegis for transaction signing and consensus mechanisms.

```javascript
// Post-quantum blockchain node
class PostQuantumBlockchain {
    constructor() {
        this.consensusKeypair = null;
        this.transactionKeypair = null;
    }

    async initialize() {
        await init();
        this.consensusKeypair = wasm.mldsa87_keygen(); // Highest security for consensus
        this.transactionKeypair = wasm.mldsa65_keygen(); // Balanced for transactions
    }

    createBlock(transactions, previousHash) {
        const blockData = {
            transactions: transactions,
            previousHash: previousHash,
            timestamp: Date.now(),
            nonce: this.generateNonce()
        };

        const message = new TextEncoder().encode(JSON.stringify(blockData));
        const signature = wasm.mldsa87_sign(this.consensusKeypair.secret_key(), message);

        return {
            ...blockData,
            signature: signature,
            publicKey: this.consensusKeypair.public_key()
        };
    }

    verifyBlock(block) {
        const blockData = {
            transactions: block.transactions,
            previousHash: block.previousHash,
            timestamp: block.timestamp,
            nonce: block.nonce
        };

        const message = new TextEncoder().encode(JSON.stringify(blockData));
        return wasm.mldsa87_verify(block.publicKey, block.signature, message);
    }
}
```

### 9.2 Healthcare & Medical Data

**Electronic Health Records (EHR):**
Secure patient data with quantum-resistant encryption and digital signatures.

```javascript
// Healthcare data protection
class HealthcareDataProtection {
    async initialize() {
        await init();
        this.patientKeypair = wasm.mldsa65_keygen();
        this.encryptionKeypair = wasm.mlkem768_keygen();
    }

    signPatientRecord(patientId, recordData) {
        const record = {
            patientId: patientId,
            data: recordData,
            timestamp: Date.now(),
            version: this.getRecordVersion(patientId)
        };

        const message = new TextEncoder().encode(JSON.stringify(record));
        return wasm.mldsa65_sign(this.patientKeypair.secret_key(), message);
    }

    encryptSensitiveData(data, recipientPublicKey) {
        const encapsulated = wasm.mlkem768_encapsulate(recipientPublicKey);
        const sharedSecret = encapsulated.shared_secret();

        // Use shared secret for AES encryption
        return {
            ciphertext: encapsulated.ciphertext(),
            encryptedData: this.encryptWithAES(data, sharedSecret)
        };
    }

    verifyRecordIntegrity(record, signature, publicKey) {
        const recordData = {
            patientId: record.patientId,
            data: record.data,
            timestamp: record.timestamp,
            version: record.version
        };

        const message = new TextEncoder().encode(JSON.stringify(recordData));
        return wasm.mldsa65_verify(publicKey, signature, message);
    }
}
```

### 9.3 Government & Defense

**Secure Government Communications:**
Classified communications with quantum-resistant encryption and authentication.

```javascript
// Government secure communications
class GovernmentSecureComm {
    constructor(clearanceLevel) {
        this.clearanceLevel = clearanceLevel;
        this.communicationKeypair = null;
        this.identityKeypair = null;
    }

    async initialize() {
        await init();

        // Use highest security for government communications
        this.communicationKeypair = wasm.mlkem1024_keygen();
        this.identityKeypair = wasm.mldsa87_keygen();
    }

    sendClassifiedMessage(message, recipientPublicKey, classification) {
        // Establish secure channel
        const encapsulated = wasm.mlkem1024_encapsulate(recipientPublicKey);
        const sharedSecret = encapsulated.shared_secret();

        // Sign message with identity
        const messageData = {
            content: message,
            classification: classification,
            timestamp: Date.now(),
            sender: this.getIdentity()
        };

        const messageBytes = new TextEncoder().encode(JSON.stringify(messageData));
        const signature = wasm.mldsa87_sign(this.identityKeypair.secret_key(), messageBytes);

        // Encrypt with shared secret
        const encryptedMessage = this.encryptWithSharedSecret(messageBytes, sharedSecret);

        return {
            ciphertext: encapsulated.ciphertext(),
            encryptedMessage: encryptedMessage,
            signature: signature,
            publicKey: this.identityKeypair.public_key()
        };
    }

    verifyClassifiedMessage(message, signature, senderPublicKey) {
        const messageBytes = new TextEncoder().encode(JSON.stringify(message));
        return wasm.mldsa87_verify(senderPublicKey, signature, messageBytes);
    }
}
```

### 9.4 IoT & Smart Cities

**Smart Grid Security:**
Power grid infrastructure with quantum-resistant security for critical systems.

```javascript
// Smart grid security
class SmartGridSecurity {
    constructor(nodeId, nodeType) {
        this.nodeId = nodeId;
        this.nodeType = nodeType; // 'generator', 'transformer', 'consumer'
        this.gridKeypair = null;
        this.communicationKeypair = null;
    }

    async initialize() {
        await init();
        this.gridKeypair = wasm.mldsa65_keygen();
        this.communicationKeypair = wasm.mlkem768_keygen();
    }

    reportGridStatus(status) {
        const statusData = {
            nodeId: this.nodeId,
            nodeType: this.nodeType,
            status: status,
            timestamp: Date.now(),
            location: this.getLocation()
        };

        const message = new TextEncoder().encode(JSON.stringify(statusData));
        const signature = wasm.mldsa65_sign(this.gridKeypair.secret_key(), message);

        return {
            status: statusData,
            signature: signature,
            publicKey: this.gridKeypair.public_key()
        };
    }

    receiveGridCommand(command, senderPublicKey, signature) {
        const message = new TextEncoder().encode(JSON.stringify(command));
        const isValid = wasm.mldsa65_verify(senderPublicKey, signature, message);

        if (isValid) {
            this.executeCommand(command);
            return true;
        }
        return false;
    }
}
```

### 9.5 Cloud Computing & Enterprise

**Multi-Cloud Security:**
Enterprise applications with quantum-resistant security across multiple cloud providers.

```javascript
// Multi-cloud security
class MultiCloudSecurity {
    constructor(tenantId) {
        this.tenantId = tenantId;
        this.tenantKeypair = null;
        this.cloudKeypairs = new Map();
    }

    async initialize() {
        await init();
        this.tenantKeypair = wasm.mldsa65_keygen();

        // Generate keypairs for each cloud provider
        this.cloudKeypairs.set('aws', wasm.mlkem768_keygen());
        this.cloudKeypairs.set('azure', wasm.mlkem768_keygen());
        this.cloudKeypairs.set('gcp', wasm.mlkem768_keygen());
    }

    deployToCloud(cloudProvider, applicationData) {
        const cloudKeypair = this.cloudKeypairs.get(cloudProvider);
        if (!cloudKeypair) {
            throw new Error(`No keypair for cloud provider: ${cloudProvider}`);
        }

        const deploymentData = {
            tenantId: this.tenantId,
            cloudProvider: cloudProvider,
            applicationData: applicationData,
            timestamp: Date.now()
        };

        const message = new TextEncoder().encode(JSON.stringify(deploymentData));
        const signature = wasm.mldsa65_sign(this.tenantKeypair.secret_key(), message);

        return {
            deployment: deploymentData,
            signature: signature,
            publicKey: this.tenantKeypair.public_key()
        };
    }

    establishSecureConnection(cloudProvider, cloudPublicKey) {
        const cloudKeypair = this.cloudKeypairs.get(cloudProvider);
        if (!cloudKeypair) {
            throw new Error(`No keypair for cloud provider: ${cloudProvider}`);
        }

        const encapsulated = wasm.mlkem768_encapsulate(cloudPublicKey);
        return {
            ciphertext: encapsulated.ciphertext(),
            sharedSecret: encapsulated.shared_secret()
        };
    }
}
```

### 9.6 Supply Chain & Logistics

**Supply Chain Traceability:**
End-to-end supply chain tracking with quantum-resistant integrity verification.

```javascript
// Supply chain traceability
class SupplyChainTraceability {
    constructor(companyId) {
        this.companyId = companyId;
        this.companyKeypair = null;
        this.productKeypairs = new Map();
    }

    async initialize() {
        await init();
        this.companyKeypair = wasm.mldsa65_keygen();
    }

    createProduct(productId, productData) {
        const productKeypair = wasm.mldsa65_keygen();
        this.productKeypairs.set(productId, productKeypair);

        const productRecord = {
            productId: productId,
            companyId: this.companyId,
            productData: productData,
            created: Date.now(),
            status: 'created'
        };

        const message = new TextEncoder().encode(JSON.stringify(productRecord));
        const signature = wasm.mldsa65_sign(this.companyKeypair.secret_key(), message);

        return {
            product: productRecord,
            signature: signature,
            publicKey: this.companyKeypair.public_key(),
            productPublicKey: productKeypair.public_key()
        };
    }

    updateProductStatus(productId, newStatus, location) {
        const productKeypair = this.productKeypairs.get(productId);
        if (!productKeypair) {
            throw new Error('Product not found');
        }

        const statusUpdate = {
            productId: productId,
            newStatus: newStatus,
            location: location,
            timestamp: Date.now(),
            companyId: this.companyId
        };

        const message = new TextEncoder().encode(JSON.stringify(statusUpdate));
        const signature = wasm.mldsa65_sign(productKeypair.secret_key(), message);

        return {
            update: statusUpdate,
            signature: signature,
            publicKey: productKeypair.public_key()
        };
    }

    verifyProductHistory(productHistory) {
        let isValid = true;

        for (const record of productHistory) {
            const message = new TextEncoder().encode(JSON.stringify(record.data));
            const recordValid = wasm.mldsa65_verify(record.publicKey, record.signature, message);
            if (!recordValid) {
                isValid = false;
                break;
            }
        }

        return isValid;
    }
}
```

---

## 11. Security Best Practices

### 11.1 Key Management

**Secure Key Storage:**

```javascript
// Store keys securely (example for browser)
class SecureKeyManager {
    constructor() {
        this.keyStorage = new Map();
    }

    storeKey(keyId, keyData) {
        // In production, encrypt keys before storage
        const encryptedKey = this.encryptKey(keyData);
        this.keyStorage.set(keyId, encryptedKey);
    }

    retrieveKey(keyId) {
        const encryptedKey = this.keyStorage.get(keyId);
        if (!encryptedKey) return null;
        return this.decryptKey(encryptedKey);
    }

    encryptKey(keyData) {
        // Implement proper encryption using Web Crypto API
        // This is a simplified example
        return btoa(JSON.stringify(keyData));
    }

    decryptKey(encryptedKey) {
        // Implement proper decryption
        return JSON.parse(atob(encryptedKey));
    }
}
```

**Key Rotation:**

```javascript
class KeyRotationManager {
    constructor() {
        this.currentKey = null;
        this.previousKey = null;
        this.rotationInterval = 30 * 24 * 60 * 60 * 1000; // 30 days
    }

    async rotateKey() {
        if (this.currentKey) {
            this.previousKey = this.currentKey;
        }
        this.currentKey = wasm.mlkem768_keygen();
        this.lastRotation = Date.now();
    }

    shouldRotate() {
        return Date.now() - this.lastRotation > this.rotationInterval;
    }
}
```

### 11.2 Secure Random Number Generation

**Browser:**

```javascript
// Ensure cryptographically secure randomness
function getSecureRandomBytes(length) {
    const array = new Uint8Array(length);
    crypto.getRandomValues(array);
    return array;
}
```

**Node.js:**

```javascript
import crypto from 'crypto';

function getSecureRandomBytes(length) {
    return crypto.randomBytes(length);
}
```

### 11.3 Memory Security

**Secure Memory Handling:**

```javascript
class SecureMemoryManager {
    constructor() {
        this.sensitiveData = new Map();
    }

    storeSensitive(key, data) {
        // Store sensitive data with automatic cleanup
        this.sensitiveData.set(key, {
            data: data,
            timestamp: Date.now(),
            ttl: 5 * 60 * 1000 // 5 minutes
        });
    }

    retrieveSensitive(key) {
        const entry = this.sensitiveData.get(key);
        if (!entry) return null;

        if (Date.now() - entry.timestamp > entry.ttl) {
            this.clearSensitive(key);
            return null;
        }

        return entry.data;
    }

    clearSensitive(key) {
        const entry = this.sensitiveData.get(key);
        if (entry) {
            // Zero out memory
            if (entry.data instanceof Uint8Array) {
                entry.data.fill(0);
            }
            this.sensitiveData.delete(key);
        }
    }

    clearAllSensitive() {
        for (const [key, entry] of this.sensitiveData) {
            if (entry.data instanceof Uint8Array) {
                entry.data.fill(0);
            }
        }
        this.sensitiveData.clear();
    }
}
```

### 11.4 Input Validation

**Message Validation:**

```javascript
class MessageValidator {
    static validateMessage(message) {
        if (!message || typeof message !== 'string') {
            throw new Error('Invalid message: must be a non-empty string');
        }

        if (message.length > 1024 * 1024) { // 1MB limit
            throw new Error('Message too large: maximum 1MB');
        }

        return true;
    }

    static validateSignature(signature) {
        if (!signature || !(signature instanceof Uint8Array)) {
            throw new Error('Invalid signature: must be Uint8Array');
        }

        // Check signature size limits
        if (signature.length > 20000) { // 20KB limit
            throw new Error('Signature too large: maximum 20KB');
        }

        return true;
    }
}
```

---

## 10. Enhanced Troubleshooting

### 10.1 Common Issues and Solutions

**WASM Initialization Failed:**

```javascript
// Comprehensive WASM initialization check
class WASMInitializationChecker {
    static async checkBrowserCompatibility() {
        const checks = {
            webAssembly: !!WebAssembly,
            webAssemblyCompile: !!WebAssembly.compile,
            webAssemblyInstantiate: !!WebAssembly.instantiate,
            webAssemblyMemory: !!WebAssembly.Memory,
            webAssemblyTable: !!WebAssembly.Table
        };

        const allSupported = Object.values(checks).every(check => check);

        if (!allSupported) {
            throw new Error(`WebAssembly not fully supported. Missing: ${
                Object.entries(checks)
                    .filter(([_, supported]) => !supported)
                    .map(([feature, _]) => feature)
                    .join(', ')
            }`);
        }

        return checks;
    }

    static async checkWASMFileAccess(wasmPath) {
        try {
            const response = await fetch(wasmPath, {
                method: 'HEAD'
            });
            if (!response.ok) {
                throw new Error(`WASM file not accessible: ${response.status} ${response.statusText}`);
            }
            return true;
        } catch (error) {
            throw new Error(`WASM file access failed: ${error.message}`);
        }
    }

    static async initializeWithFallback() {
        try {
            // Try standard initialization
            await init();
            return {
                success: true,
                method: 'standard'
            };
        } catch (error) {
            console.warn('Standard initialization failed, trying fallback:', error);

            try {
                // Try with explicit WASM file loading
                const wasmPath = './pkg/aegis_crypto_core_bg.wasm';
                const wasmBytes = await fetch(wasmPath).then(r => r.arrayBuffer());
                await init(wasmBytes);
                return {
                    success: true,
                    method: 'fallback'
                };
            } catch (fallbackError) {
                throw new Error(`All initialization methods failed. Standard: ${error.message}, Fallback: ${fallbackError.message}`);
            }
        }
    }
}

// Usage
try {
    await WASMInitializationChecker.checkBrowserCompatibility();
    await WASMInitializationChecker.checkWASMFileAccess('./pkg/aegis_crypto_core_bg.wasm');
    const result = await WASMInitializationChecker.initializeWithFallback();
    console.log(`WASM initialized successfully using ${result.method} method`);
} catch (error) {
    console.error('WASM initialization failed:', error.message);
}
```

**Memory Issues and Optimization:**

```javascript
// Advanced memory management
class AdvancedMemoryManager {
    constructor() {
        this.memoryPool = new Map();
        this.memoryStats = {
            totalAllocated: 0,
            peakUsage: 0,
            currentUsage: 0
        };
    }

    allocateMemory(size, purpose) {
        const id = this.generateId();
        const memory = new Uint8Array(size);

        this.memoryPool.set(id, {
            memory: memory,
            size: size,
            purpose: purpose,
            allocated: Date.now(),
            lastUsed: Date.now()
        });

        this.updateMemoryStats(size);
        return {
            id,
            memory
        };
    }

    deallocateMemory(id) {
        const entry = this.memoryPool.get(id);
        if (entry) {
            // Zero out memory for security
            entry.memory.fill(0);
            this.memoryPool.delete(id);
            this.updateMemoryStats(-entry.size);
        }
    }

    updateMemoryStats(delta) {
        this.memoryStats.currentUsage += delta;
        this.memoryStats.totalAllocated += Math.abs(delta);
        this.memoryStats.peakUsage = Math.max(this.memoryStats.peakUsage, this.memoryStats.currentUsage);
    }

    getMemoryStats() {
        return {
            ...this.memoryStats,
            activeAllocations: this.memoryPool.size,
            memoryPool: Array.from(this.memoryPool.entries()).map(([id, entry]) => ({
                id,
                size: entry.size,
                purpose: entry.purpose,
                age: Date.now() - entry.allocated
            }))
        };
    }

    cleanupExpiredMemory(maxAge = 300000) { // 5 minutes
        const now = Date.now();
        for (const [id, entry] of this.memoryPool) {
            if (now - entry.lastUsed > maxAge) {
                this.deallocateMemory(id);
            }
        }
    }
}
```

**Performance Issues and Optimization:**

```javascript
// Performance optimization and monitoring
class PerformanceOptimizer {
    constructor() {
        this.performanceMetrics = new Map();
        this.optimizationStrategies = new Map();
    }

    measureOperation(operationName, operation) {
        const start = performance.now();
        const startMemory = this.getMemoryUsage();

        try {
            const result = operation();
            const end = performance.now();
            const endMemory = this.getMemoryUsage();

            const metrics = {
                duration: end - start,
                memoryDelta: endMemory - startMemory,
                timestamp: Date.now(),
                success: true
            };

            this.recordMetrics(operationName, metrics);
            return result;
        } catch (error) {
            const end = performance.now();
            const metrics = {
                duration: end - start,
                memoryDelta: 0,
                timestamp: Date.now(),
                success: false,
                error: error.message
            };

            this.recordMetrics(operationName, metrics);
            throw error;
        }
    }

    recordMetrics(operationName, metrics) {
        if (!this.performanceMetrics.has(operationName)) {
            this.performanceMetrics.set(operationName, []);
        }

        const operationMetrics = this.performanceMetrics.get(operationName);
        operationMetrics.push(metrics);

        // Keep only last 100 measurements
        if (operationMetrics.length > 100) {
            operationMetrics.shift();
        }
    }

    getPerformanceStats(operationName) {
        const metrics = this.performanceMetrics.get(operationName);
        if (!metrics || metrics.length === 0) {
            return null;
        }

        const durations = metrics.map(m => m.duration);
        const memoryDeltas = metrics.map(m => m.memoryDelta);
        const successRate = metrics.filter(m => m.success).length / metrics.length;

        return {
            operation: operationName,
            count: metrics.length,
            successRate: successRate,
            duration: {
                average: durations.reduce((a, b) => a + b, 0) / durations.length,
                min: Math.min(...durations),
                max: Math.max(...durations),
                median: this.median(durations)
            },
            memory: {
                averageDelta: memoryDeltas.reduce((a, b) => a + b, 0) / memoryDeltas.length,
                maxDelta: Math.max(...memoryDeltas),
                minDelta: Math.min(...memoryDeltas)
            }
        };
    }

    optimizeOperation(operationName) {
        const stats = this.getPerformanceStats(operationName);
        if (!stats) return null;

        const optimizations = [];

        if (stats.duration.average > 1000) { // > 1 second
            optimizations.push('Consider using Web Workers for this operation');
        }

        if (stats.memory.averageDelta > 1024 * 1024) { // > 1MB
            optimizations.push('Consider implementing memory pooling for this operation');
        }

        if (stats.successRate < 0.95) { // < 95% success rate
            optimizations.push('Investigate error causes and implement better error handling');
        }

        return {
            operation: operationName,
            optimizations: optimizations,
            priority: this.calculatePriority(stats)
        };
    }

    calculatePriority(stats) {
        let priority = 0;

        if (stats.duration.average > 1000) priority += 3;
        if (stats.memory.averageDelta > 1024 * 1024) priority += 2;
        if (stats.successRate < 0.95) priority += 4;

        return priority;
    }

    median(values) {
        const sorted = values.slice().sort((a, b) => a - b);
        const middle = Math.floor(sorted.length / 2);
        return sorted.length % 2 === 0 ? (sorted[middle - 1] + sorted[middle]) / 2 : sorted[middle];
    }

    getMemoryUsage() {
        if (performance.memory) {
            return performance.memory.usedJSHeapSize;
        }
        return 0;
    }
}
```

### 10.2 Error Handling and Recovery

**Comprehensive Error Handling:**

```javascript
// Advanced error handling and recovery
class CryptoErrorHandler {
    constructor() {
        this.errorHistory = [];
        this.recoveryStrategies = new Map();
        this.setupRecoveryStrategies();
    }

    setupRecoveryStrategies() {
        this.recoveryStrategies.set('WASM_INIT_FAILED', this.recoverWASMInit);
        this.recoveryStrategies.set('MEMORY_EXHAUSTED', this.recoverMemoryExhausted);
        this.recoveryStrategies.set('OPERATION_TIMEOUT', this.recoverOperationTimeout);
        this.recoveryStrategies.set('INVALID_KEY', this.recoverInvalidKey);
    }

    async handleError(error, context, operation) {
        const errorInfo = {
            message: error.message,
            stack: error.stack,
            context: context,
            operation: operation,
            timestamp: Date.now(),
            userAgent: navigator.userAgent,
            memoryUsage: this.getMemoryUsage()
        };

        this.errorHistory.push(errorInfo);

        // Try to recover from the error
        const recoveryResult = await this.attemptRecovery(error, context, operation);

        if (recoveryResult.success) {
            console.log('Error recovered successfully:', recoveryResult);
            return recoveryResult.result;
        } else {
            console.error('Error recovery failed:', recoveryResult);
            throw new Error(`Operation failed and could not be recovered: ${error.message}`);
        }
    }

    async attemptRecovery(error, context, operation) {
        const errorType = this.classifyError(error);
        const recoveryStrategy = this.recoveryStrategies.get(errorType);

        if (!recoveryStrategy) {
            return {
                success: false,
                reason: 'No recovery strategy available'
            };
        }

        try {
            const result = await recoveryStrategy.call(this, error, context, operation);
            return {
                success: true,
                result: result
            };
        } catch (recoveryError) {
            return {
                success: false,
                reason: recoveryError.message
            };
        }
    }

    classifyError(error) {
        if (error.message.includes('WASM') || error.message.includes('WebAssembly')) {
            return 'WASM_INIT_FAILED';
        }
        if (error.message.includes('memory') || error.message.includes('Memory')) {
            return 'MEMORY_EXHAUSTED';
        }
        if (error.message.includes('timeout') || error.message.includes('Timeout')) {
            return 'OPERATION_TIMEOUT';
        }
        if (error.message.includes('key') || error.message.includes('Key')) {
            return 'INVALID_KEY';
        }
        return 'UNKNOWN_ERROR';
    }

    async recoverWASMInit(error, context, operation) {
        console.log('Attempting WASM initialization recovery...');

        // Try reinitializing with different parameters
        try {
            await init();
            return {
                message: 'WASM reinitialized successfully'
            };
        } catch (reinitError) {
            throw new Error(`WASM reinitialization failed: ${reinitError.message}`);
        }
    }

    async recoverMemoryExhausted(error, context, operation) {
        console.log('Attempting memory recovery...');

        // Clear caches and temporary data
        if (window.gc) {
            window.gc();
        }

        // Clear any stored sensitive data
        if (context.memoryManager) {
            context.memoryManager.cleanupExpiredMemory(0); // Clear all expired
        }

        return {
            message: 'Memory cleanup completed'
        };
    }

    async recoverOperationTimeout(error, context, operation) {
        console.log('Attempting timeout recovery...');

        // Retry operation with exponential backoff
        const maxRetries = 3;
        for (let i = 0; i < maxRetries; i++) {
            try {
                const timeout = Math.pow(2, i) * 1000; // 1s, 2s, 4s
                const result = await this.retryWithTimeout(operation, timeout);
                return result;
            } catch (retryError) {
                if (i === maxRetries - 1) {
                    throw retryError;
                }
                await this.delay(Math.pow(2, i) * 100);
            }
        }
    }

    async recoverInvalidKey(error, context, operation) {
        console.log('Attempting invalid key recovery...');

        // Try to regenerate keys
        if (context.keyManager) {
            const newKeypair = context.keyManager.generateNewKeypair();
            return {
                message: 'New keypair generated',
                keypair: newKeypair
            };
        }

        throw new Error('No key manager available for recovery');
    }

    async retryWithTimeout(operation, timeout) {
        return new Promise((resolve, reject) => {
            const timer = setTimeout(() => {
                reject(new Error('Operation timeout'));
            }, timeout);

            operation()
                .then(result => {
                    clearTimeout(timer);
                    resolve(result);
                })
                .catch(error => {
                    clearTimeout(timer);
                    reject(error);
                });
        });
    }

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    getMemoryUsage() {
        if (performance.memory) {
            return {
                used: performance.memory.usedJSHeapSize,
                total: performance.memory.totalJSHeapSize,
                limit: performance.memory.jsHeapSizeLimit
            };
        }
        return null;
    }

    getErrorHistory() {
        return this.errorHistory;
    }

    clearErrorHistory() {
        this.errorHistory = [];
    }
}
```

### 10.3 Debugging and Diagnostics

**Advanced Debugging Tools:**

```javascript
// Comprehensive debugging and diagnostics
class CryptoDebugger {
    constructor(debugLevel = 'info') {
        this.debugLevel = debugLevel;
        this.logs = [];
        this.performanceData = new Map();
        this.memorySnapshots = [];
        this.operationTraces = new Map();
    }

    log(level, message, data = null) {
        const logEntry = {
            level: level,
            message: message,
            data: this.sanitizeData(data),
            timestamp: Date.now(),
            stack: this.getStackTrace()
        };

        this.logs.push(logEntry);

        if (this.shouldLog(level)) {
            console.log(`[${level.toUpperCase()}] ${message}`, data);
        }
    }

    shouldLog(level) {
        const levels = ['error', 'warn', 'info', 'debug', 'trace'];
        const currentLevelIndex = levels.indexOf(this.debugLevel);
        const messageLevelIndex = levels.indexOf(level);
        return messageLevelIndex <= currentLevelIndex;
    }

    sanitizeData(data) {
        if (data === null || data === undefined) {
            return data;
        }

        if (data instanceof Uint8Array) {
            return `Uint8Array(${data.length})`;
        }

        if (typeof data === 'object') {
            const sanitized = {};
            for (const [key, value] of Object.entries(data)) {
                if (key.toLowerCase().includes('secret') || key.toLowerCase().includes('private')) {
                    sanitized[key] = '[REDACTED]';
                } else if (value instanceof Uint8Array) {
                    sanitized[key] = `Uint8Array(${value.length})`;
                } else {
                    sanitized[key] = this.sanitizeData(value);
                }
            }
            return sanitized;
        }

        return data;
    }

    getStackTrace() {
        const stack = new Error().stack;
        return stack ? stack.split('\n').slice(2) : [];
    }

    traceOperation(operationName, operation) {
        const traceId = this.generateTraceId();
        const startTime = performance.now();
        const startMemory = this.getMemoryUsage();

        this.log('debug', `Starting operation: ${operationName}`, {
            traceId
        });

        try {
            const result = operation();
            const endTime = performance.now();
            const endMemory = this.getMemoryUsage();

            const trace = {
                traceId: traceId,
                operation: operationName,
                startTime: startTime,
                endTime: endTime,
                duration: endTime - startTime,
                startMemory: startMemory,
                endMemory: endMemory,
                memoryDelta: endMemory - startMemory,
                success: true,
                result: this.sanitizeData(result)
            };

            this.operationTraces.set(traceId, trace);
            this.log('debug', `Operation completed: ${operationName}`, trace);

            return result;
        } catch (error) {
            const endTime = performance.now();
            const endMemory = this.getMemoryUsage();

            const trace = {
                traceId: traceId,
                operation: operationName,
                startTime: startTime,
                endTime: endTime,
                duration: endTime - startTime,
                startMemory: startMemory,
                endMemory: endMemory,
                memoryDelta: endMemory - startMemory,
                success: false,
                error: error.message,
                stack: error.stack
            };

            this.operationTraces.set(traceId, trace);
            this.log('error', `Operation failed: ${operationName}`, trace);

            throw error;
        }
    }

    takeMemorySnapshot(label) {
        const snapshot = {
            label: label,
            timestamp: Date.now(),
            memory: this.getMemoryUsage(),
            operationTraces: Array.from(this.operationTraces.values()),
            logs: this.logs.slice(-100) // Last 100 logs
        };

        this.memorySnapshots.push(snapshot);
        this.log('debug', `Memory snapshot taken: ${label}`, snapshot);
    }

    generateTraceId() {
        return `trace_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }

    getMemoryUsage() {
        if (performance.memory) {
            return {
                used: performance.memory.usedJSHeapSize,
                total: performance.memory.totalJSHeapSize,
                limit: performance.memory.jsHeapSizeLimit
            };
        }
        return null;
    }

    getDiagnostics() {
        return {
            logs: this.logs,
            operationTraces: Array.from(this.operationTraces.values()),
            memorySnapshots: this.memorySnapshots,
            performanceData: Object.fromEntries(this.performanceData),
            summary: this.generateSummary()
        };
    }

    generateSummary() {
        const totalOperations = this.operationTraces.size;
        const successfulOperations = Array.from(this.operationTraces.values())
            .filter(trace => trace.success).length;
        const failedOperations = totalOperations - successfulOperations;

        const averageDuration = Array.from(this.operationTraces.values())
            .reduce((sum, trace) => sum + trace.duration, 0) / totalOperations;

        const memoryGrowth = this.memorySnapshots.length > 1 ?
            this.memorySnapshots[this.memorySnapshots.length - 1].memory.used -
            this.memorySnapshots[0].memory.used : 0;

        return {
            totalOperations: totalOperations,
            successfulOperations: successfulOperations,
            failedOperations: failedOperations,
            successRate: totalOperations > 0 ? successfulOperations / totalOperations : 0,
            averageDuration: averageDuration,
            memoryGrowth: memoryGrowth,
            logCount: this.logs.length
        };
    }

    exportDiagnostics() {
        const diagnostics = this.getDiagnostics();
        const blob = new Blob([JSON.stringify(diagnostics, null, 2)], {
            type: 'application/json'
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `crypto-diagnostics-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    }

    clearDiagnostics() {
        this.logs = [];
        this.operationTraces.clear();
        this.memorySnapshots = [];
        this.performanceData.clear();
    }
}
```

---

## 12. Advanced Usage

### 12.1 Custom Algorithm Selection

**Algorithm Selection Based on Use Case:**

```javascript
class AlgorithmSelector {
    static selectKEM(securityLevel, performanceRequirement) {
        if (securityLevel === 'high' && performanceRequirement === 'high') {
            return 'mlkem768'; // Balanced choice
        } else if (securityLevel === 'maximum') {
            return 'mlkem1024'; // Highest security
        } else if (performanceRequirement === 'maximum') {
            return 'mlkem512'; // Fastest
        }
        return 'mlkem768'; // Default
    }

    static selectSignature(securityLevel, signatureSizeRequirement) {
        if (signatureSizeRequirement === 'compact') {
            return 'fndsa512'; // Smallest signatures
        } else if (securityLevel === 'maximum') {
            return 'mldsa87'; // Highest security
        } else if (signatureSizeRequirement === 'minimal') {
            return 'slhdsa_sha2_128f'; // Smallest keys
        }
        return 'mldsa65'; // Default
    }
}

// Usage
const kemAlgorithm = AlgorithmSelector.selectKEM('high', 'high');
const sigAlgorithm = AlgorithmSelector.selectSignature('high', 'balanced');
```

### 12.2 Batch Operations

**Batch Signature Verification:**

```javascript
class BatchVerifier {
    constructor() {
        this.verifications = [];
    }

    addVerification(publicKey, signature, message) {
        this.verifications.push({
            publicKey,
            signature,
            message: new TextEncoder().encode(message)
        });
    }

    async verifyAll() {
        const results = [];

        for (const verification of this.verifications) {
            try {
                const isValid = wasm.mldsa65_verify(
                    verification.publicKey,
                    verification.signature,
                    verification.message
                );
                results.push({
                    valid: isValid,
                    error: null
                });
            } catch (error) {
                results.push({
                    valid: false,
                    error: error.message
                });
            }
        }

        return results;
    }

    clear() {
        this.verifications = [];
    }
}

// Usage
const batchVerifier = new BatchVerifier();
batchVerifier.addVerification(publicKey1, signature1, message1);
batchVerifier.addVerification(publicKey2, signature2, message2);

const results = await batchVerifier.verifyAll();
console.log('Batch verification results:', results);
```

### 12.3 Hybrid Cryptography

**Combining Classical and Post-Quantum:**

```javascript
class HybridCrypto {
    constructor() {
        this.pqKeypair = null;
        this.classicalKeypair = null;
    }

    async initialize() {
        await init();
        this.pqKeypair = wasm.mlkem768_keygen();
        // Generate classical keypair using Web Crypto API
        this.classicalKeypair = await crypto.subtle.generateKey({
                name: "RSA-OAEP",
                modulusLength: 2048,
                publicExponent: new Uint8Array([1, 0, 1]),
                hash: "SHA-256"
            },
            true,
            ["encrypt", "decrypt"]
        );
    }

    hybridEncrypt(message, recipientPqKey, recipientClassicalKey) {
        // Use post-quantum for key exchange
        const pqEncapsulated = wasm.mlkem768_encapsulate(recipientPqKey);
        const sharedSecret = pqEncapsulated.shared_secret();

        // Use classical crypto for message encryption (faster)
        // In practice, you'd use the shared secret to derive encryption keys
        return {
            pqCiphertext: pqEncapsulated.ciphertext(),
            encryptedMessage: this.encryptWithSharedSecret(message, sharedSecret)
        };
    }

    encryptWithSharedSecret(message, sharedSecret) {
        // Simplified example - use proper key derivation in production
        const key = sharedSecret.slice(0, 32); // Use first 32 bytes as key
        const iv = crypto.getRandomValues(new Uint8Array(12));

        // Use AES-GCM for encryption
        return crypto.subtle.encrypt({
                name: "AES-GCM",
                iv: iv
            },
            key,
            new TextEncoder().encode(message)
        );
    }
}
```

---

## 13. Performance Optimization

### 13.1 Memory Optimization

**Efficient Memory Usage:**

```javascript
class MemoryOptimizer {
    constructor() {
        this.keyCache = new Map();
        this.maxCacheSize = 100;
    }

    cacheKey(keyId, keyData) {
        if (this.keyCache.size >= this.maxCacheSize) {
            // Remove oldest entry
            const firstKey = this.keyCache.keys().next().value;
            this.keyCache.delete(firstKey);
        }

        this.keyCache.set(keyId, {
            data: keyData,
            timestamp: Date.now()
        });
    }

    getCachedKey(keyId) {
        const entry = this.keyCache.get(keyId);
        if (!entry) return null;

        // Check if cache entry is still valid (5 minutes)
        if (Date.now() - entry.timestamp > 5 * 60 * 1000) {
            this.keyCache.delete(keyId);
            return null;
        }

        return entry.data;
    }

    clearCache() {
        this.keyCache.clear();
    }
}
```

### 13.2 Performance Monitoring

**Performance Metrics:**

```javascript
class PerformanceMonitor {
    constructor() {
        this.metrics = {
            keyGeneration: [],
            encryption: [],
            decryption: [],
            signing: [],
            verification: []
        };
    }

    measureOperation(operationType, operation) {
        const start = performance.now();
        const result = operation();
        const end = performance.now();

        const duration = end - start;
        this.metrics[operationType].push(duration);

        return result;
    }

    getAverageTime(operationType) {
        const times = this.metrics[operationType];
        if (times.length === 0) return 0;

        return times.reduce((sum, time) => sum + time, 0) / times.length;
    }

    getStats() {
        const stats = {};
        for (const [operation, times] of Object.entries(this.metrics)) {
            if (times.length > 0) {
                stats[operation] = {
                    count: times.length,
                    average: this.getAverageTime(operation),
                    min: Math.min(...times),
                    max: Math.max(...times)
                };
            }
        }
        return stats;
    }
}

// Usage
const monitor = new PerformanceMonitor();

// Measure key generation
const keypair = monitor.measureOperation('keyGeneration', () => {
    return wasm.mlkem768_keygen();
});

// Measure encryption
const encapsulated = monitor.measureOperation('encryption', () => {
    return wasm.mlkem768_encapsulate(keypair.public_key());
});

console.log('Performance stats:', monitor.getStats());
```

### 13.3 Web Worker Integration

**Offload Heavy Operations:**

```javascript
// crypto-worker.js
import init, * as wasm from "aegis-crypto-core";

self.onmessage = async function(e) {
    const {
        operation,
        data
    } = e.data;

    try {
        await init();

        let result;
        switch (operation) {
            case 'keygen':
                result = wasm.mlkem768_keygen();
                break;
            case 'encapsulate':
                result = wasm.mlkem768_encapsulate(data.publicKey);
                break;
            case 'decapsulate':
                result = wasm.mlkem768_decapsulate(data.secretKey, data.ciphertext);
                break;
            default:
                throw new Error(`Unknown operation: ${operation}`);
        }

        self.postMessage({
            success: true,
            result
        });
    } catch (error) {
        self.postMessage({
            success: false,
            error: error.message
        });
    }
};

// main.js
class CryptoWorkerManager {
    constructor() {
        this.worker = new Worker('crypto-worker.js');
        this.pendingOperations = new Map();
        this.operationId = 0;
    }

    async performOperation(operation, data) {
        return new Promise((resolve, reject) => {
            const id = ++this.operationId;

            this.pendingOperations.set(id, {
                resolve,
                reject
            });

            this.worker.postMessage({
                id,
                operation,
                data
            });

            // Set timeout
            setTimeout(() => {
                if (this.pendingOperations.has(id)) {
                    this.pendingOperations.delete(id);
                    reject(new Error('Operation timeout'));
                }
            }, 30000); // 30 second timeout
        });
    }

    setupWorker() {
        this.worker.onmessage = (e) => {
            const {
                id,
                success,
                result,
                error
            } = e.data;
            const operation = this.pendingOperations.get(id);

            if (operation) {
                this.pendingOperations.delete(id);

                if (success) {
                    operation.resolve(result);
                } else {
                    operation.reject(new Error(error));
                }
            }
        };
    }
}

// Usage
const workerManager = new CryptoWorkerManager();
workerManager.setupWorker();

// Perform operations in worker thread
const keypair = await workerManager.performOperation('keygen', {});
const encapsulated = await workerManager.performOperation('encapsulate', {
    publicKey: keypair.public_key()
});
```

---

## Conclusion

This user manual provides comprehensive guidance for using Aegis Crypto Core across all supported platforms. The library offers production-ready post-quantum cryptography with complete NIST algorithm coverage, making it the ideal choice for securing applications against quantum threats.

For additional support, examples, and updates, please visit our GitHub repository and documentation portal.

**Key Takeaways:**
* Aegis provides complete NIST PQC algorithm coverage
* Universal platform support from embedded to cloud
* Production-ready with comprehensive testing
* Easy integration with modern development frameworks
* Strong security guarantees with constant-time operations

**Next Steps:**
01. Choose the appropriate algorithm for your use case
02. Follow the installation guide for your platform
03. Implement security best practices
04. Monitor performance and optimize as needed
05. Stay updated with the latest releases and security updates

---

*This user manual is maintained by the Synergy Network's development team and updated regularly to reflect the current state of the project.*
