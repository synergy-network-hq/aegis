//! Native tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{ sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify };

#[test]
fn test_sphincsplus_sign_and_verify() {
    // Generate a key pair
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Sign a message
    let message = b"Hello, SPHINCS+!";
    let signed_message = sphincsplus_sign(&secret_key, message);

    // Verify the signature
    let is_valid = sphincsplus_verify(&public_key, &signed_message);
    assert!(is_valid, "Signature should be valid");

    // Tamper with the signed message
    let mut tampered_signed_message = signed_message.clone();
    tampered_signed_message[0] ^= 0x01;
    let is_valid_tampered = sphincsplus_verify(&public_key, &tampered_signed_message);
    assert!(!is_valid_tampered, "Signature should be invalid for tampered message");
}

#[test]
fn test_sphincsplus_keygen_sign_verify() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+";
    let signed_message = sphincsplus_sign(&secret_key, message);

    let is_valid = sphincsplus_verify(&public_key, &signed_message);
    assert!(is_valid, "Signature should be valid");
}

#[test]
fn test_sphincsplus_keypair_methods() {
    let keypair = sphincsplus_keygen();
    // Just test that the methods exist and return valid data
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}
