//! Browser (wasm32-unknown-unknown) tests for SPHINCS+ signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_sphincsplus_sign_verify_browser() {
    let kp = sphincsplus_keygen().expect("sphincs+ keygen");
    let msg = b"Aegis SPHINCS+ browser test";
    let sig = sphincsplus_sign(kp.secret_key(), msg).expect("sphincs+ sign");
    assert!(sphincsplus_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_sphincsplus_tampered_sig_browser() {
    let kp = sphincsplus_keygen().expect("sphincs+ keygen");
    let msg = b"Aegis SPHINCS+ tamper";
    let mut sig = sphincsplus_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!sphincsplus_verify(kp.public_key(), msg, &sig));
}
