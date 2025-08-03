//! Native tests for the Kyber key encapsulation mechanism (KEM).
//!
//! These tests ensure that encapsulation and decapsulation produce matching
//! shared secrets and that incorrect inputs are handled correctly.

use aegis_crypto_core::{kyber_keygen, kyber_encapsulate, kyber_decapsulate};
use aegis_crypto_core::kyber::utils::{public_key_length, secret_key_length, ciphertext_length, shared_secret_length};

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

#[test]
fn test_kyber_keygen_encapsulate_decapsulate() {
    let keypair = kyber_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    assert_eq!(public_key.len(), public_key_length());
    assert_eq!(secret_key.len(), secret_key_length());

    let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    assert_eq!(ciphertext.len(), ciphertext_length());
    assert_eq!(shared_secret1.len(), shared_secret_length());

    let shared_secret2 = kyber_decapsulate(&secret_key, &ciphertext).expect("Decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_invalid_key_lengths() {
    let short_pk = vec![0u8; public_key_length() - 1];
    let result = kyber_encapsulate(&short_pk);
    assert!(result.is_err(), "Encapsulation should fail with invalid public key length");

    let keypair = kyber_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();

    let short_sk = vec![0u8; secret_key_length() - 1];
    let result = kyber_decapsulate(&short_sk, &ciphertext);
    assert!(result.is_err(), "Decapsulation should fail with invalid secret key length");
}

#[test]
fn test_invalid_ciphertext_length() {
    let keypair = kyber_keygen().expect("Keygen failed");
    let secret_key = keypair.secret_key();
    let short_ciphertext = vec![0u8; ciphertext_length() - 1];

    let result = kyber_decapsulate(&secret_key, &short_ciphertext);
    assert!(result.is_err(), "Decapsulation should fail with invalid ciphertext length");
}

#[test]
fn test_kyber_keypair_methods() {
    let keypair = kyber_keygen().expect("Keygen failed");
    assert_eq!(keypair.public_key().len(), keypair.public_key_length());
    assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
}

#[test]
fn test_kyber_encapsulated_methods() {
    let keypair = kyber_keygen().expect("Keygen failed");
    let public_key = keypair.public_key();
    let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");

    assert_eq!(encapsulated.ciphertext().len(), encapsulated.ciphertext_length());
    assert_eq!(encapsulated.shared_secret().len(), encapsulated.shared_secret_length());
}
