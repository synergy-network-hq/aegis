//! Browser (wasm32-unknown-unknown) tests for Dilithium signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{dilithium_keygen, dilithium_sign, dilithium_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_dilithium_sign_verify_browser() {
    let kp = dilithium_keygen().expect("dilithium keygen");
    let msg = b"Aegis Dilithium browser test";
    let sig = dilithium_sign(kp.secret_key(), msg).expect("dilithium sign");
    assert!(dilithium_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_dilithium_tampered_sig_browser() {
    let kp = dilithium_keygen().expect("dilithium keygen");
    let msg = b"Aegis Dilithium tamper";
    let mut sig = dilithium_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!dilithium_verify(kp.public_key(), msg, &sig));
}
