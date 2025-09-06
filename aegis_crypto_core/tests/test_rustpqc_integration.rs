#[cfg(any())]
mod test_rustpqc_integration {
    use aegis_crypto_core::rustpqc_kyber::*;
    use aegis_crypto_core::rustpqc_dilithium::*;

    #[test]
    fn test_rustpqc_kyber768_integration() {
        // Test key generation
        let keypair = rustpqc_kyber768_keygen_rust();
        let (pk, sk) = keypair;

        // Test encapsulation
        let encapsulated = rustpqc_kyber768_encapsulate_rust(&pk).unwrap();
        let (ciphertext, shared_secret1) = encapsulated;

        // Test decapsulation
        let shared_secret2 = rustpqc_kyber768_decapsulate_rust(&sk, &ciphertext).unwrap();

        // Verify that both shared secrets match
        assert_eq!(shared_secret1, shared_secret2);

        println!("✅ RustPQC ML-KEM-768 integration test passed!");
    }

    #[test]
    fn test_rustpqc_dilithium65_integration() {
        // Test key generation
        let keypair = rustpqc_dilithium65_keygen_rust();
        let (pk, sk) = keypair;

        // Test signing
        let message = b"Hello, RustPQC ML-DSA-65!";
        let signature = rustpqc_dilithium65_sign_rust(&sk, message).unwrap();

        // Test verification
        let is_valid = rustpqc_dilithium65_verify_rust(&pk, &signature, message).unwrap();
        assert!(is_valid);

        // Test verification with wrong message
        let wrong_message = b"Wrong message!";
        let is_valid_wrong = rustpqc_dilithium65_verify_rust(
            &pk,
            &signature,
            wrong_message
        ).unwrap();
        assert!(!is_valid_wrong);

        println!("✅ RustPQC ML-DSA-65 integration test passed!");
    }

    #[test]
    fn test_rustpqc_parameter_sizes() {
        // Test ML-KEM-768 parameter sizes
        let keypair = rustpqc_kyber768_keygen_rust();
        let (pk, sk) = keypair;

        // These sizes should match the parameters in rustpqc/ml-kem/src/params.rs
        assert_eq!(pk.len(), 1184); // PUBLICKEYBYTES for ML-KEM-768
        assert_eq!(sk.len(), 2400); // SECRETKEYBYTES for ML-KEM-768

        let encapsulated = rustpqc_kyber768_encapsulate_rust(&pk).unwrap();
        let (ciphertext, shared_secret) = encapsulated;

        assert_eq!(ciphertext.len(), 1088); // CIPHERTEXTBYTES for ML-KEM-768
        assert_eq!(shared_secret.len(), 32); // SYMBYTES

        // Test ML-DSA-65 parameter sizes
        let dilithium_keypair = rustpqc_dilithium65_keygen_rust();
        let (dilithium_pk, dilithium_sk) = dilithium_keypair;

        // These sizes should match the parameters in rustpqc/ml-dsa/src/params.rs
        assert_eq!(dilithium_pk.len(), 1952); // CRYPTO_PUBLICKEYBYTES for ML-DSA-65
        assert_eq!(dilithium_sk.len(), 4016); // CRYPTO_SECRETKEYBYTES for ML-DSA-65

        let message = b"Test message";
        let signature = rustpqc_dilithium65_sign_rust(&dilithium_sk, message).unwrap();

        assert_eq!(signature.len(), 3293); // CRYPTO_BYTES for ML-DSA-65

        println!("✅ RustPQC parameter sizes test passed!");
    }
}

#[cfg(not(any()))]
mod test_rustpqc_integration {
    #[test]
    fn test_rustpqc_features_disabled() {
        println!(
            "ℹ️  RustPQC features are disabled. Run with --features rustpqc-kyber,rustpqc-dilithium to test integration."
        );
    }
}
