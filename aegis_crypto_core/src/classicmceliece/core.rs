//! Core Classic McEliece implementation.
//!
//! Provides the core Classic McEliece post-quantum KEM scheme implementation using pqcrypto.
//! WASM bindings are provided for JavaScript/TypeScript interop.

use wasm_bindgen::prelude::*;
use pqcrypto_traits::kem::{ PublicKey, SecretKey, Ciphertext, SharedSecret };
use super::utils::*;
use alloc::string::ToString;

#[cfg(not(feature = "std"))]
use alloc::{ vec::Vec, string::String };
#[cfg(feature = "std")]
use std::{ vec::Vec, string::String };

use pqcrypto_classicmceliece::mceliece348864;
use pqcrypto_classicmceliece::mceliece460896;
use pqcrypto_classicmceliece::mceliece6688128;

/// Classic McEliece 128-bit security key pair.
#[wasm_bindgen]
pub struct ClassicMcEliece128KeyPair {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece128KeyPair {
    /// Returns the public key bytes.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }
    /// Returns the secret key bytes.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }
    /// Public key length.
    #[wasm_bindgen]
    pub fn public_key_length(&self) -> usize {
        self.public_key.len()
    }
    /// Secret key length.
    #[wasm_bindgen]
    pub fn secret_key_length(&self) -> usize {
        self.secret_key.len()
    }
}

/// Encapsulation result for 128-bit security.
#[wasm_bindgen]
pub struct ClassicMcEliece128Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece128Encapsulated {
    /// Returns the ciphertext bytes.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret bytes.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
    /// Ciphertext length.
    #[wasm_bindgen]
    pub fn ciphertext_length(&self) -> usize {
        self.ciphertext.len()
    }
    /// Shared secret length.
    #[wasm_bindgen]
    pub fn shared_secret_length(&self) -> usize {
        self.shared_secret.len()
    }
}

/// Classic McEliece 192-bit security key pair.
#[wasm_bindgen]
pub struct ClassicMcEliece192KeyPair {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece192KeyPair {
    /// Returns a copy of the public key as a vector of bytes.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    /// Returns a copy of the secret key as a vector of bytes.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }

    /// Returns the length of the public key in bytes.
    #[wasm_bindgen]
    pub fn public_key_length(&self) -> usize {
        self.public_key.len()
    }

    /// Returns the length of the secret key in bytes.
    #[wasm_bindgen]
    pub fn secret_key_length(&self) -> usize {
        self.secret_key.len()
    }
}

/// Represents the result of Classic McEliece 192-bit encapsulation.
#[wasm_bindgen]
pub struct ClassicMcEliece192Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece192Encapsulated {
    /// Returns the ciphertext bytes.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret bytes.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
    /// Ciphertext length.
    #[wasm_bindgen]
    pub fn ciphertext_length(&self) -> usize {
        self.ciphertext.len()
    }
    /// Shared secret length.
    #[wasm_bindgen]
    pub fn shared_secret_length(&self) -> usize {
        self.shared_secret.len()
    }
}

/// Classic McEliece 256-bit security key pair.
#[wasm_bindgen]
pub struct ClassicMcEliece256KeyPair {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece256KeyPair {
    /// Returns a copy of the public key as a vector of bytes.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    /// Returns a copy of the secret key as a vector of bytes.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }

    /// Returns the length of the public key in bytes.
    #[wasm_bindgen]
    pub fn public_key_length(&self) -> usize {
        self.public_key.len()
    }

    /// Returns the length of the secret key in bytes.
    #[wasm_bindgen]
    pub fn secret_key_length(&self) -> usize {
        self.secret_key.len()
    }
}

/// Represents the result of Classic McEliece 256-bit encapsulation.
#[wasm_bindgen]
pub struct ClassicMcEliece256Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece256Encapsulated {
    /// Returns the ciphertext bytes.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret bytes.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
    /// Ciphertext length.
    #[wasm_bindgen]
    pub fn ciphertext_length(&self) -> usize {
        self.ciphertext.len()
    }
    /// Shared secret length.
    #[wasm_bindgen]
    pub fn shared_secret_length(&self) -> usize {
        self.shared_secret.len()
    }
}

// ---- Native KEM functions ----

/// Generate a ClassicMcEliece-128 keypair natively (non-wasm).
pub fn classicmceliece128_keygen_native() -> Result<ClassicMcEliece128KeyPair, String> {
    let (pk, sk) = mceliece348864::keypair();
    Ok(ClassicMcEliece128KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    })
}
/// Encapsulate with ClassicMcEliece-128 natively.
pub fn classicmceliece128_encapsulate_native(
    public_key: &[u8]
) -> Result<ClassicMcEliece128Encapsulated, String> {
    validate_public_key_length_128(public_key).map_err(|e| e.to_string())?;
    let pk = mceliece348864::PublicKey
        ::from_bytes(public_key)
        .map_err(|_| "Invalid public key".to_string())?;
    let (ss, ct) = mceliece348864::encapsulate(&pk);
    Ok(ClassicMcEliece128Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
/// Decapsulate with ClassicMcEliece-128 natively.
pub fn classicmceliece128_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    validate_secret_key_length_128(secret_key).map_err(|e| e.to_string())?;
    validate_ciphertext_length_128(ciphertext).map_err(|e| e.to_string())?;
    let sk = mceliece348864::SecretKey
        ::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key".to_string())?;
    let ct = mceliece348864::Ciphertext
        ::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext".to_string())?;
    let ss = mceliece348864::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

/// Generate a ClassicMcEliece-192 keypair natively (non-wasm).
pub fn classicmceliece192_keygen_native() -> Result<ClassicMcEliece192KeyPair, String> {
    let (pk, sk) = mceliece460896::keypair();
    Ok(ClassicMcEliece192KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    })
}
/// Encapsulate with ClassicMcEliece-192 natively.
pub fn classicmceliece192_encapsulate_native(
    public_key: &[u8]
) -> Result<ClassicMcEliece192Encapsulated, String> {
    validate_public_key_length_192(public_key).map_err(|e| e.to_string())?;
    let pk = mceliece460896::PublicKey
        ::from_bytes(public_key)
        .map_err(|_| "Invalid public key".to_string())?;
    let (ss, ct) = mceliece460896::encapsulate(&pk);
    Ok(ClassicMcEliece192Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
/// Decapsulate with ClassicMcEliece-192 natively.
pub fn classicmceliece192_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    validate_secret_key_length_192(secret_key).map_err(|e| e.to_string())?;
    validate_ciphertext_length_192(ciphertext).map_err(|e| e.to_string())?;
    let sk = mceliece460896::SecretKey
        ::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key".to_string())?;
    let ct = mceliece460896::Ciphertext
        ::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext".to_string())?;
    let ss = mceliece460896::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

/// Generate a ClassicMcEliece-256 keypair natively (non-wasm).
pub fn classicmceliece256_keygen_native() -> Result<ClassicMcEliece256KeyPair, String> {
    let (pk, sk) = mceliece6688128::keypair();
    Ok(ClassicMcEliece256KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    })
}
/// Encapsulate with ClassicMcEliece-256 natively.
pub fn classicmceliece256_encapsulate_native(
    public_key: &[u8]
) -> Result<ClassicMcEliece256Encapsulated, String> {
    validate_public_key_length_256(public_key).map_err(|e| e.to_string())?;
    let pk = mceliece6688128::PublicKey
        ::from_bytes(public_key)
        .map_err(|_| "Invalid public key".to_string())?;
    let (ss, ct) = mceliece6688128::encapsulate(&pk);
    Ok(ClassicMcEliece256Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
/// Decapsulate with ClassicMcEliece-256 natively.
pub fn classicmceliece256_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    validate_secret_key_length_256(secret_key).map_err(|e| e.to_string())?;
    validate_ciphertext_length_256(ciphertext).map_err(|e| e.to_string())?;
    let sk = mceliece6688128::SecretKey
        ::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key".to_string())?;
    let ct = mceliece6688128::Ciphertext
        ::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext".to_string())?;
    let ss = mceliece6688128::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ---- WASM Bindings ----

/// Generate a ClassicMcEliece-128 keypair (WASM).
#[wasm_bindgen]
pub fn classicmceliece128_keygen() -> Result<ClassicMcEliece128KeyPair, JsValue> {
    classicmceliece128_keygen_native().map_err(|e| JsValue::from_str(&e))
}

/// Encapsulate with ClassicMcEliece-128 (WASM).
#[wasm_bindgen]
pub fn classicmceliece128_encapsulate(
    public_key: &[u8]
) -> Result<ClassicMcEliece128Encapsulated, JsValue> {
    classicmceliece128_encapsulate_native(public_key).map_err(|e| JsValue::from_str(&e))
}

/// Decapsulate with ClassicMcEliece-128 (WASM).
#[wasm_bindgen]
pub fn classicmceliece128_decapsulate(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, JsValue> {
    classicmceliece128_decapsulate_native(secret_key, ciphertext).map_err(|e| JsValue::from_str(&e))
}

/// Generate a ClassicMcEliece-192 keypair (WASM).
#[wasm_bindgen]
pub fn classicmceliece192_keygen() -> Result<ClassicMcEliece192KeyPair, JsValue> {
    classicmceliece192_keygen_native().map_err(|e| JsValue::from_str(&e))
}

/// Encapsulate with ClassicMcEliece-192 (WASM).
#[wasm_bindgen]
pub fn classicmceliece192_encapsulate(
    public_key: &[u8]
) -> Result<ClassicMcEliece192Encapsulated, JsValue> {
    classicmceliece192_encapsulate_native(public_key).map_err(|e| JsValue::from_str(&e))
}

/// Decapsulate with ClassicMcEliece-192 (WASM).
#[wasm_bindgen]
pub fn classicmceliece192_decapsulate(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, JsValue> {
    classicmceliece192_decapsulate_native(secret_key, ciphertext).map_err(|e| JsValue::from_str(&e))
}

/// Generate a ClassicMcEliece-256 keypair (WASM).
#[wasm_bindgen]
pub fn classicmceliece256_keygen() -> Result<ClassicMcEliece256KeyPair, JsValue> {
    classicmceliece256_keygen_native().map_err(|e| JsValue::from_str(&e))
}

/// Encapsulate with ClassicMcEliece-256 (WASM).
#[wasm_bindgen]
pub fn classicmceliece256_encapsulate(
    public_key: &[u8]
) -> Result<ClassicMcEliece256Encapsulated, JsValue> {
    classicmceliece256_encapsulate_native(public_key).map_err(|e| JsValue::from_str(&e))
}

/// Decapsulate with ClassicMcEliece-256 (WASM).
#[wasm_bindgen]
pub fn classicmceliece256_decapsulate(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, JsValue> {
    classicmceliece256_decapsulate_native(secret_key, ciphertext).map_err(|e| JsValue::from_str(&e))
}

// ---- Generic Default-to-128 Wrappers ----

/// Generate Classic McEliece keypair (128-bit by default).
pub fn classicmceliece_keygen() -> ClassicMcEliece128KeyPair {
    classicmceliece128_keygen_native().expect("Classic McEliece keygen should not fail")
}
/// Encapsulate (128-bit default).
pub fn classicmceliece_encapsulate(
    public_key: &[u8]
) -> Result<ClassicMcEliece128Encapsulated, String> {
    classicmceliece128_encapsulate_native(public_key)
}
/// Decapsulate (128-bit default).
pub fn classicmceliece_decapsulate(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    classicmceliece128_decapsulate_native(secret_key, ciphertext)
}

// ---- Manual Demo Functions (for running, not tests) ----

/// Demo for 128-bit KEM (should pass and print nothing).
pub fn test_classicmceliece128_keygen_encap_decap_demo() {
    let keypair = classicmceliece128_keygen_native().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let encapsulated = classicmceliece128_encapsulate_native(&public_key).expect(
        "Encapsulation failed"
    );
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();
    let shared_secret2 = classicmceliece128_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}
/// Demo for 192-bit KEM.
pub fn test_classicmceliece192_keygen_encap_decap_demo() {
    let keypair = classicmceliece192_keygen_native().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let encapsulated = classicmceliece192_encapsulate_native(&public_key).expect(
        "Encapsulation failed"
    );
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();
    let shared_secret2 = classicmceliece192_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}
/// Demo for 256-bit KEM.
pub fn test_classicmceliece256_keygen_encap_decap_demo() {
    let keypair = classicmceliece256_keygen_native().expect("Keygen failed");
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let encapsulated = classicmceliece256_encapsulate_native(&public_key).expect(
        "Encapsulation failed"
    );
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();
    let shared_secret2 = classicmceliece256_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

// ---- Unit Tests ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classicmceliece128_keygen_encap_decap() {
        let keypair = classicmceliece128_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = classicmceliece128_encapsulate(&public_key).expect(
            "Encapsulation failed"
        );
        let ciphertext = encapsulated.ciphertext();
        let shared_secret1 = encapsulated.shared_secret();
        let shared_secret2 = classicmceliece128_decapsulate(&secret_key, &ciphertext).expect(
            "Decapsulation failed"
        );
        assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
    }

    #[test]
    fn test_classicmceliece192_keygen_encap_decap() {
        let keypair = classicmceliece192_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = classicmceliece192_encapsulate(&public_key).expect(
            "Encapsulation failed"
        );
        let ciphertext = encapsulated.ciphertext();
        let shared_secret1 = encapsulated.shared_secret();
        let shared_secret2 = classicmceliece192_decapsulate(&secret_key, &ciphertext).expect(
            "Decapsulation failed"
        );
        assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
    }

    #[test]
    fn test_classicmceliece256_keygen_encap_decap() {
        let keypair = classicmceliece256_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = classicmceliece256_encapsulate(&public_key).expect(
            "Encapsulation failed"
        );
        let ciphertext = encapsulated.ciphertext();
        let shared_secret1 = encapsulated.shared_secret();
        let shared_secret2 = classicmceliece256_decapsulate(&secret_key, &ciphertext).expect(
            "Decapsulation failed"
        );
        assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
    }

    #[test]
    fn test_invalid_key_lengths() {
        let short_pk = vec![0u8; 100];
        let result = classicmceliece128_encapsulate(&short_pk);
        assert!(result.is_err(), "Encapsulation should fail with invalid public key length");

        let short_sk = vec![0u8; 100];
        let short_ct = vec![0u8; 50];
        let result = classicmceliece128_decapsulate(&short_sk, &short_ct);
        assert!(result.is_err(), "Decapsulation should fail with invalid key/ciphertext length");
    }

    #[test]
    fn test_keypair_methods() {
        let keypair = classicmceliece128_keygen().expect("Keygen failed");
        assert_eq!(keypair.public_key().len(), keypair.public_key_length());
        assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
        let encapsulated = classicmceliece128_encapsulate(&keypair.public_key()).expect(
            "Encapsulation failed"
        );
        assert_eq!(encapsulated.ciphertext().len(), encapsulated.ciphertext_length());
        assert_eq!(encapsulated.shared_secret().len(), encapsulated.shared_secret_length());
    }
}
