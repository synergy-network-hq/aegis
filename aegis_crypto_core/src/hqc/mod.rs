//! This module provides the HQC post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqcrypto-hqc` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use pqcrypto_hqc::hqc128::{
    PublicKey as PublicKey128,
    SecretKey as SecretKey128,
    Ciphertext as Ciphertext128,
    encapsulate as encapsulate128,
    decapsulate as decapsulate128,
    keypair as keypair128,
};
use pqcrypto_hqc::hqc192::{
    PublicKey as PublicKey192,
    SecretKey as SecretKey192,
    Ciphertext as Ciphertext192,
    encapsulate as encapsulate192,
    decapsulate as decapsulate192,
    keypair as keypair192,
};
use pqcrypto_hqc::hqc256::{
    PublicKey as PublicKey256,
    SecretKey as SecretKey256,
    Ciphertext as Ciphertext256,
    encapsulate as encapsulate256,
    decapsulate as decapsulate256,
    keypair as keypair256,
};
use pqcrypto_traits::kem::{ PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _ };
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Represents an HQC key pair, containing both the public and secret keys.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct HqcKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl HqcKeyPair {
    /// Returns the public key component of the HQC key pair.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the HQC key pair.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the HQC encapsulation process.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct HqcEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl HqcEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret derived during encapsulation.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

// HQC-128 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc128_keygen() -> HqcKeyPair {
    let (pk, sk) = keypair128();
    HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc128_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, Box<dyn std::error::Error>> {
    let pk = PublicKey128::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate128(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let sk = SecretKey128::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext128::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate128(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC-192 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc192_keygen() -> HqcKeyPair {
    let (pk, sk) = keypair192();
    HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc192_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, Box<dyn std::error::Error>> {
    let pk = PublicKey192::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate192(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let sk = SecretKey192::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext192::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate192(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC-256 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc256_keygen() -> HqcKeyPair {
    let (pk, sk) = keypair256();
    HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc256_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, Box<dyn std::error::Error>> {
    let pk = PublicKey256::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate256(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let sk = SecretKey256::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext256::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate256(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// Legacy functions (for backward compatibility - default to HQC-128)
/// Generates a new HQC key pair (HQC-128).
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc_keygen() -> HqcKeyPair {
    hqc128_keygen()
}

/// Encapsulates a shared secret using the provided HQC public key (HQC-128).
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, Box<dyn std::error::Error>> {
    hqc128_encapsulate(public_key)
}

/// Decapsulates a shared secret using the provided HQC secret key and ciphertext (HQC-128).
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hqc_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    hqc128_decapsulate(secret_key, ciphertext)
}
