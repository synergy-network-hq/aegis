//! Python bindings for Aegis Crypto Core.
//!
//! This module exposes a minimal set of functions and classes via the
//! [PyO3](https://github.com/PyO3/pyo3) framework so that Python
//! applications can directly use the post‑quantum cryptographic
//! primitives implemented in Rust.  These bindings are meant to be
//! compiled as a Python extension module.  Each exported function
//! delegates to the corresponding Rust implementation in the parent
//! crate without introducing any placeholder code.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::{
    kyber::{self, KyberKeyPair, KyberEncapsulated},
    dilithium::{self, DilithiumKeyPair},
    falcon::{self, FalconKeyPair},
    hash,
    utils,
};

/// A Python class representing a Kyber key pair.
#[pyclass(name = "KyberKeyPair")]
struct KyberKeyPairPy {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[pymethods]
impl KyberKeyPairPy {
    #[getter]
    fn public_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.pk.clone())
    }
    #[getter]
    fn secret_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.sk.clone())
    }
}

impl From<KyberKeyPair> for KyberKeyPairPy {
    fn from(k: KyberKeyPair) -> Self {
        Self {
            pk: k.public_key(),
            sk: k.secret_key(),
        }
    }
}

/// A Python class representing a Dilithium key pair.
#[pyclass(name = "DilithiumKeyPair")]
struct DilithiumKeyPairPy {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[pymethods]
impl DilithiumKeyPairPy {
    #[getter]
    fn public_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.pk.clone())
    }
    #[getter]
    fn secret_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.sk.clone())
    }
}

impl From<DilithiumKeyPair> for DilithiumKeyPairPy {
    fn from(k: DilithiumKeyPair) -> Self {
        Self {
            pk: k.public_key(),
            sk: k.secret_key(),
        }
    }
}

/// A Python class representing a Falcon key pair.
#[pyclass(name = "FalconKeyPair")]
struct FalconKeyPairPy {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[pymethods]
impl FalconKeyPairPy {
    #[getter]
    fn public_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.pk.clone())
    }
    #[getter]
    fn secret_key(&self) -> PyResult<Vec<u8>> {
        Ok(self.sk.clone())
    }
}

impl From<FalconKeyPair> for FalconKeyPairPy {
    fn from(k: FalconKeyPair) -> Self {
        Self {
            pk: k.public_key(),
            sk: k.secret_key(),
        }
    }
}

/// Generates a Kyber key pair.
#[pyfunction]
fn kyber_keygen_py() -> PyResult<KyberKeyPairPy> {
    Ok(kyber::kyber_keygen().into())
}

/// Encapsulates a shared secret using Kyber.
#[pyfunction]
fn kyber_encapsulate_py(public_key: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
    match kyber::kyber_encapsulate(&public_key) {
        Ok(encaps) => Ok((encaps.ciphertext().to_vec(), encaps.shared_secret().to_vec())),
        Err(e) => Err(PyValueError::new_err(format!("Kyber encapsulation failed: {}", e)) ),
    }
}

/// Decapsulates a shared secret using Kyber.
#[pyfunction]
fn kyber_decapsulate_py(secret_key: Vec<u8>, ciphertext: Vec<u8>) -> PyResult<Vec<u8>> {
    match kyber::kyber_decapsulate(&secret_key, &ciphertext) {
        Ok(shared) => Ok(shared),
        Err(e) => Err(PyValueError::new_err(format!("Kyber decapsulation failed: {}", e)) ),
    }
}

/// Generates a Dilithium key pair.
#[pyfunction]
fn dilithium_keygen_py() -> PyResult<DilithiumKeyPairPy> {
    Ok(dilithium::dilithium_keygen().into())
}

/// Signs a message using Dilithium.
#[pyfunction]
fn dilithium_sign_py(secret_key: Vec<u8>, message: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(dilithium::dilithium_sign(&secret_key, &message))
}

/// Verifies a Dilithium signature.
#[pyfunction]
fn dilithium_verify_py(public_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> PyResult<bool> {
    Ok(dilithium::dilithium_verify(&public_key, &message, &signature))
}

/// Generates a Falcon key pair.
#[pyfunction]
fn falcon_keygen_py() -> PyResult<FalconKeyPairPy> {
    Ok(falcon::falcon_keygen().into())
}

/// Signs a message using Falcon.
#[pyfunction]
fn falcon_sign_py(secret_key: Vec<u8>, message: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(falcon::falcon_sign(&secret_key, &message))
}

/// Verifies a Falcon signature.
#[pyfunction]
fn falcon_verify_py(public_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> PyResult<bool> {
    Ok(falcon::falcon_verify(&public_key, &message, &signature))
}

/// Computes the SHA3‑256 hash of the given data.
#[pyfunction]
fn sha3_256_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::sha3_256_hash(&data))
}

/// Computes the SHA3‑512 hash of the given data.
#[pyfunction]
fn sha3_512_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::sha3_512_hash(&data))
}

/// Computes the BLAKE3 hash of the given data.
#[pyfunction]
fn blake3_hash_py(data: Vec<u8>) -> PyResult<Vec<u8>> {
    Ok(hash::blake3_hash(&data))
}

/// Convert a hex string to bytes.
#[pyfunction]
fn hex_to_bytes_py(hex_string: String) -> PyResult<Vec<u8>> {
    match utils::hex_to_bytes(&hex_string) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(PyValueError::new_err(format!("Invalid hex string: {}", e)) ),
    }
}

/// Convert bytes to a hex string.
#[pyfunction]
fn bytes_to_hex_py(data: Vec<u8>) -> PyResult<String> {
    Ok(utils::bytes_to_hex(&data))
}

/// Defines the Python module.  This function is called by the Python
/// interpreter when the module is imported.  All functions and classes
/// are added to the module here.
#[pymodule]
fn aegis_crypto_core_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KyberKeyPairPy>()?;
    m.add_class::<DilithiumKeyPairPy>()?;
    m.add_class::<FalconKeyPairPy>()?;
    m.add_function(wrap_pyfunction!(kyber_keygen_py, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_encapsulate_py, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_decapsulate_py, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_keygen_py, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_sign_py, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_verify_py, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_keygen_py, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_sign_py, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_verify_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_256_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_512_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(blake3_hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(hex_to_bytes_py, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_to_hex_py, m)?)?;
    Ok(())
}