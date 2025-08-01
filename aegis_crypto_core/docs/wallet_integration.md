# Web3/Blockchain Wallet Integration with Aegis Crypto Core

This guide provides comprehensive instructions for integrating Aegis Crypto Core into Web3 and blockchain wallet applications. Leveraging post-quantum cryptography, Aegis ensures that your wallet's key management and transaction signing capabilities are secure against future quantum attacks.

## Overview

Aegis Crypto Core is designed to be a plug-and-play solution for adding post-quantum cryptographic primitives to various applications, including Web3 wallets. The core functionalities relevant to wallet integration include:

*   **Keypair Generation:** Securely generate post-quantum key pairs (Kyber, Dilithium, Falcon, SPHINCS+) directly within the browser environment.
*   **Signing and Verification:** Perform digital signature operations for transactions and messages using Dilithium and Falcon algorithms. SPHINCS+ is currently non-functional due to upstream bugs.
*   **WASM Compatibility:** The entire cryptographic suite is compiled to WebAssembly (WASM), ensuring high performance and compatibility across modern web browsers.
*   **TypeScript Type Definitions:** Comprehensive TypeScript definitions are provided to enhance developer experience and ensure type safety.

## WASM Build for Modern Browsers

The `aegis_crypto_core` is built as a WebAssembly module, making it highly compatible with modern browser environments. This allows for client-side execution of cryptographic operations, enhancing security by keeping private keys and sensitive operations within the user's browser.

To ensure your WASM build is ready for browser use, follow these steps:

1.  **Build with `wasm-pack`:** Navigate to the `aegis_crypto_core` directory and build the project with the `web` target:

    ```bash
    wasm-pack build --target web
    ```

    This command generates the necessary `.wasm` and `.js` files in the `pkg` directory, optimized for web environments.

2.  **Serve the `pkg` directory:** When integrating into a web application, ensure that the `pkg` directory (or its contents) is served by your web server. This allows the browser to load the WASM module and its associated JavaScript glue code.

    For development, you can use a simple HTTP server:

    ```bash
    npx http-server -p 8000
    ```

    Then, your application can import the module using standard ES module syntax.

## Examples: Keypair Generation, Signing, and Verifying in Browser JS

Integrating Aegis Crypto Core into your wallet's frontend involves importing the WASM module and utilizing its exposed functions. Below are examples demonstrating keypair generation, signing, and verification directly in browser JavaScript.

### Importing the Module

In your HTML file or JavaScript module, import the `init` function and the cryptographic primitives from the generated `aegis_crypto_core.js`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Wallet Integration Demo</title>
</head>
<body>
    <h1>Aegis Wallet Integration Demo</h1>
    <p>Open the browser console to see the cryptographic operations.</p>

    <script type="module">
        import init, { kyber_keygen, dilithium_keygen, dilithium_sign, dilithium_verify, falcon_keygen, falcon_sign, falcon_verify } from "./pkg/aegis_crypto_core.js";

        async function runWalletDemo() {
            console.log("--- Aegis Wallet Integration Demo ---");

            // Initialize the WASM module
            await init();
            console.log("Aegis Crypto Core WASM module initialized.");

            // ... cryptographic operations will go here ...
        }

        runWalletDemo();
    </script>
</body>
</html>
```

### Generating Keypairs

Wallets typically require generating various types of key pairs for different cryptographic purposes (e.g., KEM for secure communication, signatures for transactions). Aegis provides functions for Kyber, Dilithium, and Falcon key generation.

```javascript
// Generate Kyber Key Pair (for KEM/encryption)
const kyberKeyPair = kyber_keygen();
console.log("Kyber Public Key:", kyberKeyPair.public_key);
console.log("Kyber Secret Key:", kyberKeyPair.secret_key);

// Generate Dilithium Key Pair (for digital signatures)
const dilithiumKeyPair = dilithium_keygen();
console.log("Dilithium Public Key:", dilithiumKeyPair.public_key);
console.log("Dilithium Secret Key:", dilithiumKeyPair.secret_key);

// Generate Falcon Key Pair (for digital signatures)
const falconKeyPair = falcon_keygen();
console.log("Falcon Public Key:", falconKeyPair.public_key);
console.log("Falcon Secret Key:", falconKeyPair.secret_key);
```

### Signing Transactions/Messages

Wallets use digital signatures to authorize transactions or sign arbitrary messages. Here's how to use Dilithium and Falcon for signing.

```javascript
const messageToSign = new TextEncoder().encode("0x123abc... (your transaction hash or message)");

// Sign with Dilithium
const dilithiumSignature = dilithium_sign(dilithiumKeyPair.secret_key, messageToSign);
console.log("Dilithium Signature:", dilithiumSignature);

// Sign with Falcon
const falconSignature = falcon_sign(falconKeyPair.secret_key, messageToSign);
console.log("Falcon Signature:", falconSignature);
```

### Verifying Signatures

Verification is crucial for confirming the authenticity of transactions or messages. This process typically happens on-chain (for transactions) or by other parties (for messages).

```javascript
// Verify Dilithium Signature
const isDilithiumValid = dilithium_verify(dilithiumKeyPair.public_key, messageToSign, dilithiumSignature);
console.log("Dilithium Signature Valid:", isDilithiumValid);

// Verify Falcon Signature
const isFalconValid = falcon_verify(falconKeyPair.public_key, messageToSign, falconSignature);
console.log("Falcon Signature Valid:", isFalconValid);
```

## TypeScript Type Definitions

For TypeScript-based wallet development, `wasm-pack` automatically generates `.d.ts` files in the `pkg` directory. These files provide full type information for all WASM-exported functions and data structures, enabling strong type checking and improved developer tooling.

To leverage these types in your TypeScript project, simply import them alongside the JavaScript module:

```typescript
import init, { 
    kyber_keygen, 
    dilithium_keygen, 
    falcon_keygen, 
    dilithium_sign, 
    dilithium_verify, 
    falcon_sign, 
    falcon_verify, 
    KyberKeyPair, 
    DilithiumKeyPair, 
    FalconKeyPair 
} from "./pkg/aegis_crypto_core";

async function runTypedWalletDemo() {
    await init();

    const kyberKeys: KyberKeyPair = kyber_keygen();
    console.log("Kyber Public Key (typed):
", kyberKeys.public_key);

    const message: Uint8Array = new TextEncoder().encode("My secure transaction");
    const dilithiumKeys: DilithiumKeyPair = dilithium_keygen();
    const signature: Uint8Array = dilithium_sign(dilithiumKeys.secret_key, message);
    const isValid: boolean = dilithium_verify(dilithiumKeys.public_key, message, signature);

    console.log("Dilithium Signature Valid (typed):
", isValid);
}

runTypedWalletDemo();
```

## Integration Notes for Wallet Developers

*   **Key Storage:** Post-quantum secret keys are generally larger than traditional ECC keys. Wallets should consider secure and efficient storage mechanisms for these keys, potentially leveraging hardware security modules (HSMs) or secure enclaves where available.
*   **Transaction Formats:** When integrating with existing blockchain protocols, ensure that the post-quantum signatures are correctly formatted and encoded to be compatible with the chain's transaction structure. This might involve custom transaction types or encoding schemes.
*   **User Experience:** Educate users about the benefits of post-quantum security. The increased key and signature sizes might impact transaction fees or network bandwidth, which should be communicated transparently.
*   **Performance Considerations:** While WASM provides near-native performance, cryptographic operations can still be computationally intensive. Optimize your wallet's UI/UX to handle potential delays during key generation or signing, perhaps by showing loading indicators.
*   **Future-Proofing:** The field of post-quantum cryptography is evolving. Design your wallet's architecture to be modular, allowing for easy updates to new algorithms or parameter sets as they become standardized.

## Conclusion

Aegis Crypto Core offers a robust and easy-to-integrate solution for bringing post-quantum security to Web3 and blockchain wallets. By following this guide, developers can build the next generation of secure digital asset management systems, resilient against the threats of quantum computing.

