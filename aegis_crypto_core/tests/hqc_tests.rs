//! Integration tests for the HQC key encapsulation mechanism (KEM).

use aegis_crypto_core::{hqc_keygen, hqc_encapsulate, hqc_decapsulate};

#[test]
fn test_hqc_encaps_and_decaps() {
    // Add some debug output
    println!("Starting HQC test...");

    // Generate a recipient key pair
    let keypair = hqc_keygen();
    println!("Generated keypair");

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    println!("Public key length: {}, Secret key length: {}", public_key.len(), secret_key.len());

    // Encapsulate a shared secret
    let encapsulated_result = hqc_encapsulate(&public_key);

    match encapsulated_result {
        Ok(encapsulated) => {
            println!("Encapsulation successful");
            let ciphertext = encapsulated.ciphertext();
            let shared_secret_enc = encapsulated.shared_secret();
            println!("Ciphertext length: {}, Shared secret length: {}", ciphertext.len(), shared_secret_enc.len());

            // Decapsulate the shared secret
            let shared_secret_dec_result = hqc_decapsulate(&secret_key, &ciphertext);

            match shared_secret_dec_result {
                Ok(shared_secret_dec) => {
                    println!("Decapsulation successful");
                    assert_eq!(shared_secret_enc, shared_secret_dec, "Shared secrets should match");
                    println!("Shared secrets match - test passed!");
                }
                Err(e) => {
                    panic!("Decapsulation failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            panic!("Encapsulation failed: {:?}", e);
        }
    }
}
