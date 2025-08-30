//! Integration tests for the Classic McEliece key encapsulation mechanism (KEM).

use aegis_crypto_core::classicmceliece::{classicmceliece128_keygen_native, classicmceliece128_encapsulate_native, classicmceliece128_decapsulate_native, classicmceliece192_keygen_native, classicmceliece192_encapsulate_native, classicmceliece192_decapsulate_native, classicmceliece256_keygen_native, classicmceliece256_encapsulate_native, classicmceliece256_decapsulate_native, ClassicMcElieceKeyPair, ClassicMcElieceEncapsulated};
use std::thread;

#[test]
fn test_classicmceliece128_encaps_and_decaps() {
    // Run the test in a thread with a larger stack size (8MB)
    let handle = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB stack
        .spawn(|| {
            run_classicmceliece_test(128);
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}

#[test]
fn test_classicmceliece192_encaps_and_decaps() {
    // Run the test in a thread with a larger stack size (8MB)
    let handle = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB stack
        .spawn(|| {
            run_classicmceliece_test(192);
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}

#[test]
fn test_classicmceliece256_encaps_and_decaps() {
    // Run the test in a thread with a larger stack size (8MB)
    let handle = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB stack
        .spawn(|| {
            run_classicmceliece_test(256);
        })
        .expect("Failed to spawn test thread");

    handle.join().expect("Test thread panicked");
}

fn run_classicmceliece_test(variant: u16) {
    let (keypair, encapsulate, decapsulate): (ClassicMcElieceKeyPair, fn(&[u8]) -> Result<ClassicMcElieceEncapsulated, String>, fn(&[u8], &[u8]) -> Result<Vec<u8>, String>) = match variant {
        128 => (classicmceliece128_keygen_native().expect("keygen failed"), classicmceliece128_encapsulate_native, classicmceliece128_decapsulate_native),
        192 => (classicmceliece192_keygen_native().expect("keygen failed"), classicmceliece192_encapsulate_native, classicmceliece192_decapsulate_native),
        256 => (classicmceliece256_keygen_native().expect("keygen failed"), classicmceliece256_encapsulate_native, classicmceliece256_decapsulate_native),
        _ => panic!("Invalid Classic McEliece variant"),
    };

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Encapsulate a shared secret
    let encapsulated = encapsulate(&public_key).expect("encapsulation should succeed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret_enc = encapsulated.shared_secret();

    // Decapsulate the shared secret
    let shared_secret_dec = decapsulate(&secret_key, &ciphertext).expect("decapsulation should succeed");

    assert_eq!(shared_secret_enc, shared_secret_dec.as_slice(), "Shared secrets should match");

    // Tamper with ciphertext
    let mut tampered_ct = ciphertext.to_vec();
    tampered_ct[0] ^= 0x01;

    // Decapsulation with tampered ciphertext should still succeed but produce a different secret
    let tampered_secret_res = decapsulate(&secret_key, &tampered_ct);
    assert!(tampered_secret_res.is_ok(), "Decapsulation of tampered ciphertext should not fail");
    let tampered_secret = tampered_secret_res.unwrap();
    assert_ne!(shared_secret_enc, tampered_secret.as_slice(), "Decapsulation of tampered ciphertext should produce a different secret");
}
