// SPHINCS+ unit tests for Aegis Crypto Core
//
// NOTE: The SPHINCS+ implementation has been disabled due to critical bugs
// in the upstream `pqc_sphincsplus` crate.  The test cases below are
// retained for reference but commented out so they do not compile.  Once a
// functional SPHINCS+ implementation is restored, these tests can be
// uncommented to verify key generation, signing and verification.

/*
use crate::sphincsplus::core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

#[test]
fn test_sphincsplus_keygen_sign_verify() {
    let keypair = sphincsplus_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let message = b"This is a test message.";
    let signature = sphincsplus_sign(&public_key, &secret_key, message).expect("Sign failed");
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
*/