//! Integration tests for the Falcon signature scheme.
//!
//! These tests exercise key generation, signing, and verification to ensure the
//! Falcon API exposed by Aegis Crypto Core functions correctly.

use aegis_crypto_core::{falcon_keygen, falcon_sign, falcon_verify};

#[test]
fn test_falcon_sign_and_verify() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let message = b"Falcon signatures for quantum safety";
    let signature = falcon_sign(&secret_key, message).expect("signing should succeed");
    let is_valid = falcon_verify(&public_key, message, &signature).expect("verification should succeed");
    assert!(is_valid, "Falcon signature should be valid");
    // Tamper with signature
    let mut bad_sig = signature.clone();
    bad_sig[0] ^= 0xFF;
    let bad_valid = falcon_verify(&public_key, message, &bad_sig).expect("verification should succeed");
    assert!(!bad_valid, "Verification should fail for a tampered signature");
}