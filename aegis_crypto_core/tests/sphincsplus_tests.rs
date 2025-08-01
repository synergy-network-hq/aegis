//! Integration tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

#[test]
fn test_sphincsplus_sign_and_verify() {
    // Generate a key pair
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for SPHINCS+.";

    // Sign the message (returns a signed message, not just signature)
    let signed_message = sphincsplus_sign(&secret_key, message);

    // Verify the signed message
    assert!(
        sphincsplus_verify(&public_key, &signed_message),
        "Signature should be valid"
    );

    // Tamper with the signed message
    let mut tampered = signed_message.clone();
    tampered[0] ^= 0xff;
    assert!(
        !sphincsplus_verify(&public_key, &tampered),
        "Tampered signed message should be invalid"
    );
}
