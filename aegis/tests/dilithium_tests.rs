//! Integration tests for the Dilithium signature scheme.
//!
//! These tests exercise key generation, signing, and verification to ensure the
//! Dilithium API exposed by Aegis Crypto Core functions correctly.

use aegis_crypto_core::{dilithium_keygen, dilithium_sign, dilithium_verify};

#[test]
fn test_dilithium_sign_and_verify() {
    // Generate a new key pair
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum‑safe signatures are cool!";
    // Sign the message
    let signature = dilithium_sign(&secret_key, message).expect("signing should succeed");
    // Verify the signature
    let is_valid = dilithium_verify(&public_key, message, &signature).expect("verification should succeed");
    assert!(is_valid, "Dilithium signature should be valid");
    // Tamper with the message
    let tampered_message = b"Quantum‑safe signatures are NOT cool!";
    let tampered_valid = dilithium_verify(&public_key, tampered_message, &signature).expect("verification should succeed");
    assert!(!tampered_valid, "Verification should fail for a tampered message");
}