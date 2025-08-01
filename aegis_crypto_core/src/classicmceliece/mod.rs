use pqcrypto_classicmceliece::mceliece348864::{PublicKey, SecretKey, Ciphertext};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClassicMcElieceKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcElieceKeyPair {
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
pub struct ClassicMcElieceEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcElieceEncapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

#[wasm_bindgen]
pub fn classicmceliece_keygen() -> ClassicMcElieceKeyPair {
    let (pk, sk) = pqcrypto_classicmceliece::mceliece348864::keypair();
    ClassicMcElieceKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn classicmceliece_encapsulate(public_key: &[u8]) -> Result<ClassicMcElieceEncapsulated, JsValue> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|e| JsValue::from_str(&format!("Invalid public key: {:?}", e)))?;
    let (ss, ct) = pqcrypto_classicmceliece::mceliece348864::encapsulate(&pk);
    Ok(ClassicMcElieceEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn classicmceliece_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|e| JsValue::from_str(&format!("Invalid secret key: {:?}", e)))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|e| JsValue::from_str(&format!("Invalid ciphertext: {:?}", e)))?;
    let ss = pqcrypto_classicmceliece::mceliece348864::decapsulate(&ct, &sk);

    Ok(ss.as_bytes().to_vec())
}
