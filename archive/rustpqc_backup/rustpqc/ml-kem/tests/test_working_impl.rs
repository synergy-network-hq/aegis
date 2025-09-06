use ml_kem::params::*;
use ml_kem::kem::*;

#[test]
fn test_working_implementation() {
    println!("Testing ML-KEM implementation with current parameters:");
    println!("K: {}", ml_kem::params::K);
    println!("SECRETKEYBYTES: {}", ml_kem::params::SECRETKEYBYTES);
    println!("PUBLICKEYBYTES: {}", ml_kem::params::PUBLICKEYBYTES);
    println!("CIPHERTEXTBYTES: {}", ml_kem::params::CIPHERTEXTBYTES);
    println!("SYMBYTES: {}", ml_kem::params::SYMBYTES);

    // Generate keypair
    let mut pk = [0u8; ml_kem::params::PUBLICKEYBYTES];
    let mut sk = [0u8; ml_kem::params::SECRETKEYBYTES];

    let result = ml_kem::kem::crypto_kem_keypair(&mut pk, &mut sk);
    assert_eq!(result, 0, "Keypair generation failed");
    println!("✓ Keypair generation successful");

    // Encapsulate shared secret
    let mut ct = [0u8; ml_kem::params::CIPHERTEXTBYTES];
    let mut ss1 = [0u8; ml_kem::params::SYMBYTES];

    let result = ml_kem::kem::crypto_kem_enc(&mut ct, &mut ss1, &pk);
    assert_eq!(result, 0, "Encapsulation failed");
    println!("✓ Encapsulation successful");

    // Decapsulate shared secret
    let mut ss2 = [0u8; ml_kem::params::SYMBYTES];

    let result = ml_kem::kem::crypto_kem_dec(&mut ss2, &ct, &sk);
    assert_eq!(result, 0, "Decapsulation failed");
    println!("✓ Decapsulation successful");

    // Debug: Print the first few bytes of each shared secret
    println!("First 8 bytes of ss1: {:?}", &ss1[..8]);
    println!("First 8 bytes of ss2: {:?}", &ss2[..8]);

    // Debug: Print the last few bytes of each shared secret
    println!("Last 8 bytes of ss1: {:?}", &ss1[24..]);
    println!("Last 8 bytes of ss2: {:?}", &ss2[24..]);

    // Debug: Print the first few bytes of the ciphertext
    println!("First 8 bytes of ciphertext: {:?}", &ct[..8]);

    // Debug: Print the first few bytes of the public key
    println!("First 8 bytes of public key: {:?}", &pk[..8]);

    // Debug: Print the first few bytes of the secret key
    println!("First 8 bytes of secret key: {:?}", &sk[..8]);

    // Verify that the public key is correctly embedded in the secret key
    let pk_from_sk =
        &sk
            [
                ml_kem::params::INDCPA_SECRETKEYBYTES..ml_kem::params::INDCPA_SECRETKEYBYTES +
                    ml_kem::params::INDCPA_PUBLICKEYBYTES
            ];
    assert_eq!(
        pk_from_sk,
        &pk[..ml_kem::params::INDCPA_PUBLICKEYBYTES],
        "Public key not correctly embedded in secret key"
    );

    // Verify that the shared secrets match
    assert_eq!(ss1, ss2, "Shared secrets don't match");
    println!("✓ Shared secrets match");
}
