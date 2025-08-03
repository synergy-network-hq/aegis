// src/python_bindings.rs
//! Python bindings for Aegis Crypto Core — all NIST PQC algorithms & utilities.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::{
    sphincsplus, hqc, classicmceliece,
    dilithium, kyber, falcon,
    hash, utils,
};

//
// SPHINCS+
//
#[pyclass]
pub struct SphincsPlus192fKeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl SphincsPlus192fKeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<sphincsplus::SphincsPlus192fKeyPair> for SphincsPlus192fKeyPair {
    fn from(k: sphincsplus::SphincsPlus192fKeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

#[pyclass]
pub struct SphincsPlus256fKeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl SphincsPlus256fKeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<sphincsplus::SphincsPlus256fKeyPair> for SphincsPlus256fKeyPair {
    fn from(k: sphincsplus::SphincsPlus256fKeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

#[pyclass]
pub struct SphincsPlus;
#[pymethods]
impl SphincsPlus {
    #[staticmethod] fn keygen_192() -> SphincsPlus192fKeyPair { sphincsplus::sphincsplus192_keygen().into() }
    #[staticmethod] fn sign_192(secret_key: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
        sphincsplus::sphincsplus192_sign(&secret_key, &msg)
    }
    #[staticmethod] fn verify_192(public_key: Vec<u8>, msg: Vec<u8>) -> bool {
        sphincsplus::sphincsplus192_verify(&public_key, &msg)
    }
    #[staticmethod] fn keygen_256() -> SphincsPlus256fKeyPair { sphincsplus::sphincsplus256_keygen().into() }
    #[staticmethod] fn sign_256(secret_key: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
        sphincsplus::sphincsplus256_sign(&secret_key, &msg)
    }
    #[staticmethod] fn verify_256(public_key: Vec<u8>, msg: Vec<u8>) -> bool {
        sphincsplus::sphincsplus256_verify(&public_key, &msg)
    }
}

//
// HQC
//
#[pyclass]
pub struct Hqc128KeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl Hqc128KeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<hqc::Hqc128KeyPair> for Hqc128KeyPair {
    fn from(k: hqc::Hqc128KeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

// (… duplicate for 192, 256 …)

#[pyclass]
pub struct HQC;
#[pymethods]
impl HQC {
    #[staticmethod] fn keygen_128() -> Hqc128KeyPair { hqc::hqc128_keygen().into() }
    #[staticmethod] fn encapsulate_128(pk: Vec<u8>) -> PyResult<(Vec<u8>,Vec<u8>)> {
        let out = hqc::hqc128_encapsulate(&pk)
            .map_err(|e| PyValueError::new_err(format!("HQC-128 failed: {}", e)))?;
        Ok((out.ciphertext(), out.shared_secret()))
    }
    #[staticmethod] fn decapsulate_128(sk: Vec<u8>, ct: Vec<u8>) -> PyResult<Vec<u8>> {
        hqc::hqc128_decapsulate(&sk, &ct)
            .map_err(|e| PyValueError::new_err(format!("HQC-128 failed: {}", e)))
    }
    // … keygen_192, encapsulate_192, … keygen_256, etc.
}

//
// Classic McEliece
//
#[pyclass]
pub struct ClassicMcEliece128KeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl ClassicMcEliece128KeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<classicmceliece::ClassicMcEliece128KeyPair> for ClassicMcEliece128KeyPair {
    fn from(k: classicmceliece::ClassicMcEliece128KeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

// (… 192, 256 …)

#[pyclass]
pub struct ClassicMcEliece;
#[pymethods]
impl ClassicMcEliece {
    #[staticmethod] fn keygen_128() -> ClassicMcEliece128KeyPair { classicmceliece::classicmceliece128_keygen().into() }
    #[staticmethod] fn encapsulate_128(pk: Vec<u8>) -> PyResult<(Vec<u8>,Vec<u8>)> {
        let out = classicmceliece::classicmceliece128_encapsulate(&pk)
            .map_err(|e| PyValueError::new_err(format!("McEliece-128 failed: {}", e)))?;
        Ok((out.ciphertext(), out.shared_secret()))
    }
    #[staticmethod] fn decapsulate_128(sk: Vec<u8>, ct: Vec<u8>) -> PyResult<Vec<u8>> {
        classicmceliece::classicmceliece128_decapsulate(&sk, &ct)
            .map_err(|e| PyValueError::new_err(format!("McEliece-128 failed: {}", e)))
    }
    // … 192, 256 …
}

//
// Dilithium3
//
#[pyclass]
pub struct Dilithium3KeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl Dilithium3KeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<dilithium::DilithiumKeyPair> for Dilithium3KeyPair {
    fn from(k: dilithium::DilithiumKeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

#[pyclass]
pub struct Dilithium;
#[pymethods]
impl Dilithium {
    #[staticmethod] fn keygen() -> Dilithium3KeyPair { dilithium::dilithium_keygen().into() }
    #[staticmethod] fn sign(sk: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
        dilithium::dilithium_sign(&sk, &msg)
    }
    #[staticmethod] fn verify(pk: Vec<u8>, msg: Vec<u8>, sig: Vec<u8>) -> bool {
        dilithium::dilithium_verify(&pk, &msg, &sig)
    }
}

//
// Kyber
//
#[pyclass]
pub struct KyberKeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl KyberKeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<kyber::KyberKeyPair> for KyberKeyPair {
    fn from(k: kyber::KyberKeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

#[pyclass]
pub struct Kyber;
#[pymethods]
impl Kyber {
    #[staticmethod] fn keygen() -> KyberKeyPair { kyber::kyber_keygen().into() }
    #[staticmethod] fn encapsulate(pk: Vec<u8>) -> PyResult<(Vec<u8>,Vec<u8>)> {
        let out = kyber::kyber_encapsulate(&pk)
            .map_err(|e| PyValueError::new_err(format!("Kyber failed: {}", e)))?;
        Ok((out.ciphertext(), out.shared_secret()))
    }
    #[staticmethod] fn decapsulate(sk: Vec<u8>, ct: Vec<u8>) -> PyResult<Vec<u8>> {
        kyber::kyber_decapsulate(&sk, &ct)
            .map_err(|e| PyValueError::new_err(format!("Kyber failed: {}", e)))
    }
}

//
// Falcon
//
#[pyclass]
pub struct FalconKeyPair { pk: Vec<u8>, sk: Vec<u8> }
#[pymethods]
impl FalconKeyPair {
    #[getter] fn public_key(&self) -> PyResult<Vec<u8>> { Ok(self.pk.clone()) }
    #[getter] fn secret_key(&self) -> PyResult<Vec<u8>> { Ok(self.sk.clone()) }
}
impl From<falcon::FalconKeyPair> for FalconKeyPair {
    fn from(k: falcon::FalconKeyPair) -> Self {
        Self { pk: k.public_key(), sk: k.secret_key() }
    }
}

#[pyclass]
pub struct Falcon;
#[pymethods]
impl Falcon {
    #[staticmethod] fn keygen() -> FalconKeyPair { falcon::falcon_keygen().into() }
    #[staticmethod] fn sign(sk: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
        falcon::falcon_sign(&sk, &msg)
    }
    #[staticmethod] fn verify(pk: Vec<u8>, msg: Vec<u8>, sig: Vec<u8>) -> bool {
        falcon::falcon_verify(&pk, &msg, &sig)
    }
}

//
// Utilities & Hash
//
#[pyfunction]
fn sha3_256_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::sha3_256_hash(&data))
}
#[pyfunction]
fn sha3_256_hash_hex_py(data: Vec<u8>) -> PyResult<String> {
    Ok(hex::encode(hash::sha3_256_hash(&data)))
}
#[pyfunction]
fn sha3_256_hash_base64_py(data: Vec<u8>) -> PyResult<String> {
    Ok(base64::encode(hash::sha3_256_hash(&data)))
}
#[pyfunction]
fn sha3_512_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::sha3_512_hash(&data))
}
#[pyfunction]
fn sha3_512_hash_hex_py(data: Vec<u8>) -> PyResult<String> {
    Ok(hex::encode(hash::sha3_512_hash(&data)))
}
#[pyfunction]
fn sha3_512_hash_base64_py(data: Vec<u8>) -> PyResult<String> {
    Ok(base64::encode(hash::sha3_512_hash(&data)))
}
#[pyfunction]
fn blake3_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::blake3_hash(&data))
}
#[pyfunction]
fn blake3_hash_hex_py(data: Vec<u8>) -> PyResult<String> {
    Ok(hex::encode(hash::blake3_hash(&data)))
}
#[pyfunction]
fn blake3_hash_base64_py(data: Vec<u8>) -> PyResult<String> {
    Ok(base64::encode(hash::blake3_hash(&data)))
}
#[pyfunction]
fn hex_to_bytes_py(hex_string: String) -> PyResult<Vec<u8>> {
    utils::hex_to_bytes(&hex_string).map_err(|e| PyValueError::new_err(format!("Invalid hex: {}", e)))
}
#[pyfunction]
fn bytes_to_hex_py(data: Vec<u8>) -> PyResult<String> {
    Ok(utils::bytes_to_hex(&data))
}

#[pymodule]
fn aegis_crypto_core_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // Algorithm classes
    m.add_class::<SphincsPlus>()?;
    m.add_class::<SphincsPlus192fKeyPair>()?;
    m.add_class::<SphincsPlus256fKeyPair>()?;
    m.add_class::<HQC>()?;
    m.add_class::<Hqc128KeyPair>()?;
    // … add the rest: Hqc192KeyPair, Hqc256KeyPair, ClassicMcEliece, ClassicMcEliece128KeyPair, …
    m.add_class::<Dilithium>()?;
    m.add_class::<Dilithium3KeyPair>()?;
    m.add_class::<Kyber>()?;
    m.add_class::<KyberKeyPair>()?;
    m.add_class::<Falcon>()?;
    m.add_class::<FalconKeyPair>()?;

    // Utility / Hash functions
    m.add_function(wrap_pyfunction!(sha3_256_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_256_hash_hex_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_256_hash_base64_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_512_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_512_hash_hex_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_512_hash_base64_py, m)?)?;
    m.add_function(wrap_pyfunction!(blake3_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(blake3_hash_hex_py, m)?)?;
    m.add_function(wrap_pyfunction!(blake3_hash_base64_py, m)?)?;
    m.add_function(wrap_pyfunction!(hex_to_bytes_py, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_to_hex_py, m)?)?;

    Ok(())
}
