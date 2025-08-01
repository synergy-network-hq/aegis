use pqcrypto_falcon::falcon512::{PublicKey, SecretKey, sign, keypair, open, SignedMessage};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, SignedMessage as _};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DilithiumKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl DilithiumKeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

#[wasm_bindgen]
pub fn dilithium_keygen() -> DilithiumKeyPair {
    let (pk, sk) = keypair();
    DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn dilithium_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sign(message, &sk);
    signed_message.as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn dilithium_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let signed_message = match SignedMessage::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => return false,
    };
    open(&signed_message, &pk).is_ok()
}
