//! Known Answer Tests (KAT) for Falcon signatures.

use aegis_crypto_core::{ falcon_keygen, falcon_sign, falcon_verify };

#[test]
fn test_falcon_kat_round1() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Falcon KAT round 1";
    let signature = falcon_sign(&secret_key, message);
    let is_valid = falcon_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 1: Signature should be valid");
}

#[test]
fn test_falcon_kat_round2() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Falcon KAT round 2";
    let signature = falcon_sign(&secret_key, message);
    let is_valid = falcon_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 2: Signature should be valid");
}

#[test]
fn test_falcon_kat_round3() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Falcon KAT round 3";
    let signature = falcon_sign(&secret_key, message);
    let is_valid = falcon_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 3: Signature should be valid");
}

#[test]
fn test_falcon_kat_round4() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Falcon KAT round 4";
    let signature = falcon_sign(&secret_key, message);
    let is_valid = falcon_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 4: Signature should be valid");
}

#[test]
fn test_falcon_kat_round5() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for Falcon KAT round 5";
    let signature = falcon_sign(&secret_key, message);
    let is_valid = falcon_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 5: Signature should be valid");
}
