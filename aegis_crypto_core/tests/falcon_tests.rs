use aegis_crypto_core::{falcon_keygen, falcon_sign, falcon_verify};

#[test]
fn test_falcon_sign_and_verify() {
    let keypair = falcon_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    let message = b"Falcon signatures for quantum safety";

    let signature = falcon_sign(&secret_key, message); // signature (not signed_message)
    assert!(
        falcon_verify(&public_key, message, &signature),
        "Falcon signature should be valid"
    );

    // Tamper with signature
    let mut bad_sig = signature.clone();
    bad_sig[0] ^= 0xFF;
    assert!(
        !falcon_verify(&public_key, message, &bad_sig),
        "Verification should fail for a tampered signature"
    );
}
