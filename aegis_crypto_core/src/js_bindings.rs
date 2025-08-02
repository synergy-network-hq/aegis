//! JavaScript/WebAssembly bindings for Aegis Crypto Core — all algorithm variants.
//! Exports camelCase functions for each PQC algorithm and variant.

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;

    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    #[cfg(feature = "std")]
    use std::vec::Vec;

    // Import hash functions and utilities from hash module
    use crate::hash::{
        sha3_256_hash, sha3_256_hash_hex, sha3_256_hash_base64,
        sha3_512_hash, sha3_512_hash_hex, sha3_512_hash_base64,
        blake3_hash, blake3_hash_hex, blake3_hash_base64,
        hex_to_bytes, bytes_to_hex
    };

    // ===== Kyber =====
    #[cfg(feature = "kyber")]
    mod kyber_bindings {
        use super::*;
        use crate::{kyber_keygen, kyber_encapsulate, kyber_decapsulate, KyberKeyPair, KyberEncapsulated};

        #[wasm_bindgen(js_name = generateKyberKeyPair)]
        pub fn generate_kyber_keypair() -> KyberKeyPair {
            kyber_keygen()
        }

        #[wasm_bindgen(js_name = kyberEncapsulate)]
        pub fn kyber_encapsulate_js(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
            kyber_encapsulate(public_key)
        }

        #[wasm_bindgen(js_name = kyberDecapsulate)]
        pub fn kyber_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
            kyber_decapsulate(secret_key, ciphertext)
        }
    }

    // ===== Dilithium =====
    #[cfg(feature = "dilithium")]
    mod dilithium_bindings {
        use super::*;
        use crate::{dilithium_keygen, dilithium_sign, dilithium_verify, DilithiumKeyPair};

        #[wasm_bindgen(js_name = generateDilithium3KeyPair)]
        pub fn generate_dilithium3_keypair() -> DilithiumKeyPair {
            dilithium_keygen()
        }

        #[wasm_bindgen(js_name = signDilithium3)]
        pub fn sign_dilithium3_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
            dilithium_sign(secret_key, message)
        }

        #[wasm_bindgen(js_name = verifyDilithium3Signature)]
        pub fn verify_dilithium3_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
            dilithium_verify(public_key, message, signature)
        }
    }

    // ===== Falcon =====
    #[cfg(feature = "falcon")]
    mod falcon_bindings {
        use super::*;
        use crate::{falcon_keygen, falcon_sign, falcon_verify, FalconKeyPair};

        #[wasm_bindgen(js_name = generateFalconKeyPair)]
        pub fn generate_falcon_keypair() -> FalconKeyPair {
            falcon_keygen()
        }

        #[wasm_bindgen(js_name = signFalcon)]
        pub fn sign_falcon_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
            falcon_sign(secret_key, message)
        }

        #[wasm_bindgen(js_name = verifyFalconSignature)]
        pub fn verify_falcon_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
            falcon_verify(public_key, message, signature)
        }
    }

    // ===== Hash Functions =====
    #[wasm_bindgen(js_name = sha3_256)]
    pub fn sha3_256_js(data: &[u8]) -> Vec<u8> {
        sha3_256_hash(data)
    }

    #[wasm_bindgen(js_name = sha3_256_hex)]
    pub fn sha3_256_hex_js(data: &[u8]) -> String {
        sha3_256_hash_hex(data)
    }

    #[wasm_bindgen(js_name = sha3_512)]
    pub fn sha3_512_js(data: &[u8]) -> Vec<u8> {
        sha3_512_hash(data)
    }

    #[wasm_bindgen(js_name = blake3)]
    pub fn blake3_js(data: &[u8]) -> Vec<u8> {
        blake3_hash(data)
    }

    // ===== Utility Functions =====
    #[wasm_bindgen(js_name = hexToBytes)]
    pub fn hex_to_bytes_js(hex_str: &str) -> Result<Vec<u8>, JsValue> {
        hex_to_bytes(hex_str)
    }

    #[wasm_bindgen(js_name = bytesToHex)]
    pub fn bytes_to_hex_js(bytes: &[u8]) -> String {
        bytes_to_hex(bytes)
    }
}
