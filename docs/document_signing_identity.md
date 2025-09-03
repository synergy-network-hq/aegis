# Document Signing and Identity with Aegis Crypto Core

This document provides a comprehensive guide to leveraging Aegis Crypto Core for secure document signing and identity verification using post-quantum cryptographic algorithms. Digital signatures are fundamental for ensuring data integrity, authenticity, and non-repudiation, and with the advent of quantum computing, it is crucial to transition to quantum-resistant solutions. **Note: The SPHINCS+ implementation is currently non-functional due to critical upstream library bugs and cannot be used for document signing or identity verification.**

## Introduction to Digital Signatures

Digital signatures are cryptographic mechanisms used to verify the authenticity and integrity of digital documents or messages. They provide assurances similar to handwritten signatures in the physical world, but with a higher level of security and verifiability. A valid digital signature confirms that the document originated from the stated signer and has not been altered since it was signed.

Key properties of digital signatures include:

*   **Authenticity:** The recipient can confirm the identity of the signer.
*   **Integrity:** The recipient can verify that the document has not been tampered with since it was signed.
*   **Non-repudiation:** The signer cannot credibly deny having signed the document.

Traditional digital signatures rely on algorithms like RSA and ECDSA. However, these algorithms are vulnerable to attacks by sufficiently powerful quantum computers. Post-quantum cryptography (PQC) offers new algorithms designed to resist such attacks, ensuring the long-term security of digital signatures.

## Aegis Crypto Core for Document Signing

Aegis Crypto Core provides implementations of NIST-standardized post-quantum digital signature algorithms: Dilithium, Falcon, and SPHINCS+. These algorithms offer varying trade-offs in terms of key sizes, signature sizes, and performance, allowing developers to choose the most suitable option for their specific use case.

### Key Algorithms and Their Characteristics

| Algorithm   | Public Key Size | Secret Key Size | Signature Size | Security Level (NIST) | Use Case                                     |
|-------------|-----------------|-----------------|----------------|-----------------------|----------------------------------------------|
| **Dilithium** | ~1.9 KB         | ~4.0 KB         | ~3.3 KB        | Level 3               | High-throughput, strong security             |
| **Falcon**    | ~0.9 KB         | ~1.3 KB         | ~0.7 KB        | Level 1               | Balanced size/performance, general purpose   |
| **SPHINCS+**  | ~32 bytes       | ~64 bytes       | ~7.8 KB        | Level 1               | Smallest keys, largest signatures, hash-based|

*(Note: Sizes are approximate and can vary slightly based on specific parameters and implementations.)*

## WASM/JS API for Document Signing and Verification

Aegis Crypto Core is compiled to WebAssembly (WASM), enabling its use directly in web browsers and Node.js environments. This allows for client-side key generation, signing, and verification, enhancing user privacy and reducing reliance on centralized services.

### Initialization

Before using any cryptographic functions, the WASM module must be initialized. This typically involves loading the `.wasm` file.

```javascript
import { initializeAegis } from './document_signing_api.js';

async function initAegis() {
    // For Node.js, provide the path to the WASM file
    // For browser, if WASM is served from same origin, no path needed
    await initializeAegis(); 
    console.log("Aegis Crypto Core WASM module initialized.");
}

initAegis();
```

### Key Pair Generation

Each algorithm provides a function to generate a new public/secret key pair. It is crucial to securely store the `secretKey` as its compromise would allow unauthorized signing.

#### Dilithium Key Generation

```javascript
import { generateDilithiumKeyPair } from './document_signing_api.js';

const dilithiumKeyPair = generateDilithiumKeyPair();
console.log("Dilithium Public Key:", dilithiumKeyPair.publicKey);
console.log("Dilithium Secret Key:", dilithiumKeyPair.secretKey);
```

#### Falcon Key Generation

```javascript
import { generateFalconKeyPair } from './document_signing_api.js';

const falconKeyPair = generateFalconKeyPair();
console.log("Falcon Public Key:", falconKeyPair.publicKey);
console.log("Falcon Secret Key:", falconKeyPair.secretKey);
```

#### SPHINCS+ Key Generation

```javascript
import { generateSphincsPlusKeyPair } from './document_signing_api.js';

const sphincsplusKeyPair = generateSphincsPlusKeyPair();
console.log("SPHINCS+ Public Key:", sphincsplusKeyPair.publicKey);
console.log("SPHINCS+ Secret Key:", sphincsplusKeyPair.secretKey);
```

### Document Signing

To sign a document, you will need the `secretKey` and the document content (as `Uint8Array`). It is common practice to hash the document content first and then sign the hash, rather than signing the entire document directly. This example signs the raw document content for simplicity.

```javascript
import { signWithDilithium, signWithFalcon, signWithSphincsPlus } from './document_signing_api.js';

const documentContent = new TextEncoder().encode("This is the content of my important document.");

// Sign with Dilithium
const dilithiumSignature = signWithDilithium(dilithiumKeyPair.secretKey, documentContent);
console.log("Dilithium Signature:", dilithiumSignature);

// Sign with Falcon
const falconSignature = signWithFalcon(falconKeyPair.secretKey, documentContent);
console.log("Falcon Signature:", falconSignature);

// Sign with SPHINCS+
const sphincsplusSignature = signWithSphincsPlus(sphincsplusKeyPair.secretKey, documentContent);
console.log("SPHINCS+ Signature:", sphincsplusSignature);
```

### Signature Verification

To verify a signature, you need the `publicKey`, the original `documentContent`, and the `signature` itself. The verification function will return `true` if the signature is valid and `false` otherwise.

```javascript
import { verifyWithDilithium, verifyWithFalcon, verifyWithSphincsPlus } from './document_signing_api.js';

const documentContent = new TextEncoder().encode("This is the content of my important document.");

// Verify with Dilithium
const isDilithiumValid = verifyWithDilithium(dilithiumKeyPair.publicKey, documentContent, dilithiumSignature);
console.log("Dilithium Signature Valid:", isDilithiumValid);

// Verify with Falcon
const isFalconValid = verifyWithFalcon(falconKeyPair.publicKey, documentContent, falconSignature);
console.log("Falcon Signature Valid:", isFalconValid);

// Verify with SPHINCS+
const isSphincsPlusValid = verifyWithSphincsPlus(sphincsplusKeyPair.publicKey, documentContent, sphincsplusSignature);
console.log("SPHINCS+ Signature Valid:", isSphincsPlusValid);
```

## Key Management and Signature Format Notes

Effective key management is paramount for the security of any cryptographic system. For post-quantum cryptography, additional considerations arise due to the larger key and signature sizes, and the unique properties of lattice-based and hash-based schemes.

### Key Storage and Protection

*   **Hardware Security Modules (HSMs):** For high-value assets or critical identities, consider storing private keys in HSMs. These devices provide a secure, tamper-resistant environment for cryptographic operations.
*   **Secure Enclaves/Trusted Execution Environments:** On modern processors, secure enclaves (e.g., Intel SGX, ARM TrustZone) can offer a protected environment for key material and signing operations.
*   **Encryption at Rest:** Always encrypt stored private keys using strong symmetric encryption algorithms (e.g., AES-256) with a robust key derivation function (KDF) if derived from a passphrase.
*   **Access Control:** Implement strict access controls (e.g., multi-factor authentication, role-based access control) to prevent unauthorized access to key material.
*   **Key Backup and Recovery:** Establish secure, redundant backup and recovery procedures for private keys. Loss of a private key means loss of the ability to sign, and potentially loss of access to signed data.

### Key Rotation

Regular key rotation is a best practice that limits the impact of a key compromise and mitigates risks from evolving cryptanalytic techniques. For stateful signature schemes (like some variants of SPHINCS+), key rotation is inherently part of their security model.

*   **Scheduled Rotation:** Implement a policy for periodic key rotation (e.g., annually, quarterly).
*   **Usage-Based Rotation:** For high-volume signing, consider rotating keys after a certain number of signatures.
*   **Secure Decommissioning:** When a key is retired, ensure it is securely erased from all storage locations.

### Signature Format and Size Considerations

As noted in the table above, post-quantum signatures and public keys are significantly larger than their classical counterparts. This has implications for storage, transmission, and processing.

*   **Dilithium:** Offers a good balance of security and performance, but its signatures are substantial. This might impact network bandwidth and storage requirements for applications that generate many signatures.
*   **Falcon:** Provides more compact signatures, making it a strong candidate for environments with bandwidth constraints, such as mobile devices or IoT applications.
*   **SPHINCS+:** While having very small public keys, its signatures are the largest among the three. This is a trade-off for its hash-based security, which relies on minimal cryptographic assumptions. SPHINCS+ is often considered for long-term archival and situations where key management simplicity (due to small public keys) is highly valued, despite the large signature size.

When designing systems, consider:

*   **Data Transmission:** Optimize protocols to handle larger data payloads. Compression techniques might be beneficial.
*   **Storage:** Plan for increased storage needs for signed documents and associated signatures.
*   **Blockchain Integration:** If signatures are to be verified on-chain, the gas costs associated with larger data inputs will be significantly higher. This often necessitates off-chain verification with on-chain proofs or attestations.

## Conclusion

Aegis Crypto Core provides robust tools for implementing post-quantum digital signatures for document signing and identity verification. By understanding the characteristics of Dilithium, Falcon, and SPHINCS+, and by adhering to best practices for key management and signature handling, developers can build secure, future-proof applications capable of resisting quantum attacks. The provided WASM/JS API simplifies integration into both web and Node.js environments, enabling a smooth transition to the post-quantum era.

