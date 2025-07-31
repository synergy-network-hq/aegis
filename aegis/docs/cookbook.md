# Aegis Crypto Core Cookbook

This cookbook provides practical examples and code snippets for common cryptographic tasks using Aegis Crypto Core.

## Table of Contents

1.  [Key Generation for All Algorithms](#key-generation-for-all-algorithms)
2.  [Signing and Verifying Data (Dilithium, SPHINCS+, Falcon)](#signing-and-verifying-data-dilithium-sphincs-falcon)
3.  [Encrypting and Decrypting Data (Kyber)](#encrypting-and-decrypting-data-kyber)
4.  [Hashing Data](#hashing-data)
5.  [Hexadecimal Conversions](#hexadecimal-conversions)

---

## 1. Key Generation for All Algorithms

Generating cryptographic key pairs is the first step for most operations. Aegis Crypto Core provides dedicated functions for each algorithm.

```javascript
import init, * as wasm from "../pkg/aegis_crypto_core.js";

async function generateKeys() {
    await init();

    console.log("--- Generating Key Pairs ---");

    // Dilithium Key Pair
    const dilithiumKeypair = wasm.dilithium_keygen();
    console.log("Dilithium Public Key (hex):", wasm.bytes_to_hex(dilithiumKeypair.public_key));
    console.log("Dilithium Secret Key (hex):", wasm.bytes_to_hex(dilithiumKeypair.secret_key));

    // Kyber Key Pair
    const kyberKeypair = wasm.kyber_keygen();
    console.log("Kyber Public Key (hex):", wasm.bytes_to_hex(kyberKeypair.public_key));
    console.log("Kyber Secret Key (hex):", wasm.bytes_to_hex(kyberKeypair.secret_key));

    // SPHINCS+ Key Pair
    const sphincsKeypair = wasm.sphincsplus_keygen();
    console.log("SPHINCS+ Public Key (hex):", wasm.bytes_to_hex(sphincsKeypair.public_key));
    console.log("SPHINCS+ Secret Key (hex):", wasm.bytes_to_hex(sphincsKeypair.secret_key));

    // Falcon Key Pair
    const falconKeypair = wasm.falcon_keygen();
    console.log("Falcon Public Key (hex):", wasm.bytes_to_hex(falconKeypair.public_key));
    console.log("Falcon Secret Key (hex):", wasm.bytes_to_hex(falconKeypair.secret_key));
}

generateKeys();
```

---

## 2. Signing and Verifying Data (Dilithium, SPHINCS+, Falcon)

Digital signatures ensure data integrity and authenticity. Here's how to sign and verify messages.

```javascript
import init, * as wasm from "../pkg/aegis_crypto_core.js";

async function signAndVerify() {
    await init();

    const message = new TextEncoder().encode("This is the data to be signed.");

    console.log("--- Signing and Verifying ---");

    // Dilithium
    const dilithiumKeypair = wasm.dilithium_keygen();
    const dilithiumSignature = wasm.dilithium_sign(dilithiumKeypair.secret_key, message);
    const dilithiumValid = wasm.dilithium_verify(dilithiumKeypair.public_key, message, dilithiumSignature);
    console.log("Dilithium Signature Valid:", dilithiumValid);

    // SPHINCS+
    const sphincsKeypair = wasm.sphincsplus_keygen();
    const sphincsSignature = wasm.sphincsplus_sign(sphincsKeypair.secret_key, message);
    const sphincsValid = wasm.sphincsplus_verify(sphincsKeypair.public_key, message, sphincsSignature);
    console.log("SPHINCS+ Signature Valid:", sphincsValid);

    // Falcon
    const falconKeypair = wasm.falcon_keygen();
    const falconSignature = wasm.falcon_sign(falconKeypair.secret_key, message);
    const falconValid = wasm.falcon_verify(falconKeypair.public_key, message, falconSignature);
    console.log("Falcon Signature Valid:", falconValid);

    // Example of invalid signature verification
    const tamperedMessage = new TextEncoder().encode("This is tampered data.");
    const invalidDilithiumValid = wasm.dilithium_verify(dilithiumKeypair.public_key, tamperedMessage, dilithiumSignature);
    console.log("Dilithium Tampered Signature Valid (expected false):", invalidDilithiumValid);
}

signAndVerify();
```

---

## 3. Encrypting and Decrypting Data (Kyber)

Kyber is a Key Encapsulation Mechanism (KEM) used for establishing shared secrets securely.

```javascript
import init, * as wasm from "../pkg/aegis_crypto_core.js";

async function encryptDecryptKyber() {
    await init();

    console.log("--- Kyber Encapsulation and Decapsulation ---");

    // Generate Kyber key pair for the recipient
    const recipientKeypair = wasm.kyber_keygen();

    // Sender encapsulates a shared secret using recipient's public key
    const encapsulated = wasm.kyber_encapsulate(recipientKeypair.public_key);
    const ciphertext = encapsulated.ciphertext;
    const senderSharedSecret = encapsulated.shared_secret;
    console.log("Sender's Shared Secret (hex):", wasm.bytes_to_hex(senderSharedSecret));

    // Recipient decapsulates the shared secret using their secret key and the ciphertext
    const recipientSharedSecret = wasm.kyber_decapsulate(recipientKeypair.secret_key, ciphertext);
    console.log("Recipient's Shared Secret (hex):", wasm.bytes_to_hex(recipientSharedSecret));

    // Verify that both shared secrets match
    const secretsMatch = senderSharedSecret.toString() === recipientSharedSecret.toString();
    console.log("Shared Secrets Match:", secretsMatch);
}

encryptDecryptKyber();
```

---

## 4. Hashing Data

Hash functions are used to create fixed-size digests of data, useful for integrity checks.

```javascript
import init, * as wasm from "../pkg/aegis_crypto_core.js";

async function hashData() {
    await init();

    const dataToHash = new TextEncoder().encode("This is some data to compute hashes for.");

    console.log("--- Hashing Data ---");

    // SHA3-256
    const sha3_256_hash = wasm.sha3_256_hash(dataToHash);
    console.log("SHA3-256 Hash:", wasm.bytes_to_hex(sha3_256_hash));

    // SHA3-512
    const sha3_512_hash = wasm.sha3_512_hash(dataToHash);
    console.log("SHA3-512 Hash:", wasm.bytes_to_hex(sha3_512_hash));

    // BLAKE3
    const blake3_hash = wasm.blake3_hash(dataToHash);
    console.log("BLAKE3 Hash:", wasm.bytes_to_hex(blake3_hash));
}

hashData();
```

---

## 5. Hexadecimal Conversions

Convert between `Uint8Array` (byte arrays) and hexadecimal string representations.

```javascript
import init, * as wasm from "../pkg/aegis_crypto_core.js";

async function hexConversions() {
    await init();

    console.log("--- Hexadecimal Conversions ---");

    const hexString = "0123456789abcdef";
    console.log("Original Hex String:", hexString);

    // Convert hex string to bytes
    const bytes = wasm.hex_to_bytes(hexString);
    console.log("Bytes (Uint8Array):");
    console.log(bytes);

    // Convert bytes back to hex string
    const convertedHexString = wasm.bytes_to_hex(bytes);
    console.log("Converted Hex String:", convertedHexString);

    console.log("Conversion successful:", hexString === convertedHexString);
}

hexConversions();
```


