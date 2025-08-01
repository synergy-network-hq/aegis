//! Integration tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

#[test]
fn test_sphincsplus_sign_and_verify() {
    // Generate a key pair
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for SPHINCS+.";

    // Sign the message
    let signature = sphincsplus_sign(&secret_key, message);

    // Verify the signature
    assert!(sphincsplus_verify(&public_key, message, &signature), "Signature should be valid");

    // Test with a wrong message
    let wrong_message = b"This is a different message.";
    assert!(!sphincsplus_verify(&public_key, wrong_message, &signature), "Signature should be invalid for a different message");

    // Tamper with the signature
    let mut tampered_signature = signature.clone();
    tampered_signature[0] ^= 0xff;
    assert!(!sphincsplus_verify(&public_key, message, &tampered_signature), "Tampered signature should be invalid");
}
