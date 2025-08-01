# General-Purpose Post-Quantum Cryptography for JS/TS Applications

This document serves as a comprehensive guide for developers looking to integrate post-quantum cryptography into their JavaScript and TypeScript applications using Aegis Crypto Core. Whether you are building a Node.js backend, a client-side web application, or a cross-platform tool, Aegis provides a powerful and accessible way to secure your application against the threat of quantum computers.

## Overview

Aegis Crypto Core is a high-performance cryptographic library, compiled to WebAssembly (WASM), that exposes a suite of NIST-standardized post-quantum algorithms. By leveraging WASM, Aegis runs at near-native speed in any environment that supports the WASM runtime, including all modern browsers and Node.js versions.

The library provides two main categories of cryptographic primitives:

1.  **Key Encapsulation Mechanisms (KEMs):** Used for establishing a shared secret between two parties over an insecure channel. This is the foundation for building secure communication protocols (e.g., post-quantum TLS).
    *   **Kyber (ML-KEM):** The NIST standard for general-purpose KEMs.

2.  **Digital Signature Algorithms:** Used for verifying the authenticity and integrity of data. This is essential for secure software updates, identity verification, and legally binding digital documents.
    *   **Dilithium (ML-DSA):** The NIST standard for general-purpose digital signatures.
    *   **Falcon:** An alternative signature scheme that offers significantly smaller signatures, making it ideal for bandwidth-constrained environments.
    *   **SPHINCS+:** A hash-based signature scheme with the most conservative security assumptions, providing a robust fallback option.

## Getting Started

### Installation

Aegis Crypto Core is distributed as an NPM package. To add it to your project, you can use `npm` or `yarn`:

```bash
npm install /path/to/aegis_crypto_core/pkg
# or
yarn add /path/to/aegis_crypto_core/pkg
```

For local development, you can also use `npm link` to create a symbolic link to your local build of the library.

### Initialization

Before any cryptographic functions can be called, the WebAssembly module must be initialized. This is an asynchronous operation.

```javascript
import init from "aegis_crypto_core";

async function initialize() {
    try {
        // In a browser, `init` can often be called without arguments
        // if the .wasm file is served from the same origin.
        await init();
        console.log("Aegis Crypto Core initialized successfully.");
    } catch (error) {
        console.error("Failed to initialize Aegis Crypto Core:", error);
    }
}

initialize();
```

In a Node.js environment, you will need to load the `.wasm` file from the filesystem:

```javascript
const fs = require("fs");
const path = require("path");
const init = require("aegis_crypto_core");

async function initializeNode() {
    try {
        const wasmPath = path.resolve(__dirname, "node_modules/aegis_crypto_core/aegis_crypto_core_bg.wasm");
        const wasmBytes = fs.readFileSync(wasmPath);
        await init(wasmBytes);
        console.log("Aegis Crypto Core initialized successfully.");
    } catch (error) {
        console.error("Failed to initialize Aegis Crypto Core:", error);
    }
}

initializeNode();
```

## Usage Guide for Cryptographic Functions

Below are detailed examples of how to use each of the core cryptographic primitives provided by Aegis.

### Kyber Key Encapsulation Mechanism (KEM)

Kyber is used to establish a secure channel. One party (the sender) encapsulates a secret against the other party's (the receiver's) public key. The receiver can then decapsulate it with their secret key.

1.  **Key Generation (Receiver):** The receiver generates a Kyber key pair.

    ```javascript
    const kyberKeyPair = aegis.kyber_keygen();
    // kyberKeyPair.publicKey should be sent to the sender
    // kyberKeyPair.secretKey must be kept private
    ```

2.  **Encapsulation (Sender):** The sender uses the receiver's public key to generate a ciphertext and a shared secret.

    ```javascript
    const { ciphertext, shared_secret: sharedSecretSender } = aegis.kyber_encapsulate(kyberKeyPair.publicKey);
    // The `ciphertext` is sent to the receiver.
    // `sharedSecretSender` is now used to encrypt messages.
    ```

3.  **Decapsulation (Receiver):** The receiver uses their private key and the received ciphertext to derive the same shared secret.

    ```javascript
    const sharedSecretReceiver = aegis.kyber_decapsulate(kyberKeyPair.secretKey, ciphertext);
    // `sharedSecretReceiver` will be identical to `sharedSecretSender`.
    ```

### Digital Signature Algorithms

Digital signatures are used to sign and verify messages.

#### Dilithium

1.  **Key Generation:**

    ```javascript
    const dilithiumKeyPair = aegis.dilithium_keygen();
    ```

2.  **Signing:**

    ```javascript
    const message = new TextEncoder().encode("This message needs to be signed.");
    const signature = aegis.dilithium_sign(dilithiumKeyPair.secret_key, message);
    ```

3.  **Verification:**

    ```javascript
    const isValid = aegis.dilithium_verify(dilithiumKeyPair.public_key, message, signature);
    console.log("Signature is valid:", isValid); // true
    ```

#### Falcon

Falcon follows the same API pattern as Dilithium but produces much smaller signatures.

1.  **Key Generation:**

    ```javascript
    const falconKeyPair = aegis.falcon_keygen();
    ```

2.  **Signing:**

    ```javascript
    const message = new TextEncoder().encode("This message needs to be signed.");
    const signature = aegis.falcon_sign(falconKeyPair.secret_key, message);
    ```

3.  **Verification:**

    ```javascript
    const isValid = aegis.falcon_verify(falconKeyPair.public_key, message, signature);
    console.log("Signature is valid:", isValid); // true
    ```

#### SPHINCS+

SPHINCS+ also follows the same API pattern. Note its significantly larger signature size.

1.  **Key Generation:**

    ```javascript
    const sphincsplusKeyPair = aegis.sphincsplus_keygen();
    ```

2.  **Signing:**

    ```javascript
    const message = new TextEncoder().encode("This message needs to be signed.");
    const signature = aegis.sphincsplus_sign(sphincsplusKeyPair.secret_key, message);
    ```

3.  **Verification:**

    ```javascript
    const isValid = aegis.sphincsplus_verify(sphincsplusKeyPair.public_key, message, signature);
    console.log("Signature is valid:", isValid); // true
    ```

## Compatibility

Aegis Crypto Core is designed for broad compatibility across the JavaScript ecosystem. For detailed information on supported Node.js versions and browsers, please refer to the `compatibility_matrix.md` document.

## Security Best Practices

*   **Secret Key Management:** The security of all cryptographic operations depends on the secrecy of the private keys. **Never** expose secret keys in client-side code or commit them to version control. Use secure storage mechanisms like HSMs, secure enclaves, or environment variables on the server-side.
*   **Randomness:** The key generation functions rely on the host environment's source of randomness (`crypto.getRandomValues` in browsers, `crypto.randomBytes` in Node.js). Ensure your target environment provides a cryptographically secure random number generator.
*   **Algorithm Choice:** Choose the algorithm that best fits your application's needs. Consider the trade-offs between key size, signature size, and performance.
    *   Use **Kyber** for establishing shared secrets.
    *   Use **Dilithium** as a general-purpose, high-security signature scheme.
    *   Use **Falcon** when signature size is a primary concern.
    *   Use **SPHINCS+** for applications requiring the most conservative security assumptions, where its large signature size is acceptable.

By following this guide, developers can effectively use Aegis Crypto Core to build secure, forward-compatible applications protected against the threat of quantum computers.

