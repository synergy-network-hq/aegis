use pqcrypto_mldsa::mldsa44::{
    PublicKey as PublicKey44,
    SecretKey as SecretKey44,
    sign as sign44,
    keypair as keypair44,
    open as open44,
    SignedMessage as SignedMessage44,
};
use pqcrypto_mldsa::mldsa65::{
    PublicKey as PublicKey65,
    SecretKey as SecretKey65,
    sign as sign65,
    keypair as keypair65,
    open as open65,
    SignedMessage as SignedMessage65,
};
use pqcrypto_mldsa::mldsa87::{
    PublicKey as PublicKey87,
    SecretKey as SecretKey87,
    sign as sign87,
    keypair as keypair87,
    open as open87,
    SignedMessage as SignedMessage87,
};
use pqcrypto_traits::sign::{ PublicKey as _, SecretKey as _, SignedMessage as _ };
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct DilithiumKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl DilithiumKeyPair {
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

// ML-DSA-44 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium44_keygen() -> DilithiumKeyPair {
    let (pk, sk) = keypair44();
    DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium44_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey44::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sign44(message, &sk);
    signed_message.as_bytes().to_vec()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium44_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match PublicKey44::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            return false;
        }
    };
    let signed_message = match SignedMessage44::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => {
            return false;
        }
    };
    open44(&signed_message, &pk).is_ok()
}

// ML-DSA-65 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium65_keygen() -> DilithiumKeyPair {
    let (pk, sk) = keypair65();
    DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium65_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey65::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sign65(message, &sk);
    signed_message.as_bytes().to_vec()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium65_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match PublicKey65::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            return false;
        }
    };
    let signed_message = match SignedMessage65::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => {
            return false;
        }
    };
    open65(&signed_message, &pk).is_ok()
}

// ML-DSA-87 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium87_keygen() -> DilithiumKeyPair {
    let (pk, sk) = keypair87();
    DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium87_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey87::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sign87(message, &sk);
    signed_message.as_bytes().to_vec()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium87_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match PublicKey87::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            return false;
        }
    };
    let signed_message = match SignedMessage87::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => {
            return false;
        }
    };
    open87(&signed_message, &pk).is_ok()
}

// Legacy functions (for backward compatibility - default to ML-DSA-87)
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium_keygen() -> DilithiumKeyPair {
    dilithium87_keygen()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    dilithium87_sign(secret_key, message)
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn dilithium_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    dilithium87_verify(public_key, signed_message)
}
