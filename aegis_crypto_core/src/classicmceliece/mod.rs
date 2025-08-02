//! Classic McEliece Key Encapsulation Mechanism

use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

// Classic McEliece variants - using the smaller ones for example
use pqcrypto_classicmceliece::mceliece348864::*;

#[wasm_bindgen]
pub struct ClassicMcEliece128KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct ClassicMcEliece128Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

#[wasm_bindgen]
pub struct ClassicMcEliece192KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct ClassicMcEliece192Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

#[wasm_bindgen]
pub struct ClassicMcEliece256KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct ClassicMcEliece256Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

// Classic McEliece 128 (using 348864 variant as representative)
pub fn classicmceliece128_keygen() -> ClassicMcEliece128KeyPair {
    let (pk, sk) = keypair();
    ClassicMcEliece128KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn classicmceliece128_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece128Encapsulated, wasm_bindgen::JsValue> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(ClassicMcEliece128Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn classicmceliece128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// Classic McEliece 192 (using 460896 variant)
pub fn classicmceliece192_keygen() -> ClassicMcEliece192KeyPair {
    use pqcrypto_classicmceliece::mceliece460896::*;
    let (pk, sk) = keypair();
    ClassicMcEliece192KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn classicmceliece192_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece192Encapsulated, wasm_bindgen::JsValue> {
    use pqcrypto_classicmceliece::mceliece460896::*;
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(ClassicMcEliece192Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn classicmceliece192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    use pqcrypto_classicmceliece::mceliece460896::*;
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// Classic McEliece 256 (using 6688128 variant)
pub fn classicmceliece256_keygen() -> ClassicMcEliece256KeyPair {
    use pqcrypto_classicmceliece::mceliece6688128::*;
    let (pk, sk) = keypair();
    ClassicMcEliece256KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn classicmceliece256_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece256Encapsulated, wasm_bindgen::JsValue> {
    use pqcrypto_classicmceliece::mceliece6688128::*;
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(ClassicMcEliece256Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn classicmceliece256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    use pqcrypto_classicmceliece::mceliece6688128::*;
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}
