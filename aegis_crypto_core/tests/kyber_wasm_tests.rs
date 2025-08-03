//! Integration tests for the Kyber key encapsulation mechanism (KEM).
//!
//! These tests ensure that encapsulation and decapsulation produce matching
//! shared secrets and that incorrect inputs are handled correctly.

use aegis_crypto_core::{kyber_keygen, kyber_encapsulate, kyber_decapsulate};

#[test]
fn test_kyber_encaps_and_decaps() {
    // Generate a recipient key pair
    let keypair = kyber_keygen().expect("Keygen should succeed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Encapsulate a shared secret
    let encapsulated = kyber_encapsulate(&public_key).expect("encapsulation should succeed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret_enc = encapsulated.shared_secret();

    // Decapsulate the shared secret
    let shared_secret_dec = kyber_decapsulate(&secret_key, &ciphertext).expect("decapsulation should succeed");

    assert_eq!(shared_secret_enc, shared_secret_dec, "Shared secrets should match");

    // Tamper with ciphertext
    let mut tampered_ct = ciphertext.clone();
    tampered_ct[0] ^= 0x01;

    // Decapsulation with tampered ciphertext should still succeed but produce a different secret
    let tampered_secret_res = kyber_decapsulate(&secret_key, &tampered_ct);
    assert!(tampered_secret_res.is_ok(), "Decapsulation of tampered ciphertext should not fail");
    let tampered_secret = tampered_secret_res.unwrap();
    assert_ne!(shared_secret_enc, tampered_secret, "Decapsulation of tampered ciphertext should produce a different secret");
}
