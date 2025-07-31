//! Utility functions for SPHINCS+ operations.

use pqc_sphincsplus::{CRYPTO_PUBLICKEYBYTES as PK_LEN, CRYPTO_SECRETKEYBYTES as SK_LEN, CRYPTO_BYTES as SIG_LEN};

/// Returns the expected public key length for SPHINCS+.
pub fn public_key_length() -> usize {
    PK_LEN
}

/// Returns the expected secret key length for SPHINCS+.
pub fn secret_key_length() -> usize {
    SK_LEN
}

/// Returns the expected signature length for SPHINCS+.
pub fn signature_length() -> usize {
    SIG_LEN
}

/// Validates that a public key has the correct length.
pub fn validate_public_key_length(key: &[u8]) -> Result<(), String> {
    if key.len() != PK_LEN {
        Err(format!("Invalid public key length. Expected {}, got {}", PK_LEN, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a secret key has the correct length.
pub fn validate_secret_key_length(key: &[u8]) -> Result<(), String> {
    if key.len() != SK_LEN {
        Err(format!("Invalid secret key length. Expected {}, got {}", SK_LEN, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a signature has the correct length.
pub fn validate_signature_length(signature: &[u8]) -> Result<(), String> {
    if signature.len() != SIG_LEN {
        Err(format!("Invalid signature length. Expected {}, got {}", SIG_LEN, signature.len()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_constants() {
        assert_eq!(public_key_length(), PK_LEN);
        assert_eq!(secret_key_length(), SK_LEN);
        assert_eq!(signature_length(), SIG_LEN);
    }

    #[test]
    fn test_validation_functions() {
        let valid_pk = vec![0u8; PK_LEN];
        let invalid_pk = vec![0u8; PK_LEN + 1];
        
        assert!(validate_public_key_length(&valid_pk).is_ok());
        assert!(validate_public_key_length(&invalid_pk).is_err());

        let valid_sk = vec![0u8; SK_LEN];
        let invalid_sk = vec![0u8; SK_LEN - 1];
        
        assert!(validate_secret_key_length(&valid_sk).is_ok());
        assert!(validate_secret_key_length(&invalid_sk).is_err());

        let valid_sig = vec![0u8; SIG_LEN];
        let invalid_sig = vec![0u8; SIG_LEN + 10];
        
        assert!(validate_signature_length(&valid_sig).is_ok());
        assert!(validate_signature_length(&invalid_sig).is_err());
    }
}

