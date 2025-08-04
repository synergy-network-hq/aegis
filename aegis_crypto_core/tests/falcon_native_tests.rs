//! Native tests for the Falcon digital signature scheme.

use aegis_crypto_core::{falcon_keygen, falcon_sign, falcon_verify};
use aegis_crypto_core::falcon::utils::{public_key_length, secret_key_length};

#[test]
fn test_falcon_sign_and_verify() {
    let keypair = falcon_keygen().expect("Keygen should succeed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let message = b"Falcon signatures for quantum safety";

    let signature = falcon_sign(&secret_key, message).expect("Signing should succeed");
    assert!(
        falcon_verify(&public_key, message, &signature),
        "Falcon signature should be valid"
    );

    // Tamper with signature
    let mut bad_sig = signature.clone();
    bad_sig[0] ^= 0xFF;
    assert!(
        !falcon_verify(&public_key, message, &bad_sig),
        "Verification should fail for a tampered signature"
    );
}

#[test]
fn test_falcon_keygen_sign_verify() {
    let keypair = falcon_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    assert_eq!(public_key.len(), public_key_length());
    assert_eq!(secret_key.len(), secret_key_length());

    let message = b"This is a test message for Falcon signing.";
    let signature = falcon_sign(&secret_key, message).expect("Sign failed");

    // Falcon signatures have variable length, so we just check it's not empty
    assert!(!signature.is_empty(), "Signature should not be empty");

    let is_valid = falcon_verify(&public_key, message, &signature);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let is_valid_wrong_message = falcon_verify(&public_key, wrong_message, &signature);
    assert!(!is_valid_wrong_message, "Signature verification succeeded with wrong message");

    // Test with a wrong signature
    let mut wrong_signature = signature.clone();
    if !wrong_signature.is_empty() {
        wrong_signature[0] ^= 0x01; // Flip a bit
        let is_valid_wrong_signature = falcon_verify(&public_key, message, &wrong_signature);
        assert!(!is_valid_wrong_signature, "Signature verification succeeded with wrong signature");
    }
}

#[test]
fn test_invalid_key_lengths() {
    let short_sk = vec![0u8; secret_key_length() - 1];
    let message = b"test message";

    let result = falcon_sign(&short_sk, message);
    assert!(result.is_err(), "Sign should fail with invalid secret key length");

    let short_pk = vec![0u8; public_key_length() - 1];
    let signature = vec![0u8; 100]; // Some dummy signature

    let is_valid = falcon_verify(&short_pk, message, &signature);
    assert!(!is_valid, "Verify should fail with invalid public key length");
}

#[test]
fn test_falcon_keypair_methods() {
    let keypair = falcon_keygen().expect("Keygen failed");
    assert_eq!(keypair.public_key().len(), keypair.public_key_length());
    assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
}