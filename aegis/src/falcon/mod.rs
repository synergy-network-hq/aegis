use wasm_bindgen::prelude::*;
use quantum_fips206::falcon512::{ self, SecretKey, PublicKey, Signature };
use rand::thread_rng;
use rand::Rng;

#[wasm_bindgen]
pub struct FalconKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl FalconKeyPair {
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
pub fn falcon_keygen() -> FalconKeyPair {
    let (sk, pk) = falcon512::keygen(thread_rng().gen());
    FalconKeyPair { pk: pk.to_bytes().to_vec(), sk: sk.to_bytes().to_vec() }
}

#[wasm_bindgen]
pub fn falcon_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = SecretKey::from_bytes(secret_key).map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let signature = falcon512::sign(message, &sk);
    Ok(signature.to_bytes().to_vec())
}

#[wasm_bindgen]
pub fn falcon_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, JsValue> {
    let pk = PublicKey::from_bytes(public_key).map_err(|e| format!("Invalid public key: {:?}", e))?;
    let sig = Signature::from_bytes(signature).map_err(|e| format!("Invalid signature: {:?}", e))?;
    let is_valid = falcon512::verify(message, &sig, &pk);
    Ok(is_valid)
}