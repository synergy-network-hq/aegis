//! Native tests for the HQC key encapsulation mechanism (KEM).

use aegis_crypto_core::{hqc_keygen, hqc_encapsulate, hqc_decapsulate};
use aegis_crypto_core::{hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate};
use aegis_crypto_core::{hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate};
use aegis_crypto_core::{hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate};

#[test]
fn test_hqc_encaps_and_decaps() {
    // Add some debug output
    println!("Starting HQC test...");

    // Generate a recipient key pair
    let keypair = hqc_keygen();
    println!("Generated keypair");

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    println!("Public key length: {}, Secret key length: {}", public_key.len(), secret_key.len());

    // Encapsulate a shared secret
    let encapsulated_result = hqc_encapsulate(&public_key);

    match encapsulated_result {
        Ok(encapsulated) => {
            println!("Encapsulation successful");
            let ciphertext = encapsulated.ciphertext();
            let shared_secret_enc = encapsulated.shared_secret();
            println!("Ciphertext length: {}, Shared secret length: {}", ciphertext.len(), shared_secret_enc.len());

            // Decapsulate the shared secret
            let shared_secret_dec_result = hqc_decapsulate(&secret_key, &ciphertext);

            match shared_secret_dec_result {
                Ok(shared_secret_dec) => {
                    println!("Decapsulation successful");
                    assert_eq!(shared_secret_enc, shared_secret_dec, "Shared secrets should match");
                    println!("Shared secrets match - test passed!");
                }
                Err(e) => {
                    panic!("Decapsulation failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            panic!("Encapsulation failed: {:?}", e);
        }
    }
}

#[test]
fn test_hqc128_keygen_encapsulate_decapsulate() {
    let keypair = hqc128_keygen().expect("HQC-128 keygen failed");
    assert_eq!(keypair.security_level(), 128);

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc128_encapsulate(&public_key).expect("HQC-128 encapsulation failed");
    assert_eq!(encapsulated.security_level(), 128);

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc128_decapsulate(&secret_key, &ciphertext).expect("HQC-128 decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_hqc192_keygen_encapsulate_decapsulate() {
    let keypair = hqc192_keygen().expect("HQC-192 keygen failed");
    assert_eq!(keypair.security_level(), 192);

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc192_encapsulate(&public_key).expect("HQC-192 encapsulation failed");
    assert_eq!(encapsulated.security_level(), 192);

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc192_decapsulate(&secret_key, &ciphertext).expect("HQC-192 decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_hqc256_keygen_encapsulate_decapsulate() {
    let keypair = hqc256_keygen().expect("HQC-256 keygen failed");
    assert_eq!(keypair.security_level(), 255);

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc256_encapsulate(&public_key).expect("HQC-256 encapsulation failed");
    assert_eq!(encapsulated.security_level(), 255);

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc256_decapsulate(&secret_key, &ciphertext).expect("HQC-256 decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_hqc_keypair_methods() {
    let keypair128 = hqc128_keygen().expect("HQC-128 keygen failed");
    assert_eq!(keypair128.public_key().len(), keypair128.public_key_length());
    assert_eq!(keypair128.secret_key().len(), keypair128.secret_key_length());
    assert_eq!(keypair128.security_level(), 128);

    let keypair192 = hqc192_keygen().expect("HQC-192 keygen failed");
    assert_eq!(keypair192.public_key().len(), keypair192.public_key_length());
    assert_eq!(keypair192.secret_key().len(), keypair192.secret_key_length());
    assert_eq!(keypair192.security_level(), 192);

    let keypair256 = hqc256_keygen().expect("HQC-256 keygen failed");
    assert_eq!(keypair256.public_key().len(), keypair256.public_key_length());
    assert_eq!(keypair256.secret_key().len(), keypair256.secret_key_length());
    assert_eq!(keypair256.security_level(), 255);
}

#[test]
fn test_hqc_encapsulated_methods() {
    let keypair = hqc128_keygen().expect("HQC-128 keygen failed");
    let public_key = keypair.public_key();

    let encapsulated = hqc128_encapsulate(&public_key).expect("HQC-128 encapsulation failed");
    assert_eq!(encapsulated.ciphertext().len(), encapsulated.ciphertext_length());
    assert_eq!(encapsulated.shared_secret().len(), encapsulated.shared_secret_length());
    assert_eq!(encapsulated.security_level(), 128);
}

#[test]
fn test_hqc_different_security_levels() {
    // Test that different security levels produce different key sizes
    let keypair128 = hqc128_keygen().expect("HQC-128 keygen failed");
    let keypair192 = hqc192_keygen().expect("HQC-192 keygen failed");
    let keypair256 = hqc256_keygen().expect("HQC-256 keygen failed");

    // Keys should have different sizes for different security levels
    assert_ne!(keypair128.public_key().len(), keypair192.public_key().len());
    assert_ne!(keypair128.public_key().len(), keypair256.public_key().len());
    assert_ne!(keypair192.public_key().len(), keypair256.public_key().len());

    assert_ne!(keypair128.secret_key().len(), keypair192.secret_key().len());
    assert_ne!(keypair128.secret_key().len(), keypair256.secret_key().len());
    assert_ne!(keypair192.secret_key().len(), keypair256.secret_key().len());
}

#[test]
fn test_hqc_encapsulation_produces_different_outputs() {
    let keypair = hqc128_keygen().expect("HQC-128 keygen failed");
    let public_key = keypair.public_key();

    // Multiple encapsulations should produce different ciphertexts and shared secrets
    let enc1 = hqc128_encapsulate(&public_key).expect("First encapsulation failed");
    let enc2 = hqc128_encapsulate(&public_key).expect("Second encapsulation failed");

    // Ciphertexts should be different (probabilistic encryption)
    assert_ne!(enc1.ciphertext(), enc2.ciphertext());
    // Shared secrets should also be different
    assert_ne!(enc1.shared_secret(), enc2.shared_secret());
}

#[test]
fn test_hqc_cross_security_level_incompatibility() {
    // Test that keys from one security level cannot be used with functions from another
    let keypair128 = hqc128_keygen().expect("HQC-128 keygen failed");
    let keypair192 = hqc192_keygen().expect("HQC-192 keygen failed");

    let pk128 = keypair128.public_key();
    let sk192 = keypair192.secret_key();

    // Using HQC-128 public key with HQC-192 should fail
    let result = hqc192_encapsulate(&pk128);
    assert!(result.is_err(), "Cross-security-level encapsulation should fail");

    // Create a valid HQC-192 encapsulation
    let pk192 = keypair192.public_key();
    let enc192 = hqc192_encapsulate(&pk192).expect("HQC-192 encapsulation should succeed");
    let ct192 = enc192.ciphertext();

    // Using HQC-192 ciphertext with HQC-128 secret key should fail
    let sk128 = keypair128.secret_key();
    let result = hqc128_decapsulate(&sk128, &ct192);
    assert!(result.is_err(), "Cross-security-level decapsulation should fail");
}
