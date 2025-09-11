# AEGIS WASM Usage Guide

## Overview

AEGIS provides comprehensive post-quantum cryptography support through WebAssembly (WASM) implementations. This guide covers all available algorithms and their usage patterns.

## Available Algorithms

### 1. ML-KEM (Module-Lattice Key Encapsulation Mechanism) - FIPS 203
**Formerly known as Kyber**

- **Security Levels**: 512, 768, 1024
- **Implementations**: Reference, Optimized, AVX2, Clean
- **Use Cases**: Key exchange, hybrid encryption

### 2. ML-DSA (Module-Lattice Digital Signature Algorithm) - FIPS 204
**Formerly known as Dilithium**

- **Security Levels**: 44, 65, 87 (Level 2, 3, 5)
- **Implementations**: Reference, AVX2
- **Use Cases**: Digital signatures, authentication

### 3. Falcon (Fast-Fourier Lattice-based Compact Signatures over NTRU) - FIPS 206
**Also known as FN-DSA**

- **Security Levels**: 512, 1024
- **Implementations**: Reference, Optimized, Extra
- **Use Cases**: Compact digital signatures

### 4. SPHINCS+ (Stateless Hash-based Signatures) - FIPS 205
**Also known as SLH-DSA**

- **Security Levels**: 128f, 128s, 192f, 192s, 256f, 256s
- **Implementations**: Reference, Optimized, Haraka-AESNI, SHA256-AVX2, SHAKE256-AVX2
- **Use Cases**: Long-term security, backup signatures

### 5. HQC-KEM (Hamming Quasi-Cyclic Key Encapsulation Mechanism) - FIPS 207

- **Security Levels**: 128, 192, 256
- **Implementations**: Reference, Optimized, Additional
- **Use Cases**: Key exchange, lightweight cryptography

## Installation

```bash
npm install aegis-crypto-core
```

## Basic Usage

### Importing the Library

```javascript
import init, {
  // ML-KEM functions
  mlkem512_keygen, mlkem512_encaps, mlkem512_decaps,
  mlkem768_keygen, mlkem768_encaps, mlkem768_decaps,
  mlkem1024_keygen, mlkem1024_encaps, mlkem1024_decaps,

  // ML-DSA functions
  mldsa44_keygen, mldsa44_sign, mldsa44_verify,
  mldsa65_keygen, mldsa65_sign, mldsa65_verify,
  mldsa87_keygen, mldsa87_sign, mldsa87_verify,

  // Falcon functions
  falcon512_keygen, falcon512_sign, falcon512_verify,
  falcon1024_keygen, falcon1024_sign, falcon1024_verify,

  // SPHINCS+ functions
  slhdsa_sha2_128f_keygen, slhdsa_sha2_128f_sign, slhdsa_sha2_128f_verify,
  slhdsa_sha2_192f_keygen, slhdsa_sha2_192f_sign, slhdsa_sha2_192f_verify,
  slhdsa_sha2_256f_keygen, slhdsa_sha2_256f_sign, slhdsa_sha2_256f_verify,

  // HQC-KEM functions
  hqc128_keygen, hqc128_encaps, hqc128_decaps,
  hqc192_keygen, hqc192_encaps, hqc192_decaps,
  hqc256_keygen, hqc256_encaps, hqc256_decaps
} from 'aegis-crypto-core';

// Initialize the WASM module
await init();
```

## Algorithm-Specific Examples

### ML-KEM (Key Encapsulation)

```javascript
// ML-KEM-768 Key Exchange
async function mlkemKeyExchange() {
  // Generate key pair
  const { publicKey, secretKey } = mlkem768_keygen();

  // Encapsulate (encrypt) a shared secret
  const { ciphertext, sharedSecret } = mlkem768_encaps(publicKey);

  // Decapsulate (decrypt) the shared secret
  const decryptedSecret = mlkem768_decaps(ciphertext, secretKey);

  // Verify the shared secrets match
  console.log('Shared secrets match:',
    Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret)));

  return { publicKey, secretKey, ciphertext, sharedSecret };
}
```

### ML-DSA (Digital Signatures)

```javascript
// ML-DSA-65 Digital Signature
async function mldsaSignature() {
  const message = new TextEncoder().encode("Hello, quantum world!");

  // Generate key pair
  const { publicKey, secretKey } = mldsa65_keygen();

  // Sign the message
  const signature = mldsa65_sign(secretKey, message);

  // Verify the signature
  const isValid = mldsa65_verify(publicKey, message, signature);

  console.log('Signature valid:', isValid);

  return { publicKey, secretKey, signature, isValid };
}
```

### Falcon (Compact Signatures)

```javascript
// Falcon-512 Compact Signature
async function falconSignature() {
  const message = new TextEncoder().encode("Compact quantum signature");

  // Generate key pair
  const { publicKey, secretKey } = falcon512_keygen();

  // Sign the message
  const signature = falcon512_sign(secretKey, message);

  // Verify the signature
  const isValid = falcon512_verify(publicKey, message, signature);

  console.log('Falcon signature valid:', isValid);
  console.log('Signature size:', signature.length, 'bytes');

  return { publicKey, secretKey, signature, isValid };
}
```

### SPHINCS+ (Hash-based Signatures)

```javascript
// SPHINCS+-SHA2-128f Hash-based Signature
async function sphincsSignature() {
  const message = new TextEncoder().encode("Long-term quantum security");

  // Generate key pair
  const { publicKey, secretKey } = slhdsa_sha2_128f_keygen();

  // Sign the message
  const signature = slhdsa_sha2_128f_sign(secretKey, message);

  // Verify the signature
  const isValid = slhdsa_sha2_128f_verify(publicKey, message, signature);

  console.log('SPHINCS+ signature valid:', isValid);
  console.log('Signature size:', signature.length, 'bytes');

  return { publicKey, secretKey, signature, isValid };
}
```

### HQC-KEM (Hamming Quasi-Cyclic)

```javascript
// HQC-128 Key Encapsulation
async function hqcKeyExchange() {
  // Generate key pair
  const { publicKey, secretKey } = hqc128_keygen();

  // Encapsulate shared secret
  const { ciphertext, sharedSecret } = hqc128_encaps(publicKey);

  // Decapsulate shared secret
  const decryptedSecret = hqc128_decaps(ciphertext, secretKey);

  // Verify shared secrets match
  const isValid = Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret));

  console.log('HQC shared secrets match:', isValid);

  return { publicKey, secretKey, ciphertext, sharedSecret };
}
```

## Advanced Usage Patterns

### Hybrid Encryption (ML-KEM + AES)

```javascript
import { createCipher, createDecipher } from 'crypto';

async function hybridEncryption(plaintext) {
  // Generate ML-KEM key pair
  const { publicKey, secretKey } = mlkem768_keygen();

  // Encapsulate shared secret
  const { ciphertext: kemCiphertext, sharedSecret } = mlkem768_encaps(publicKey);

  // Use shared secret for AES encryption
  const cipher = createCipher('aes-256-gcm', sharedSecret);
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  const authTag = cipher.getAuthTag();

  return {
    kemCiphertext,
    encrypted,
    authTag,
    secretKey // For decryption
  };
}

async function hybridDecryption(encryptedData, secretKey) {
  // Decapsulate shared secret
  const sharedSecret = mlkem768_decaps(encryptedData.kemCiphertext, secretKey);

  // Decrypt with AES
  const decipher = createDecipher('aes-256-gcm', sharedSecret);
  decipher.setAuthTag(encryptedData.authTag);
  let decrypted = decipher.update(encryptedData.encrypted, 'hex', 'utf8');
  decrypted += decipher.final('utf8');

  return decrypted;
}
```

### Multi-Algorithm Signature Scheme

```javascript
async function multiAlgorithmSignature(message) {
  const results = {};

  // ML-DSA signature
  const mldsaKeys = mldsa65_keygen();
  results.mldsa = {
    signature: mldsa65_sign(mldsaKeys.secretKey, message),
    publicKey: mldsaKeys.publicKey,
    valid: mldsa65_verify(mldsaKeys.publicKey, message,
      mldsa65_sign(mldsaKeys.secretKey, message))
  };

  // Falcon signature
  const falconKeys = falcon512_keygen();
  results.falcon = {
    signature: falcon512_sign(falconKeys.secretKey, message),
    publicKey: falconKeys.publicKey,
    valid: falcon512_verify(falconKeys.publicKey, message,
      falcon512_sign(falconKeys.secretKey, message))
  };

  // SPHINCS+ signature
  const sphincsKeys = slhdsa_sha2_128f_keygen();
  results.sphincs = {
    signature: slhdsa_sha2_128f_sign(sphincsKeys.secretKey, message),
    publicKey: sphincsKeys.publicKey,
    valid: slhdsa_sha2_128f_verify(sphincsKeys.publicKey, message,
      slhdsa_sha2_128f_sign(sphincsKeys.secretKey, message))
  };

  return results;
}
```

## Performance Considerations

### Algorithm Selection Guide

| Use Case | Recommended Algorithm | Reason |
|----------|----------------------|---------|
| High-performance key exchange | ML-KEM-768 | Balanced security/performance |
| Compact signatures | Falcon-512 | Smallest signature size |
| Long-term security | SPHINCS+-SHA2-256f | Hash-based, quantum-resistant |
| Lightweight applications | HQC-128 | Smallest key sizes |
| High security | ML-DSA-87 | Highest security level |

### Implementation Variants

- **Reference**: Most compatible, slower
- **Optimized**: Better performance, good compatibility
- **AVX2**: Best performance on modern CPUs
- **Clean**: Minimal dependencies, good for embedded

## Error Handling

```javascript
async function safeKeyGeneration() {
  try {
    const { publicKey, secretKey } = mlkem768_keygen();
    return { success: true, publicKey, secretKey };
  } catch (error) {
    console.error('Key generation failed:', error);
    return { success: false, error: error.message };
  }
}
```

## Browser vs Node.js

### Browser Usage
```html
<script type="module">
  import init, { mlkem768_keygen } from 'aegis-crypto-core';

  await init();
  const keys = mlkem768_keygen();
</script>
```

### Node.js Usage
```javascript
import init, { mlkem768_keygen } from 'aegis-crypto-core';

await init();
const keys = mlkem768_keygen();
```

## Security Best Practices

1. **Always verify signatures** before trusting data
2. **Use appropriate security levels** for your threat model
3. **Store private keys securely** (use hardware security modules when possible)
4. **Regularly rotate keys** according to your security policy
5. **Use hybrid schemes** combining classical and post-quantum cryptography during transition

## Troubleshooting

### Common Issues

1. **WASM not loading**: Ensure you call `await init()` before using functions
2. **Memory errors**: Large operations may require more memory allocation
3. **Performance issues**: Try different implementation variants (AVX2 for better performance)

### Getting Help

- Check the [GitHub repository](https://github.com/synergy-network-hq/aegis) for issues
- Review the [API documentation](https://docs.rs/aegis_crypto_core)
- Join our community discussions

---

**Note**: This guide covers the WASM interface. For Rust usage, see the main documentation.
