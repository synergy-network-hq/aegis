use quantum_fips204::ml_dsa_65;
use quantum_fips204::traits::{Signer, Verifier, SerDes};
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

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
    let (pk, mut sk) = ml_dsa_65::try_keygen().expect("Failed to generate keypair");
    let keypair = DilithiumKeyPair { pk: pk.into_bytes().to_vec(), sk: sk.clone().into_bytes().to_vec() };
    sk.zeroize(); // Securely wipe secret key from memory
    keypair
}

#[wasm_bindgen]
pub fn dilithium_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    if secret_key.len() != ml_dsa_65::SK_LEN {
        return Err(format!("Invalid secret key length. Expected {}, got {}", ml_dsa_65::SK_LEN, secret_key.len()).into());
    }
    let sk_array: [u8; ml_dsa_65::SK_LEN] = secret_key.try_into()
        .map_err(|_| "Failed to convert secret key to array")?;
    let mut sk = ml_dsa_65::PrivateKey::try_from_bytes(sk_array)
        .map_err(|e| format!("Invalid secret key: {}", e))?;
    let signature = sk.try_sign(message, &[]).map_err(|e| format!("Failed to sign: {}", e))?;
    sk.zeroize(); // Securely wipe secret key from memory
    Ok(signature.to_vec())
}

#[wasm_bindgen]
pub fn dilithium_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, JsValue> {
    if public_key.len() != ml_dsa_65::PK_LEN {
        return Err(format!("Invalid public key length. Expected {}, got {}", ml_dsa_65::PK_LEN, public_key.len()).into());
    }
    if signature.len() != ml_dsa_65::SIG_LEN {
        return Err(format!("Invalid signature length. Expected {}, got {}", ml_dsa_65::SIG_LEN, signature.len()).into());
    }
    let pk_array: [u8; ml_dsa_65::PK_LEN] = public_key.try_into()
        .map_err(|_| "Failed to convert public key to array")?;
    let pk = ml_dsa_65::PublicKey::try_from_bytes(pk_array)
        .map_err(|e| format!("Invalid public key: {}", e))?;
    let sig_array: [u8; ml_dsa_65::SIG_LEN] = signature.try_into()
        .map_err(|_| "Failed to convert signature to array")?;
    Ok(pk.verify(message, &sig_array, &[]))
}