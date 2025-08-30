//! Comprehensive Known Answer Tests (KAT) validation.
//! This file validates that all KAT files exist and have the correct number of test vectors.

use std::fs;

/// Count the number of test vectors in a KAT file.
fn count_kat_vectors(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path).expect("Failed to read KAT file");
    let mut line_count = 0;

    for line in content.lines() {
        let line = line.trim();
        if !line.is_empty() && line.chars().all(|c| c.is_ascii_hexdigit()) {
            line_count += 1;
        }
    }

    // For KEM algorithms: 5 lines per test vector (pk, sk, ct, ss, ss_verify)
    // For signature algorithms: 4 lines per test vector (pk, sk, signed_message, signature)
    if file_path.contains("ml-kem") || file_path.contains("hqc") || file_path.contains("mceliece") {
        line_count / 5
    } else {
        line_count / 4
    }
}

// ML-KEM (Kyber) KAT Tests
#[test]
fn test_ml_kem_512_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-kem-512_clean.kat");
    assert_eq!(vector_count, 100, "ML-KEM-512 should have 100 test vectors");
}

#[test]
fn test_ml_kem_768_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-kem-768_clean.kat");
    assert_eq!(vector_count, 100, "ML-KEM-768 should have 100 test vectors");
}

#[test]
fn test_ml_kem_1024_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-kem-1024_clean.kat");
    assert_eq!(vector_count, 100, "ML-KEM-1024 should have 100 test vectors");
}

// ML-DSA (Dilithium) KAT Tests
#[test]
fn test_ml_dsa_44_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-dsa-44_clean.kat");
    assert_eq!(vector_count, 100, "ML-DSA-44 should have 100 test vectors");
}

#[test]
fn test_ml_dsa_65_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-dsa-65_clean.kat");
    assert_eq!(vector_count, 100, "ML-DSA-65 should have 100 test vectors");
}

#[test]
fn test_ml_dsa_87_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/ml-dsa-87_clean.kat");
    assert_eq!(vector_count, 100, "ML-DSA-87 should have 100 test vectors");
}

// Falcon KAT Tests
#[test]
fn test_falcon_512_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/falcon-512_clean.kat");
    assert_eq!(vector_count, 100, "Falcon-512 should have 100 test vectors");
}

#[test]
fn test_falcon_1024_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/falcon-1024_clean.kat");
    assert_eq!(vector_count, 100, "Falcon-1024 should have 100 test vectors");
}

// SPHINCS+ KAT Tests
#[test]
fn test_sphincs_sha2_128f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-sha2-128f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-128f should have 100 test vectors");
}

#[test]
fn test_sphincs_sha2_192f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-sha2-192f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-192f should have 100 test vectors");
}

#[test]
fn test_sphincs_sha2_256f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-sha2-256f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-256f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_128f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-shake-128f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-128f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_192f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-shake-192f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-192f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_256f_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/sphincs-shake-256f-simple_clean.kat");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-256f should have 100 test vectors");
}

// HQC KAT Tests
#[test]
fn test_hqc_128_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/hqc-128_clean.kat");
    assert_eq!(vector_count, 100, "HQC-128 should have 100 test vectors");
}

#[test]
fn test_hqc_192_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/hqc-192_clean.kat");
    assert_eq!(vector_count, 100, "HQC-192 should have 100 test vectors");
}

#[test]
fn test_hqc_256_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/hqc-256_clean.kat");
    assert_eq!(vector_count, 100, "HQC-256 should have 100 test vectors");
}

// Classic McEliece KAT Tests
#[test]
fn test_mceliece_348864_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/mceliece348864_clean.kat");
    assert_eq!(vector_count, 100, "McEliece-348864 should have 100 test vectors");
}

#[test]
fn test_mceliece_460896_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/mceliece460896_clean.kat");
    assert_eq!(vector_count, 100, "McEliece-460896 should have 100 test vectors");
}

#[test]
fn test_mceliece_6688128_kat_file() {
    let vector_count = count_kat_vectors("../kat_files/mceliece6688128_clean.kat");
    assert_eq!(vector_count, 100, "McEliece-6688128 should have 100 test vectors");
}

// Summary test to verify all KAT files exist and have correct counts
#[test]
fn test_all_kat_files_summary() {
    let kat_files = vec![
        "../kat_files/ml-kem-512_clean.kat",
        "../kat_files/ml-kem-768_clean.kat",
        "../kat_files/ml-kem-1024_clean.kat",
        "../kat_files/ml-dsa-44_clean.kat",
        "../kat_files/ml-dsa-65_clean.kat",
        "../kat_files/ml-dsa-87_clean.kat",
        "../kat_files/falcon-512_clean.kat",
        "../kat_files/falcon-1024_clean.kat",
        "../kat_files/sphincs-sha2-128f-simple_clean.kat",
        "../kat_files/sphincs-sha2-192f-simple_clean.kat",
        "../kat_files/sphincs-sha2-256f-simple_clean.kat",
        "../kat_files/sphincs-shake-128f-simple_clean.kat",
        "../kat_files/sphincs-shake-192f-simple_clean.kat",
        "../kat_files/sphincs-shake-256f-simple_clean.kat",
        "../kat_files/hqc-128_clean.kat",
        "../kat_files/hqc-192_clean.kat",
        "../kat_files/hqc-256_clean.kat",
        "../kat_files/mceliece348864_clean.kat",
        "../kat_files/mceliece460896_clean.kat",
        "../kat_files/mceliece6688128_clean.kat"
    ];

    let mut total_vectors = 0;
    for file_path in &kat_files {
        let vector_count = count_kat_vectors(file_path);
        assert_eq!(vector_count, 100, "{} should have 100 test vectors", file_path);
        total_vectors += vector_count;
    }

    // Total: 20 algorithms × 100 vectors each = 2000 test vectors
    assert_eq!(total_vectors, 2000, "Total KAT vectors should be 2000");
    println!(
        "✅ All {} KAT files validated with {} total test vectors",
        kat_files.len(),
        total_vectors
    );
}
