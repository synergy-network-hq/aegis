//! Native tests for the Falcon digital signature scheme.

use aegis_crypto_core::{ falcon_keygen, falcon_sign, falcon_verify };

#[test]
fn test_falcon_sign_and_verify() {
    // Generate a new key pair
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum safe signatures are cool!";

    // Sign the message (returns detached signature)
    let signature = falcon_sign(&secret_key, message);

    // Verify the signature
    assert!(falcon_verify(&public_key, message, &signature), "Falcon signature should be valid");

    // Tamper with the signature
    let mut tampered = signature.clone();
    tampered[0] ^= 0x55;
    assert!(
        !falcon_verify(&public_key, message, &tampered),
        "Verification should fail for a tampered signature"
    );
}

#[test]
fn test_falcon_keygen_sign_verify() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for Falcon signing.";
    let signature = falcon_sign(&secret_key, message);

    let is_valid = falcon_verify(&public_key, message, &signature);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let is_valid_wrong_message = falcon_verify(&public_key, wrong_message, &signature);
    assert!(!is_valid_wrong_message, "Signature verification should fail with wrong message");

    // Test with a tampered signature
    let mut tampered_signature = signature.clone();
    tampered_signature[0] ^= 0x01; // Flip a bit
    let is_valid_tampered = falcon_verify(&public_key, message, &tampered_signature);
    assert!(!is_valid_tampered, "Signature verification should fail with tampered signature");
}

#[test]
fn test_falcon_keypair_methods() {
    let keypair = falcon_keygen();
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}
