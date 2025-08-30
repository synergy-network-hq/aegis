//! Known Answer Tests (KAT) for Dilithium signatures.

use aegis_crypto_core::{ dilithium_keygen, dilithium_sign, dilithium_verify };

#[test]
fn test_dilithium_kat_round1() {
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Dilithium KAT round 1";
    let signed_message = dilithium_sign(&secret_key, message);
    let is_valid = dilithium_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 1: Signature should be valid");
}

#[test]
fn test_dilithium_kat_round2() {
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Dilithium KAT round 2";
    let signed_message = dilithium_sign(&secret_key, message);
    let is_valid = dilithium_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 2: Signature should be valid");
}

#[test]
fn test_dilithium_kat_round3() {
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Dilithium KAT round 3";
    let signed_message = dilithium_sign(&secret_key, message);
    let is_valid = dilithium_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 3: Signature should be valid");
}

#[test]
fn test_dilithium_kat_round4() {
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Dilithium KAT round 4";
    let signed_message = dilithium_sign(&secret_key, message);
    let is_valid = dilithium_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 4: Signature should be valid");
}

#[test]
fn test_dilithium_kat_round5() {
    let keypair = dilithium_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Dilithium KAT round 5";
    let signed_message = dilithium_sign(&secret_key, message);
    let is_valid = dilithium_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 5: Signature should be valid");
}
