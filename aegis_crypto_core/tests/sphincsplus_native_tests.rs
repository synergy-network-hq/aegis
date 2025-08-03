//! Native tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

#[cfg(test)]
use pqcrypto_sphincsplus::sphincssha2128fsimple::{public_key_bytes, secret_key_bytes, signature_bytes};

#[test]
fn test_sphincsplus_sign_and_verify() {
    // Generate a key pair
    let keypair = sphincsplus_keygen().expect("Keygen should succeed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for SPHINCS+.";

    // Sign the message (returns detached signature)
    let signature = sphincsplus_sign(&secret_key, message).expect("Signing should succeed");

    // Verify the signature
    assert!(
        sphincsplus_verify(&public_key, message, &signature),
        "Signature should be valid"
    );

    // Tamper with the signature
    let mut tampered = signature.clone();
    tampered[0] ^= 0xff;
    assert!(
        !sphincsplus_verify(&public_key, message, &tampered),
        "Tampered signature should be invalid"
    );
}

#[test]
fn test_sphincsplus_keygen_sign_verify() {
    let keypair = sphincsplus_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    assert_eq!(public_key.len(), public_key_bytes());
    assert_eq!(secret_key.len(), secret_key_bytes());

    let message = b"This is a test message.";
    let signature = sphincsplus_sign(&secret_key, message).expect("Sign failed");

    assert!(signature.len() <= signature_bytes());

    let is_valid = sphincsplus_verify(&public_key, message, &signature);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let is_valid_wrong_message = sphincsplus_verify(&public_key, wrong_message, &signature);
    assert!(!is_valid_wrong_message, "Signature verification succeeded with wrong message");

    // Test with a wrong signature
    let mut wrong_signature = signature.clone();
    wrong_signature[0] ^= 0x01; // Flip a bit
    let is_valid_wrong_signature = sphincsplus_verify(&public_key, message, &wrong_signature);
    assert!(!is_valid_wrong_signature, "Signature verification succeeded with wrong signature");
}

#[test]
fn test_invalid_key_lengths() {
    let short_sk = vec![0u8; secret_key_bytes() - 1];
    let message = b"test message";

    let result = sphincsplus_sign(&short_sk, message);
    assert!(result.is_err(), "Sign should fail with invalid secret key length");

    let short_pk = vec![0u8; public_key_bytes() - 1];
    let signature = vec![0u8; signature_bytes()];

    let is_valid = sphincsplus_verify(&short_pk, message, &signature);
    assert!(!is_valid, "Verify should fail with invalid public key length");
}

#[test]
fn test_invalid_signature_length() {
    let keypair = sphincsplus_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let message = b"test message";
    let short_signature = vec![0u8; signature_bytes() - 1];

    let is_valid = sphincsplus_verify(&public_key, message, &short_signature);
    assert!(!is_valid, "Verify should fail with invalid signature length");
}

#[test]
fn test_sphincsplus_keypair_methods() {
    let keypair = sphincsplus_keygen().expect("Keygen failed");
    assert_eq!(keypair.public_key().len(), keypair.public_key_length());
    assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
}
