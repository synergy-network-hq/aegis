//! Known Answer Tests (KAT) for SPHINCS+ signatures.

use aegis_crypto_core::{ sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify };

#[test]
fn test_sphincsplus_kat_round1() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+ KAT round 1";
    let signed_message = sphincsplus_sign(&secret_key, message);
    let is_valid = sphincsplus_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 1: Signature should be valid");
}

#[test]
fn test_sphincsplus_kat_round2() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+ KAT round 2";
    let signed_message = sphincsplus_sign(&secret_key, message);
    let is_valid = sphincsplus_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 2: Signature should be valid");
}

#[test]
fn test_sphincsplus_kat_round3() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+ KAT round 3";
    let signed_message = sphincsplus_sign(&secret_key, message);
    let is_valid = sphincsplus_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 3: Signature should be valid");
}

#[test]
fn test_sphincsplus_kat_round4() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+ KAT round 4";
    let signed_message = sphincsplus_sign(&secret_key, message);
    let is_valid = sphincsplus_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 4: Signature should be valid");
}

#[test]
fn test_sphincsplus_kat_round5() {
    let keypair = sphincsplus_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+ KAT round 5";
    let signed_message = sphincsplus_sign(&secret_key, message);
    let is_valid = sphincsplus_verify(&public_key, &signed_message);

    assert!(is_valid, "KAT round 5: Signature should be valid");
}
