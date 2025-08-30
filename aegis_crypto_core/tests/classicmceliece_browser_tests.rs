//! Browser (wasm32-unknown-unknown) tests for Classic McEliece KEM.
//! Uses wasm-bindgen-test in headless browser.
//! Guarded to exclude WASI and native targets.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{
    classicmceliece128_keygen, classicmceliece128_encapsulate, classicmceliece128_decapsulate,
    classicmceliece192_keygen, classicmceliece192_encapsulate, classicmceliece192_decapsulate,
    classicmceliece256_keygen, classicmceliece256_encapsulate, classicmceliece256_decapsulate,
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_classicmceliece128_encaps_and_decaps_browser() {
    let kp = classicmceliece128_keygen().expect("keygen 128");
    let ct_ss = classicmceliece128_encapsulate(kp.public_key()).expect("encaps 128");
    let ss_dec = classicmceliece128_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("decaps 128");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_classicmceliece192_encaps_and_decaps_browser() {
    let kp = classicmceliece192_keygen().expect("keygen 192");
    let ct_ss = classicmceliece192_encapsulate(kp.public_key()).expect("encaps 192");
    let ss_dec = classicmceliece192_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("decaps 192");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_classicmceliece256_encaps_and_decaps_browser() {
    let kp = classicmceliece256_keygen().expect("keygen 256");
    let ct_ss = classicmceliece256_encapsulate(kp.public_key()).expect("encaps 256");
    let ss_dec = classicmceliece256_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("decaps 256");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}
