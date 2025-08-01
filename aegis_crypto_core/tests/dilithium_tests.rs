//! Integration tests for the Dilithium signature scheme.

use aegis_crypto_core::{dilithium_keygen, dilithium_sign, dilithium_verify};

#[test]
fn test_dilithium_sign_and_verify() {
    // Generate a new key pair
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum safe signatures are cool!";

    // Sign the message (returns signed message)
    let signed_message = dilithium_sign(&secret_key, message);

    // Verify the signed message
    assert!(
        dilithium_verify(&public_key, &signed_message),
        "Dilithium signature should be valid"
    );

    // Tamper with the signed message
    let mut tampered = signed_message.clone();
    tampered[0] ^= 0x55;
    assert!(
        !dilithium_verify(&public_key, &tampered),
        "Verification should fail for a tampered signed message"
    );
}
