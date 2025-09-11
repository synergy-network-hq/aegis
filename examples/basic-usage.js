#!/usr/bin/env node

/**
 * AEGIS Basic Usage Examples
 * Demonstrates basic usage of all 5 post-quantum cryptographic algorithms
 */

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

async function main() {
  console.log('🚀 AEGIS Post-Quantum Cryptography Examples');
  console.log('==========================================\n');

  // Initialize WASM module
  console.log('Initializing WASM module...');
  await init();
  console.log('✅ WASM module initialized\n');

  // Example 1: ML-KEM Key Exchange
  console.log('1. ML-KEM-768 Key Exchange Example');
  console.log('----------------------------------');
  await mlkemExample();
  console.log('');

  // Example 2: ML-DSA Digital Signature
  console.log('2. ML-DSA-65 Digital Signature Example');
  console.log('--------------------------------------');
  await mldsaExample();
  console.log('');

  // Example 3: Falcon Compact Signature
  console.log('3. Falcon-512 Compact Signature Example');
  console.log('---------------------------------------');
  await falconExample();
  console.log('');

  // Example 4: SPHINCS+ Hash-based Signature
  console.log('4. SPHINCS+-SHA2-128f Hash-based Signature Example');
  console.log('--------------------------------------------------');
  await sphincsExample();
  console.log('');

  // Example 5: HQC-KEM Key Encapsulation
  console.log('5. HQC-128 Key Encapsulation Example');
  console.log('------------------------------------');
  await hqcExample();
  console.log('');

  console.log('🎉 All examples completed successfully!');
}

async function mlkemExample() {
  try {
    const message = "Hello, quantum world!";
    console.log(`Message: "${message}"`);

    // Generate key pair
    console.log('Generating ML-KEM-768 key pair...');
    const { publicKey, secretKey } = mlkem768_keygen();
    console.log(`✅ Public key size: ${publicKey.length} bytes`);
    console.log(`✅ Secret key size: ${secretKey.length} bytes`);

    // Encapsulate shared secret
    console.log('Encapsulating shared secret...');
    const { ciphertext, sharedSecret } = mlkem768_encaps(publicKey);
    console.log(`✅ Ciphertext size: ${ciphertext.length} bytes`);
    console.log(`✅ Shared secret size: ${sharedSecret.length} bytes`);

    // Decapsulate shared secret
    console.log('Decapsulating shared secret...');
    const decryptedSecret = mlkem768_decaps(ciphertext, secretKey);
    console.log(`✅ Decrypted secret size: ${decryptedSecret.length} bytes`);

    // Verify shared secrets match
    const secretsMatch = Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret));
    console.log(`✅ Shared secrets match: ${secretsMatch}`);

    if (secretsMatch) {
      console.log('🎯 ML-KEM key exchange successful!');
    } else {
      console.log('❌ ML-KEM key exchange failed!');
    }

  } catch (error) {
    console.error('❌ ML-KEM example failed:', error.message);
  }
}

async function mldsaExample() {
  try {
    const message = new TextEncoder().encode("Sign this message with ML-DSA");
    console.log(`Message: "${new TextDecoder().decode(message)}"`);

    // Generate key pair
    console.log('Generating ML-DSA-65 key pair...');
    const { publicKey, secretKey } = mldsa65_keygen();
    console.log(`✅ Public key size: ${publicKey.length} bytes`);
    console.log(`✅ Secret key size: ${secretKey.length} bytes`);

    // Sign the message
    console.log('Signing message...');
    const signature = mldsa65_sign(secretKey, message);
    console.log(`✅ Signature size: ${signature.length} bytes`);

    // Verify the signature
    console.log('Verifying signature...');
    const isValid = mldsa65_verify(publicKey, message, signature);
    console.log(`✅ Signature valid: ${isValid}`);

    if (isValid) {
      console.log('🎯 ML-DSA signature verification successful!');
    } else {
      console.log('❌ ML-DSA signature verification failed!');
    }

  } catch (error) {
    console.error('❌ ML-DSA example failed:', error.message);
  }
}

async function falconExample() {
  try {
    const message = new TextEncoder().encode("Compact quantum signature");
    console.log(`Message: "${new TextDecoder().decode(message)}"`);

    // Generate key pair
    console.log('Generating Falcon-512 key pair...');
    const { publicKey, secretKey } = falcon512_keygen();
    console.log(`✅ Public key size: ${publicKey.length} bytes`);
    console.log(`✅ Secret key size: ${secretKey.length} bytes`);

    // Sign the message
    console.log('Signing message...');
    const signature = falcon512_sign(secretKey, message);
    console.log(`✅ Signature size: ${signature.length} bytes`);

    // Verify the signature
    console.log('Verifying signature...');
    const isValid = falcon512_verify(publicKey, message, signature);
    console.log(`✅ Signature valid: ${isValid}`);

    if (isValid) {
      console.log('🎯 Falcon signature verification successful!');
      console.log(`📊 Falcon provides compact signatures: ${signature.length} bytes`);
    } else {
      console.log('❌ Falcon signature verification failed!');
    }

  } catch (error) {
    console.error('❌ Falcon example failed:', error.message);
  }
}

async function sphincsExample() {
  try {
    const message = new TextEncoder().encode("Long-term quantum security");
    console.log(`Message: "${new TextDecoder().decode(message)}"`);

    // Generate key pair
    console.log('Generating SPHINCS+-SHA2-128f key pair...');
    const { publicKey, secretKey } = slhdsa_sha2_128f_keygen();
    console.log(`✅ Public key size: ${publicKey.length} bytes`);
    console.log(`✅ Secret key size: ${secretKey.length} bytes`);

    // Sign the message
    console.log('Signing message...');
    const signature = slhdsa_sha2_128f_sign(secretKey, message);
    console.log(`✅ Signature size: ${signature.length} bytes`);

    // Verify the signature
    console.log('Verifying signature...');
    const isValid = slhdsa_sha2_128f_verify(publicKey, message, signature);
    console.log(`✅ Signature valid: ${isValid}`);

    if (isValid) {
      console.log('🎯 SPHINCS+ signature verification successful!');
      console.log('📊 SPHINCS+ provides long-term quantum security');
    } else {
      console.log('❌ SPHINCS+ signature verification failed!');
    }

  } catch (error) {
    console.error('❌ SPHINCS+ example failed:', error.message);
  }
}

async function hqcExample() {
  try {
    const message = "Lightweight quantum cryptography";
    console.log(`Message: "${message}"`);

    // Generate key pair
    console.log('Generating HQC-128 key pair...');
    const { publicKey, secretKey } = hqc128_keygen();
    console.log(`✅ Public key size: ${publicKey.length} bytes`);
    console.log(`✅ Secret key size: ${secretKey.length} bytes`);

    // Encapsulate shared secret
    console.log('Encapsulating shared secret...');
    const { ciphertext, sharedSecret } = hqc128_encaps(publicKey);
    console.log(`✅ Ciphertext size: ${ciphertext.length} bytes`);
    console.log(`✅ Shared secret size: ${sharedSecret.length} bytes`);

    // Decapsulate shared secret
    console.log('Decapsulating shared secret...');
    const decryptedSecret = hqc128_decaps(ciphertext, secretKey);
    console.log(`✅ Decrypted secret size: ${decryptedSecret.length} bytes`);

    // Verify shared secrets match
    const secretsMatch = Buffer.from(sharedSecret).equals(Buffer.from(decryptedSecret));
    console.log(`✅ Shared secrets match: ${secretsMatch}`);

    if (secretsMatch) {
      console.log('🎯 HQC key exchange successful!');
      console.log('📊 HQC provides lightweight quantum-resistant cryptography');
    } else {
      console.log('❌ HQC key exchange failed!');
    }

  } catch (error) {
    console.error('❌ HQC example failed:', error.message);
  }
}

// Run the examples
main().catch(console.error);
