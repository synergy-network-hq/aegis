# Quickstart Guide

This guide will help you get started with Aegis Crypto Core in both Node.js and browser environments.

## Installation

### From npm (when published)

```bash
npm install aegis-crypto-core
```

### Building from source

1. Install Rust and wasm-pack:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

2. Build the project:

```bash
wasm-pack build --target web
```

## Node.js Usage

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
    const message = new TextEncoder().encode("Hello, world!");
    const signature = wasm.dilithium_sign(keypair.secret_key, message);
    
    // Verify the signature
    const isValid = wasm.dilithium_verify(keypair.public_key, message, signature);
    console.log("Signature valid:", isValid);
}

main();
```

## Browser Usage

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

## Available Algorithms

- **Kyber (ML-KEM-768)**: Key encapsulation mechanism
- **Dilithium (ML-DSA-65)**: Digital signature algorithm
- **SPHINCS+ (SLH-DSA)**: Hash-based signature scheme
- **Falcon (FN-DSA)**: Compact lattice-based signatures

## Hash Functions

- **SHA3-256** and **SHA3-512**: Secure hash algorithms
- **BLAKE3**: Fast cryptographic hash function

## Utilities

- **hex_to_bytes()**: Convert hex strings to byte arrays
- **bytes_to_hex()**: Convert byte arrays to hex strings

## Next Steps

- Check out the [examples](../examples/) directory for complete working demos
- Read the [cookbook](./cookbook.md) for common use cases
- See the [API Reference](../README.md#api-reference) for detailed function documentation

