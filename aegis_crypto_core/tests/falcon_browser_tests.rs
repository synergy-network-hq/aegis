//! Browser (wasm32-unknown-unknown) tests for Falcon signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{falcon_keygen, falcon_sign, falcon_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_falcon_sign_verify_browser() {
    let kp = falcon_keygen().expect("falcon keygen");
    let msg = b"Aegis Falcon browser test";
    let sig = falcon_sign(kp.secret_key(), msg).expect("falcon sign");
    assert!(falcon_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_falcon_tampered_sig_browser() {
    let kp = falcon_keygen().expect("falcon keygen");
    let msg = b"Aegis Falcon tamper";
    let mut sig = falcon_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!falcon_verify(kp.public_key(), msg, &sig));
}
