//! Falcon Digital Signature Algorithm

use pqcrypto_falcon::falcon512::*;
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

#[wasm_bindgen]
pub struct FalconKeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

pub fn falcon_keygen() -> FalconKeyPair {
    let (pk, sk) = keypair();
    FalconKeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn falcon_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let sig = sign(message, &sk);
    sig.as_bytes().to_vec()
}

pub fn falcon_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let pk = PublicKey::from_bytes(public_key).ok();
    let sig = Signature::from_bytes(signature).ok();

    match (pk, sig) {
        (Some(pk), Some(sig)) => verify(message, &sig, &pk).is_ok(),
        _ => false,
    }
}
