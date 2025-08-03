//! Integration tests for the Dilithium signature scheme.

use aegis_crypto_core::{dilithium_keygen, dilithium_sign, dilithium_verify};

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
