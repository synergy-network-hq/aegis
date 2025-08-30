//! Tests for trait-based API implementations.

use aegis_crypto_core::traits::{ Kem, Algorithm };
use aegis_crypto_core::kyber::traits::{ Kyber768, KyberPublicKey, KyberSecretKey };

#[test]
fn test_kyber768_trait_implementation() {
    // Test key generation
    let (public_key, secret_key) = Kyber768::keygen().expect("Key generation should succeed");

    // Test encapsulation
    let encapsulated = Kyber768::encapsulate(&public_key).expect("Encapsulation should succeed");

    // Test decapsulation
    let decapsulated = Kyber768::decapsulate(&secret_key, encapsulated.0.as_ref()).expect(
        "Decapsulation should succeed"
    );

    // Verify shared secrets match
    assert_eq!(encapsulated.1.as_ref(), decapsulated.as_ref(), "Shared secrets should match");
}

#[test]
fn test_kyber768_algorithm_trait() {
    assert_eq!(Kyber768::name(), "Kyber768");
    assert_eq!(Kyber768::security_level(), 192);
}

#[test]
fn test_kyber768_error_handling() {
    // Test with invalid public key
    let invalid_pk = KyberPublicKey(vec![0u8; 100]); // Wrong length
    let result = Kyber768::encapsulate(&invalid_pk);
    assert!(result.is_err(), "Encapsulation should fail with invalid public key");

    // Test with invalid secret key
    let invalid_sk = KyberSecretKey(vec![0u8; 100]); // Wrong length
    let invalid_ct = vec![0u8; 100]; // Wrong length
    let result = Kyber768::decapsulate(&invalid_sk, &invalid_ct);
    assert!(result.is_err(), "Decapsulation should fail with invalid inputs");
}
