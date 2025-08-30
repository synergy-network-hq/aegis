//! This module provides the Kyber post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqcrypto-mlkem` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

pub mod traits;

use pqcrypto_mlkem::mlkem512::{
    PublicKey as PublicKey512,
    SecretKey as SecretKey512,
    Ciphertext as Ciphertext512,
    encapsulate as encapsulate512,
    decapsulate as decapsulate512,
    keypair as keypair512,
};
use pqcrypto_mlkem::mlkem768::{
    PublicKey as PublicKey768,
    SecretKey as SecretKey768,
    Ciphertext as Ciphertext768,
    encapsulate as encapsulate768,
    decapsulate as decapsulate768,
    keypair as keypair768,
};
use pqcrypto_mlkem::mlkem1024::{
    PublicKey as PublicKey1024,
    SecretKey as SecretKey1024,
    Ciphertext as Ciphertext1024,
    encapsulate as encapsulate1024,
    decapsulate as decapsulate1024,
    keypair as keypair1024,
};
use pqcrypto_traits::kem::{ PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _ };
use wasm_bindgen::prelude::*;

/// Represents a Kyber key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// encapsulating and decapsulating shared secrets.
#[wasm_bindgen]
pub struct KyberKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl KyberKeyPair {
    /// Returns the public key component of the Kyber key pair.
    /// The public key is used by the sender to encapsulate a shared secret.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Kyber key pair.
    /// The secret key is used by the recipient to decapsulate the shared secret.
    /// It should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the Kyber encapsulation process, containing
/// both the ciphertext and the encapsulated shared secret.
#[wasm_bindgen]
pub struct KyberEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl KyberEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    /// This ciphertext is sent to the recipient for decapsulation.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret derived during encapsulation.
    /// This secret is used for symmetric encryption.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

// ML-KEM-512 Functions
#[wasm_bindgen]
pub fn kyber512_keygen() -> KyberKeyPair {
    let (pk, sk) = keypair512();
    KyberKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn kyber512_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    let pk = PublicKey512::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate512(&pk);
    Ok(KyberEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn kyber512_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = SecretKey512::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext512::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate512(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ML-KEM-768 Functions (existing, but renamed for consistency)
#[wasm_bindgen]
pub fn kyber768_keygen() -> KyberKeyPair {
    let (pk, sk) = keypair768();
    KyberKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn kyber768_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    let pk = PublicKey768::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate768(&pk);
    Ok(KyberEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn kyber768_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = SecretKey768::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext768::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate768(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ML-KEM-1024 Functions
#[wasm_bindgen]
pub fn kyber1024_keygen() -> KyberKeyPair {
    let (pk, sk) = keypair1024();
    KyberKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn kyber1024_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    let pk = PublicKey1024::from_bytes(public_key).map_err(|e|
        format!("Invalid public key: {:?}", e)
    )?;
    let (ss, ct) = encapsulate1024(&pk);
    Ok(KyberEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn kyber1024_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = SecretKey1024::from_bytes(secret_key).map_err(|e|
        format!("Invalid secret key: {:?}", e)
    )?;
    let ct = Ciphertext1024::from_bytes(ciphertext).map_err(|e|
        format!("Invalid ciphertext: {:?}", e)
    )?;
    let ss = decapsulate1024(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// Legacy functions (for backward compatibility - default to ML-KEM-768)
/// Generates a new Kyber key pair (ML-KEM-768).
///
/// This function uses the `pqcrypto-mlkem` backend to generate a fresh
/// public and secret key pair for the Kyber KEM scheme.
///
/// # Returns
///
/// A `KyberKeyPair` containing the newly generated public and secret keys.
#[wasm_bindgen]
pub fn kyber_keygen() -> KyberKeyPair {
    kyber768_keygen()
}

/// Encapsulates a shared secret using the provided Kyber public key (ML-KEM-768).
///
/// This function takes a recipient's public key and generates a ciphertext
/// and a shared secret. The ciphertext is sent to the recipient, who can
/// then decapsulate it to recover the same shared secret.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the recipient's Kyber public key.
///
/// # Returns
///
/// A `Result<KyberEncapsulated, JsValue>` which is:
/// - `Ok(KyberEncapsulated)` containing the generated ciphertext and shared secret.
/// - `Err(JsValue)` if the public key is invalid.
#[wasm_bindgen]
pub fn kyber_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    kyber768_encapsulate(public_key)
}

/// Decapsulates a shared secret using the provided Kyber secret key and ciphertext (ML-KEM-768).
///
/// This function takes a recipient's secret key and a ciphertext from the sender,
/// and recovers the shared secret that was encapsulated.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the recipient's Kyber secret key.
/// * `ciphertext` - A byte slice representing the ciphertext from the sender.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the recovered shared secret.
/// - `Err(JsValue)` if the secret key or ciphertext is invalid.
#[wasm_bindgen]
pub fn kyber_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    kyber768_decapsulate(secret_key, ciphertext)
}

// Native functions (for testing and non-WASM environments)
#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_keygen_native() -> KyberKeyPair {
    kyber512_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    kyber512_encapsulate(public_key).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    kyber512_decapsulate(secret_key, ciphertext).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_keygen_native() -> KyberKeyPair {
    kyber768_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    kyber768_encapsulate(public_key).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    kyber768_decapsulate(secret_key, ciphertext).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_keygen_native() -> KyberKeyPair {
    kyber1024_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    kyber1024_encapsulate(public_key).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    kyber1024_decapsulate(secret_key, ciphertext).map_err(|e|
        e.as_string().unwrap_or_else(|| "Unknown error".to_string())
    )
}

// Legacy native functions (for backward compatibility)
#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_keygen_native() -> KyberKeyPair {
    kyber768_keygen_native()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    kyber768_encapsulate_native(public_key)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_decapsulate_native(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    kyber768_decapsulate_native(secret_key, ciphertext)
}

pub use traits::*;
