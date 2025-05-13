use pyo3::prelude::*;
use pqcrypto_falcon::{falcon512, falcon1024};
use pqcrypto_traits::sign::{SecretKey, PublicKey, DetachedSignature, SignedMessage};

#[pyclass]
struct Falcon512 {}

#[pyclass]
struct Falcon1024 {}

#[pymethods]
impl Falcon512 {
    #[staticmethod]
    fn generate_keypair(_py: Python) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (public_key, secret_key) = falcon512::keypair();
        Ok((public_key.as_bytes().to_vec(), secret_key.as_bytes().to_vec()))
    }

    #[staticmethod]
    fn detached_sign(_py: Python, secret_key: &[u8], message: &[u8]) -> PyResult<Vec<u8>> {
        let secret_key = falcon512::SecretKey::from_bytes(secret_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SecretKey: {}", e)))?;

        let signature = falcon512::detached_sign(message, &secret_key);
        Ok(signature.as_bytes().to_vec())
    }

    #[staticmethod]
    fn verify_sign(_py: Python, signed_message: &[u8], public_key: &[u8]) -> PyResult<Vec<u8>> {
        let public_key = falcon512::PublicKey::from_bytes(public_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid PublicKey: {}", e)))?;

        let signed_message = falcon512::SignedMessage::from_bytes(signed_message)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SignedMessage: {}", e)))?;

        falcon512::open(&signed_message, &public_key)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Verification failed"))
    }

    #[staticmethod]
    fn sign_message(_py: Python, message: &[u8], secret_key: &[u8]) -> PyResult<Vec<u8>> {
        let secret_key = falcon512::SecretKey::from_bytes(secret_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SecretKey: {}", e)))?;

        let signed_message = falcon512::sign(message, &secret_key);
        Ok(signed_message.as_bytes().to_vec())
    }

    #[staticmethod]
    fn verify_detached_sign(signature: &[u8], message: &[u8], public_key: &[u8]) -> PyResult<bool> {
        let signature = falcon512::DetachedSignature::from_bytes(signature)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid DetachedSignature: {}", e)))?;

        let public_key = falcon512::PublicKey::from_bytes(public_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid PublicKey: {}", e)))?;

        falcon512::verify_detached_signature(&signature, message, &public_key)
            .map(|_| true)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Detached verification failed"))
    }
}

#[pymethods]
impl Falcon1024 {
    #[staticmethod]
    fn generate_keypair(_py: Python) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (public_key, secret_key) = falcon1024::keypair();
        Ok((public_key.as_bytes().to_vec(), secret_key.as_bytes().to_vec()))
    }

    #[staticmethod]
    fn detached_sign(_py: Python, secret_key: &[u8], message: &[u8]) -> PyResult<Vec<u8>> {
        let secret_key = falcon1024::SecretKey::from_bytes(secret_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SecretKey: {}", e)))?;

        let signature = falcon1024::detached_sign(message, &secret_key);
        Ok(signature.as_bytes().to_vec())
    }

    #[staticmethod]
    fn verify_sign(_py: Python, signed_message: &[u8], public_key: &[u8]) -> PyResult<Vec<u8>> {
        let public_key = falcon1024::PublicKey::from_bytes(public_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid PublicKey: {}", e)))?;

        let signed_message = falcon1024::SignedMessage::from_bytes(signed_message)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SignedMessage: {}", e)))?;

        falcon1024::open(&signed_message, &public_key)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Verification failed"))
    }

    #[staticmethod]
    fn sign_message(_py: Python, message: &[u8], secret_key: &[u8]) -> PyResult<Vec<u8>> {
        let secret_key = falcon1024::SecretKey::from_bytes(secret_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid SecretKey: {}", e)))?;

        let signed_message = falcon1024::sign(message, &secret_key);
        Ok(signed_message.as_bytes().to_vec())
    }

    #[staticmethod]
    fn verify_detached_sign(signature: &[u8], message: &[u8], public_key: &[u8]) -> PyResult<bool> {
        let signature = falcon1024::DetachedSignature::from_bytes(signature)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid DetachedSignature: {}", e)))?;

        let public_key = falcon1024::PublicKey::from_bytes(public_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid PublicKey: {}", e)))?;

        falcon1024::verify_detached_signature(&signature, message, &public_key)
            .map(|_| true)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Detached verification failed"))
    }
}

#[pymodule]
fn falcon_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Falcon512>()?;
    m.add_class::<Falcon1024>()?;
    Ok(())
}