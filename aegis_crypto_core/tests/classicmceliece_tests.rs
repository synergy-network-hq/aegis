//! Integration tests for the Classic McEliece key encapsulation mechanism (KEM).

use aegis_crypto_core::{classicmceliece_keygen, classicmceliece_encapsulate, classicmceliece_decapsulate};

#[test]
fn test_classicmceliece_encaps_and_decaps() {
    // Generate a recipient key pair - move to heap to avoid stack overflow
    let keypair = Box::new(classicmceliece_keygen());
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Encapsulate a shared secret
    let encapsulated = classicmceliece_encapsulate(&public_key).expect("encapsulation should succeed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret_enc = encapsulated.shared_secret();

    // Decapsulate the shared secret
    let shared_secret_dec = classicmceliece_decapsulate(&secret_key, &ciphertext).expect("decapsulation should succeed");

    assert_eq!(shared_secret_enc, shared_secret_dec, "Shared secrets should match");

    // Tamper with ciphertext
    let mut tampered_ct = ciphertext.clone();
    tampered_ct[0] ^= 0x01;

    // Decapsulation with tampered ciphertext should still succeed but produce a different secret
    let tampered_secret_res = classicmceliece_decapsulate(&secret_key, &tampered_ct);
    assert!(tampered_secret_res.is_ok(), "Decapsulation of tampered ciphertext should not fail");
    let tampered_secret = tampered_secret_res.unwrap();
    assert_ne!(shared_secret_enc, tampered_secret, "Decapsulation of tampered ciphertext should produce a different secret");
}
