use pqcrypto_traits::kem::SharedSecret;

#[test]
fn mceliece_direct() {
    use pqcrypto_classicmceliece::mceliece348864::{keypair, encapsulate, decapsulate};

    let (pk, sk) = keypair();
    let (ss, ct) = encapsulate(&pk);
    let ss2 = decapsulate(&ct, &sk);

    assert_eq!(ss.as_bytes(), ss2.as_bytes());
}
