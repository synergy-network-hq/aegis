//! This module provides the Kyber post‑quantum key encapsulation mechanism (KEM) implementation.
//! It integrates with the `quantum_fips203` backend for cryptographic operations
//! and exposes key functions as WebAssembly (WASM) bindings for use in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use quantum_fips203::ml_kem_768;
use quantum_fips203::traits::{ Decaps, Encaps, KeyGen, SerDes };
use zeroize::Zeroize;

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

/// Generates a new Kyber key pair.
///
/// This function uses the `quantum_fips203` backend to generate a fresh
/// public and secret key pair for the Kyber KEM scheme.
/// The secret key is securely wiped from memory after the `KyberKeyPair`
/// is created.
///
/// # Returns
///
/// A `KyberKeyPair` containing the newly generated public and secret keys.
///
/// # Panics
///
/// Panics if the key generation process fails, which indicates a critical
/// issue with the underlying cryptographic backend.
#[wasm_bindgen]
pub fn kyber_keygen() -> KyberKeyPair {
    let (pk, mut sk) = ml_kem_768::KG::try_keygen().expect("Failed to generate keypair");
    let keypair = KyberKeyPair {
        pk: pk.into_bytes().to_vec(),
        sk: sk.clone().into_bytes().to_vec(),
    };
    sk.zeroize(); // Securely wipe secret key from memory
    keypair
}

/// Encapsulates a shared secret using the provided Kyber public key.
///
/// This function takes a recipient's public key and generates a ciphertext
/// and a shared secret.  The ciphertext is sent to the recipient, who can
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
/// - `Err(JsValue)` if the public key is invalid or encapsulation fails.
#[wasm_bindgen]
pub fn kyber_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    if public_key.len() != ml_kem_768::EK_LEN {
        return Err(
            format!(
                "Invalid public key length. Expected {}, got {}",
                ml_kem_768::EK_LEN,
                public_key.len()
            ).into()
        );
    }
    let pk_array: [u8; ml_kem_768::EK_LEN] = public_key
        .try_into()
        .map_err(|_| "Failed to convert public key to array")?;
    let pk = ml_kem_768::EncapsKey
        ::try_from_bytes(pk_array)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (shared_secret, ciphertext) = pk
        .try_encaps()
        .map_err(|e| format!("Failed to encapsulate: {:?}", e))?;
    Ok(KyberEncapsulated {
        ciphertext: ciphertext.into_bytes().to_vec(),
        shared_secret: shared_secret.into_bytes().to_vec(),
    })
}

/// Decapsulates a shared secret using the provided Kyber secret key and ciphertext.
///
/// This function takes the recipient's secret key and the ciphertext received
/// from the sender, and recovers the shared secret.  If the ciphertext is invalid
/// or tampered with, implicit rejection is performed, returning a random shared secret.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the recipient's Kyber secret key.
/// * `ciphertext` - A byte slice representing the ciphertext received from the sender.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the decapsulated shared secret.
/// - `Err(JsValue)` if the secret key or ciphertext are invalid, or decapsulation fails.
#[wasm_bindgen]
pub fn kyber_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    if secret_key.len() != ml_kem_768::DK_LEN {
        return Err(
            format!(
                "Invalid secret key length. Expected {}, got {}",
                ml_kem_768::DK_LEN,
                secret_key.len()
            ).into()
        );
    }
    if ciphertext.len() != ml_kem_768::CT_LEN {
        return Err(
            format!(
                "Invalid ciphertext length. Expected {}, got {}",
                ml_kem_768::CT_LEN,
                ciphertext.len()
            ).into()
        );
    }
    let sk_array: [u8; ml_kem_768::DK_LEN] = secret_key
        .try_into()
        .map_err(|_| "Failed to convert secret key to array")?;
    let mut sk = ml_kem_768::DecapsKey
        ::try_from_bytes(sk_array)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct_array: [u8; ml_kem_768::CT_LEN] = ciphertext
        .try_into()
        .map_err(|_| "Failed to convert ciphertext to array")?;
    let ct = ml_kem_768::CipherText
        ::try_from_bytes(ct_array)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let shared_secret = sk
        .try_decaps(&ct)
        .map_err(|e| format!("Failed to decapsulate: {:?}", e))?;
    sk.zeroize(); // Securely wipe secret key from memory
    Ok(shared_secret.into_bytes().to_vec())
}