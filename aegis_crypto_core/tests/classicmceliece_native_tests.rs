#![cfg(feature = "classicmceliece")]
//! Native tests for the Classic McEliece KEM.

use aegis_crypto_core::{
    classicmceliece_keygen,
    classicmceliece_encapsulate,
    classicmceliece_decapsulate,
};
use std::thread;

#[test]
fn test_classicmceliece_keygen_encapsulate_decapsulate() {
    // Run the test in a thread with a larger stack size (16MB)
    let handle = thread::Builder
        ::new()
        .stack_size(16 * 1024 * 1024) // 16MB stack
        .spawn(|| {
            let keypair = classicmceliece_keygen();
            let public_key = keypair.public_key();
            let secret_key = keypair.secret_key();

            let encapsulated = classicmceliece_encapsulate(&public_key).expect(
                "Encapsulation should succeed"
            );
            let ciphertext = encapsulated.ciphertext();
            let shared_secret = encapsulated.shared_secret();

            let decapsulated = classicmceliece_decapsulate(&secret_key, &ciphertext).expect(
                "Decapsulation should succeed"
            );
            assert_eq!(
                shared_secret,
                decapsulated,
                "Decapsulated secret should match encapsulated secret"
            );
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}

#[test]
fn test_classicmceliece_keypair_methods() {
    // Run the test in a thread with a larger stack size (16MB)
    let handle = thread::Builder
        ::new()
        .stack_size(16 * 1024 * 1024) // 16MB stack
        .spawn(|| {
            let keypair = classicmceliece_keygen();
            assert!(!keypair.public_key().is_empty());
            assert!(!keypair.secret_key().is_empty());
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}

#[test]
fn test_classicmceliece_encapsulated_methods() {
    // Run the test in a thread with a larger stack size (16MB)
    let handle = thread::Builder
        ::new()
        .stack_size(16 * 1024 * 1024) // 16MB stack
        .spawn(|| {
            let keypair = classicmceliece_keygen();
            let public_key = keypair.public_key();

            let encapsulated = classicmceliece_encapsulate(&public_key).expect(
                "Encapsulation should succeed"
            );
            assert!(!encapsulated.ciphertext().is_empty());
            assert!(!encapsulated.shared_secret().is_empty());
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}
