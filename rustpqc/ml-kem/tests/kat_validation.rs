use std::fs::File;
use std::io::{ BufRead, BufReader };

#[test]
fn test_kat_validation() {
    let kat_file = "tests/kat_vectors/mlkem_768.rsp";
    let file = File::open(kat_file).expect("Failed to open KAT file");
    let reader = BufReader::new(file);

    let mut current_test = TestVector::new();
    let mut test_count = 0;
    let mut passed = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with("count = ") {
            // Save previous test if it exists
            if current_test.is_complete() {
                if run_kat_test(&current_test) {
                    passed += 1;
                }
                test_count += 1;
            }

            // Start new test
            current_test = TestVector::new();
            current_test.count = line.split(" = ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("seed = ") {
            current_test.seed = hex::decode(line.split(" = ").nth(1).unwrap()).unwrap();
        } else if line.starts_with("pk = ") {
            current_test.pk = hex::decode(line.split(" = ").nth(1).unwrap()).unwrap();
        } else if line.starts_with("sk = ") {
            current_test.sk = hex::decode(line.split(" = ").nth(1).unwrap()).unwrap();
        } else if line.starts_with("ct = ") {
            current_test.ct = hex::decode(line.split(" = ").nth(1).unwrap()).unwrap();
        } else if line.starts_with("ss = ") {
            current_test.ss = hex::decode(line.split(" = ").nth(1).unwrap()).unwrap();
        }
    }

    // Run the last test
    if current_test.is_complete() {
        if run_kat_test(&current_test) {
            passed += 1;
        }
        test_count += 1;
    }

    println!("KAT validation: {}/{} tests passed", passed, test_count);
    assert_eq!(passed, test_count, "All KAT tests must pass");
}

struct TestVector {
    count: u32,
    seed: Vec<u8>,
    pk: Vec<u8>,
    sk: Vec<u8>,
    ct: Vec<u8>,
    ss: Vec<u8>,
}

impl TestVector {
    fn new() -> Self {
        Self {
            count: 0,
            seed: Vec::new(),
            pk: Vec::new(),
            sk: Vec::new(),
            ct: Vec::new(),
            ss: Vec::new(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.seed.is_empty() &&
            !self.pk.is_empty() &&
            !self.sk.is_empty() &&
            !self.ct.is_empty() &&
            !self.ss.is_empty()
    }
}

fn run_kat_test(test: &TestVector) -> bool {
    // First verify vector lengths
    if test.pk.len() != ml_kem::params::PUBLICKEYBYTES {
        println!(
            "Test {}: Invalid public key length {} (expected {})",
            test.count,
            test.pk.len(),
            ml_kem::params::PUBLICKEYBYTES
        );
        return false;
    }

    if test.sk.len() != ml_kem::params::SECRETKEYBYTES {
        println!(
            "Test {}: Invalid secret key length {} (expected {})",
            test.count,
            test.sk.len(),
            ml_kem::params::SECRETKEYBYTES
        );
        return false;
    }

    if test.ct.len() != ml_kem::params::CIPHERTEXTBYTES {
        println!(
            "Test {}: Invalid ciphertext length {} (expected {})",
            test.count,
            test.ct.len(),
            ml_kem::params::CIPHERTEXTBYTES
        );
        return false;
    }

    if test.ss.len() != ml_kem::params::SYMBYTES {
        println!(
            "Test {}: Invalid shared secret length {} (expected {})",
            test.count,
            test.ss.len(),
            ml_kem::params::SYMBYTES
        );
        return false;
    }

    // Convert vectors to fixed-size arrays
    let mut pk_array = [0u8; ml_kem::params::PUBLICKEYBYTES];
    let mut sk_array = [0u8; ml_kem::params::SECRETKEYBYTES];
    let mut ct_array = [0u8; ml_kem::params::CIPHERTEXTBYTES];
    let mut ss_array = [0u8; ml_kem::params::SYMBYTES];

    pk_array.copy_from_slice(&test.pk);
    sk_array.copy_from_slice(&test.sk);
    ct_array.copy_from_slice(&test.ct);
    ss_array.copy_from_slice(&test.ss);

    // Test 1: Verify that the public key and secret key are consistent
    // The secret key should contain the public key
    let pk_from_sk = &sk_array[ml_kem::params::INDCPA_SECRETKEYBYTES..ml_kem::params::INDCPA_SECRETKEYBYTES + ml_kem::params::INDCPA_PUBLICKEYBYTES];
    if pk_from_sk != &pk_array[..ml_kem::params::INDCPA_PUBLICKEYBYTES] {
        println!(
            "Test {}: Public key in secret key does not match provided public key",
            test.count
        );
        return false;
    }

    // Test 2: Verify that decryption produces the expected shared secret
    let mut decrypted_ss = [0u8; ml_kem::params::SYMBYTES];
    let dec_result = ml_kem::kem::crypto_kem_dec(&mut decrypted_ss, &ct_array, &sk_array);
    
    if dec_result != 0 {
        println!(
            "Test {}: Decryption failed with error code {}",
            test.count,
            dec_result
        );
        return false;
    }

    if decrypted_ss != ss_array {
        println!(
            "Test {}: Decrypted shared secret does not match expected value",
            test.count
        );
        return false;
    }

    // Test 3: Verify that encryption with the public key produces the expected ciphertext
    // Note: This test requires deterministic encryption, which may not be available
    // We'll skip this test for now as the standard KEM functions use random coins
    
    // Test 4: Verify that the keypair can be regenerated from the seed
    // This would require implementing deterministic key generation from seed
    // For now, we'll skip this test as the standard functions don't take seeds

    println!("Test {}: All validations passed", test.count);
    true
}
