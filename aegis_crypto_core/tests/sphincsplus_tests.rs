//! Integration tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

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
