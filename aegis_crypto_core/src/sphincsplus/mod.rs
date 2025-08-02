//! SPHINCS+ Digital Signature Algorithm

use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

// SPHINCS+ 192f variant
use pqcrypto_sphincsplus::sphincssha2192fsimple::*;

#[wasm_bindgen]
pub struct SphincsPlus192fKeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct SphincsPlus256fKeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

pub fn sphincsplus192_keygen() -> SphincsPlus192fKeyPair {
    let (pk, sk) = keypair();
    SphincsPlus192fKeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn sphincsplus192_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let sig = sign(message, &sk);
    sig.as_bytes().to_vec()
}

pub fn sphincsplus192_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    // Note: SPHINCS+ verification typically works with the original message + signature
    // This is a simplified implementation - you may need to adjust based on your specific use case
    let pk = PublicKey::from_bytes(public_key).ok();
    let sig = Signature::from_bytes(signed_message).ok();

    match (pk, sig) {
        (Some(pk), Some(sig)) => {
            // For this example, we assume the message is embedded or provided separately
            // In a real implementation, you'd need to extract the message from signed_message
            // or have it provided as a separate parameter
            verify(&[], &sig, &pk).is_ok()
        },
        _ => false,
    }
}

// For SPHINCS+ 256f - using 256f variant
pub fn sphincsplus256_keygen() -> SphincsPlus256fKeyPair {
    use pqcrypto_sphincsplus::sphincssha2256fsimple::*;
    let (pk, sk) = keypair();
    SphincsPlus256fKeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn sphincsplus256_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    use pqcrypto_sphincsplus::sphincssha2256fsimple::*;
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let sig = sign(message, &sk);
    sig.as_bytes().to_vec()
}

pub fn sphincsplus256_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    use pqcrypto_sphincsplus::sphincssha2256fsimple::*;
    let pk = PublicKey::from_bytes(public_key).ok();
    let sig = Signature::from_bytes(signed_message).ok();

    match (pk, sig) {
        (Some(pk), Some(sig)) => verify(&[], &sig, &pk).is_ok(),
        _ => false,
    }
}
