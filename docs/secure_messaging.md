# Secure Messaging with Aegis Crypto Core

This document provides a comprehensive guide to using the Aegis Crypto Core for secure messaging, leveraging post-quantum cryptographic algorithms to ensure confidentiality and authenticity. The core functionalities include key exchange using Kyber and digital signatures using Dilithium or Falcon. **Note: SPHINCS+ is currently non-functional due to critical upstream library bugs and cannot be used for secure messaging.**

## Overview of Secure Messaging API

The `SecureMessagingAPI` class, available in `secure_messaging_api.js`, provides a high-level interface to perform cryptographic operations necessary for secure communication. It abstracts away the complexities of WASM module initialization, key generation, encryption, decryption, signing, and verification.

### Key Features:

*   **Kyber Key Exchange:** Securely establish shared secrets between two parties.
*   **AES-256-GCM Encryption:** Encrypt messages using a symmetric key derived from Kyber key exchange.
*   **Dilithium/Falcon Digital Signatures:** Authenticate messages to ensure integrity and sender verification.
*   **Combined Operations:** Functions to create and verify messages that are both encrypted and signed.

## Installation and Setup

To use the Secure Messaging API, you need to have Node.js and `wasm-pack` installed. The `aegis_crypto_core` WASM package needs to be built and linked to your project.

1.  **Build the `aegis_crypto_core` WASM package:**

    Navigate to the `aegis_crypto_core` directory and run:

    ```bash
    wasm-pack build --target web
    ```

    This will create a `pkg` directory containing the WASM module and JavaScript bindings.

2.  **Set up your Node.js project:**

    Create a new directory for your project and initialize it:

    ```bash
    mkdir secure_messaging_demo
    cd secure_messaging_demo
    npm init -y
    ```

3.  **Link the `aegis_crypto_core` package:**

    From your `secure_messaging_demo` directory, link the `pkg` directory of `aegis_crypto_core`:

    ```bash
    npm link ../aegis_crypto_core/pkg
    ```

    This makes the `aegis_crypto_core` package available in your Node.js project.

## Usage Examples

The `secure_messaging_api.js` file contains the `SecureMessagingAPI` class, and `secure_messaging_demo.js` provides a comprehensive example of its usage. Below are snippets demonstrating key functionalities.

### Initializing the API

Before using any cryptographic functions, the WASM module must be initialized. This is typically done once at the start of your application.

```javascript
const SecureMessagingAPI = require("./secure_messaging_api");
const path = require("path");
const fs = require("fs");

async function initializeApi() {
    const api = new SecureMessagingAPI();
    const wasmPath = path.resolve(__dirname, "../aegis_crypto_core/pkg/aegis_crypto_core_bg.wasm");
    const wasmBytes = fs.readFileSync(wasmPath);
    await api.init(wasmBytes);
    console.log("Secure Messaging API initialized.");
    return api;
}
```

### Key Generation

Generate Kyber key pairs for encryption/decryption and Dilithium/Falcon key pairs for signing/verification.

```javascript
// Assuming 'api' is an initialized SecureMessagingAPI instance
const aliceKyberKeys = api.generateKyberKeyPair();
const aliceDilithiumKeys = api.generateDilithiumKeyPair();
const aliceFalconKeys = api.generateFalconKeyPair();

const bobKyberKeys = api.generateKyberKeyPair();
const bobDilithiumKeys = api.generateDilithiumKeyPair();
const bobFalconKeys = api.generateFalconKeyPair();

console.log("Alice's Kyber Public Key:", aliceKyberKeys.public_key);
console.log("Bob's Dilithium Secret Key:", bobDilithiumKeys.secret_key);
```

### Basic Encryption and Decryption (Kyber + AES-256-GCM)

This demonstrates how Alice can encrypt a message for Bob using Kyber for key exchange and AES-256-GCM for symmetric encryption.

```javascript
// Alice encrypts a message for Bob
const messageToEncrypt = "Hello Bob! This is a secret message from Alice.";
const encryptedMessage = api.encrypt(bobKyberKeys.public_key, messageToEncrypt);
console.log("Encrypted Message:", encryptedMessage);

// Bob decrypts the message
const decryptedBytes = api.decrypt(bobKyberKeys.secret_key, encryptedMessage);
const decryptedText = new TextDecoder().decode(decryptedBytes);
console.log("Decrypted Message:", decryptedText);
console.log("Encryption/Decryption successful:", messageToEncrypt === decryptedText);
```

### Digital Signatures (Dilithium and Falcon)

Messages can be signed to ensure their authenticity and integrity. The API supports both Dilithium and Falcon signature schemes.

```javascript
// Alice signs a message with Dilithium
const messageToSignDilithium = "This message is signed with Dilithium.";
const dilithiumSignature = api.signWithDilithium(aliceDilithiumKeys.secret_key, messageToSignDilithium);
console.log("Dilithium Signature:", dilithiumSignature);

// Bob verifies the Dilithium signature
const isDilithiumValid = api.verifyDilithiumSignature(aliceDilithiumKeys.public_key, messageToSignDilithium, dilithiumSignature);
console.log("Dilithium Signature Valid:", isDilithiumValid);

// Alice signs a message with Falcon
const messageToSignFalcon = "This message is signed with Falcon.";
const falconSignature = api.signWithFalcon(aliceFalconKeys.secret_key, messageToSignFalcon);
console.log("Falcon Signature:", falconSignature);

// Bob verifies the Falcon signature
const isFalconValid = api.verifyFalconSignature(aliceFalconKeys.public_key, messageToSignFalcon, falconSignature);
console.log("Falcon Signature Valid:", isFalconValid);
```

### Combined Secure Messaging (Encryption + Signature)

For a complete secure messaging solution, messages can be both encrypted and signed. The `createSecureMessage` and `verifySecureMessage` functions handle this combined process.

```javascript
// Alice creates a secure message for Bob using Dilithium for signing
const combinedMessageDilithium = "This is a confidential and authenticated message using Dilithium.";
const secureMessageDilithium = await api.createSecureMessage({
    senderSigningKey: aliceDilithiumKeys.secret_key,
    recipientPublicKey: bobKyberKeys.public_key
}, combinedMessageDilithium, 'dilithium');
console.log("Secure Message (Dilithium):
", secureMessageDilithium);

// Bob verifies and decrypts the secure message
const verifiedDecryptedDilithium = await api.verifySecureMessage({
    recipientSecretKey: bobKyberKeys.secret_key,
    senderVerifyingKey: aliceDilithiumKeys.public_key
}, secureMessageDilithium);
console.log("Verified and Decrypted Message (Dilithium):
", verifiedDecryptedDilithium.message);
console.log("Signature Valid (Dilithium):", verifiedDecryptedDilithium.signatureValid);

// Alice creates a secure message for Bob using Falcon for signing
const combinedMessageFalcon = "This is a confidential and authenticated message using Falcon.";
const secureMessageFalcon = await api.createSecureMessage({
    senderSigningKey: aliceFalconKeys.secret_key,
    recipientPublicKey: bobKyberKeys.public_key
}, combinedMessageFalcon, 'falcon');
console.log("Secure Message (Falcon):
", secureMessageFalcon);

// Bob verifies and decrypts the secure message
const verifiedDecryptedFalcon = await api.verifySecureMessage({
    recipientSecretKey: bobKyberKeys.secret_key,
    senderVerifyingKey: aliceFalconKeys.public_key
}, secureMessageFalcon);
console.log("Verified and Decrypted Message (Falcon):
", verifiedDecryptedFalcon.message);
console.log("Signature Valid (Falcon):", verifiedDecryptedFalcon.signatureValid);
```

## Browser Compatibility

The `aegis_crypto_core` WASM module and the `SecureMessagingAPI` are designed to work in modern web browsers. The browser examples (`kyber_browser_demo.html`, `dilithium_browser_demo.html`, `falcon_browser_demo.html`, `secure_messaging_browser_demo.html`) demonstrate how to import and use the module in a browser environment. Note that the browser environment uses the Web Crypto API for AES-GCM operations, which is handled internally by the `SecureMessagingAPI`.

To run the browser demos, you can use a simple HTTP server (e.g., `http-server` via `npx`):

```bash
npx http-server -p 8000
```

Then, open your browser and navigate to `http://localhost:8000/kyber_browser_demo.html` (or other HTML files) and check the browser's developer console for output.

## TypeScript Type Definitions

The `wasm-pack build --target web` command generates TypeScript type definitions (`aegis_crypto_core.d.ts`) in the `pkg` directory. These definitions provide type hints for all WASM-exported functions and classes, enhancing developer experience in TypeScript projects.

Developers can import these types in their TypeScript files:

```typescript
import init, { kyber_keygen, KyberKeyPair, KyberEncapsulated } from "aegis_crypto_core";

// Example usage with types
async function demo() {
    await init();
    const keyPair: KyberKeyPair = kyber_keygen();
    console.log(keyPair.public_key);
}
```

## Conclusion

The Aegis Crypto Core provides a robust foundation for building secure messaging applications with post-quantum cryptography. By following this guide and utilizing the provided API and examples, developers can integrate advanced cryptographic functionalities into their Node.js and browser-based applications with ease.

