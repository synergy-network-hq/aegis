//! Browser (wasm32-unknown-unknown) tests for Kyber KEM using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{kyber_keygen, kyber_encapsulate, kyber_decapsulate};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_kyber_encaps_and_decaps_browser() {
    let kp = kyber_keygen().expect("kyber keygen");
    let ct_ss = kyber_encapsulate(kp.public_key()).expect("kyber encaps");
    let ss_dec = kyber_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("kyber decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_kyber_decaps_tampered_browser() {
    let kp = kyber_keygen().expect("kyber keygen");
    let ct_ss = kyber_encapsulate(kp.public_key()).expect("kyber encaps");
    let mut tampered = ct_ss.ciphertext().to_vec();
    if !tampered.is_empty() { tampered[0] ^= 0x01; }
    let ss_tampered = kyber_decapsulate(kp.secret_key(), &tampered).expect("kyber decaps tampered");
    assert_ne!(ct_ss.shared_secret(), ss_tampered.as_slice());
}
