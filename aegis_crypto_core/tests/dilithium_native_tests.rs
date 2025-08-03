//! Native tests for the Dilithium digital signature scheme.

use aegis_crypto_core::{dilithium_keygen, dilithium_sign, dilithium_verify};
use aegis_crypto_core::dilithium::{dilithium_keygen_native, dilithium_sign_native, dilithium_verify_native};
use aegis_crypto_core::dilithium::utils::{public_key_length, secret_key_length, signature_length};

#[test]
fn test_dilithium_sign_and_verify() {
    // Generate a new key pair
    let keypair = dilithium_keygen().expect("Keygen should succeed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum safe signatures are cool!";

    // Sign the message (returns detached signature)
    let signature = dilithium_sign(&secret_key, message).expect("Signing should succeed");

    // Verify the signature
    assert!(
        dilithium_verify(&public_key, message, &signature),
        "Dilithium signature should be valid"
    );

    // Tamper with the signature
    let mut tampered = signature.clone();
    tampered[0] ^= 0x55;
    assert!(
        !dilithium_verify(&public_key, message, &tampered),
        "Verification should fail for a tampered signature"
    );
}

#[test]
fn test_dilithium_keygen_sign_verify() {
    let keypair = dilithium_keygen_native().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    assert_eq!(public_key.len(), public_key_length());
    assert_eq!(secret_key.len(), secret_key_length());

    let message = b"This is a test message for Dilithium signing.";
    let signature = dilithium_sign_native(&secret_key, message).expect("Sign failed");

    assert_eq!(signature.len(), signature_length());

    let is_valid = dilithium_verify_native(&public_key, message, &signature);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let is_valid_wrong_message = dilithium_verify_native(&public_key, wrong_message, &signature);
    assert!(!is_valid_wrong_message, "Signature verification succeeded with wrong message");

    // Test with a wrong signature
    let mut wrong_signature = signature.clone();
    wrong_signature[0] ^= 0x01; // Flip a bit
    let is_valid_wrong_signature = dilithium_verify_native(&public_key, message, &wrong_signature);
    assert!(!is_valid_wrong_signature, "Signature verification succeeded with wrong signature");
}

#[test]
fn test_invalid_key_lengths() {
    let short_sk = vec![0u8; secret_key_length() - 1];
    let message = b"test message";

    let result = dilithium_sign_native(&short_sk, message);
    assert!(result.is_err(), "Sign should fail with invalid secret key length");

    let short_pk = vec![0u8; public_key_length() - 1];
    let signature = vec![0u8; signature_length()];

    let is_valid = dilithium_verify_native(&short_pk, message, &signature);
    assert!(!is_valid, "Verify should fail with invalid public key length");
}

#[test]
fn test_invalid_signature_length() {
    let keypair = dilithium_keygen_native().expect("Keygen failed");
    let public_key = keypair.public_key();
    let message = b"test message";
    let short_signature = vec![0u8; signature_length() - 1];

    let is_valid = dilithium_verify_native(&public_key, message, &short_signature);
    assert!(!is_valid, "Verify should fail with invalid signature length");
}

#[test]
fn test_dilithium_keypair_methods() {
    let keypair = dilithium_keygen_native().expect("Keygen failed");
    assert_eq!(keypair.public_key().len(), keypair.public_key_length());
    assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
}
