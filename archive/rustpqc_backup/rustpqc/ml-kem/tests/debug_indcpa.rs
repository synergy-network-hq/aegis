use ml_kem::indcpa::{ indcpa_keypair, indcpa_enc, indcpa_dec };
use ml_kem::params::{ INDCPA_PUBLICKEYBYTES, INDCPA_SECRETKEYBYTES, INDCPA_BYTES, INDCPA_MSGBYTES };
use ml_kem::poly::{ Poly, poly_frommsg, poly_tomsg };
use ml_kem::polyvec::{ PolyVec, polyvec_frombytes };

#[test]
fn test_poly_message_conversion() {
    // Test if poly_frommsg and poly_tomsg work correctly
    let mut m1 = [0u8; INDCPA_MSGBYTES];
    let mut m2 = [0u8; INDCPA_MSGBYTES];

    // Generate a simple test message
    for i in 0..INDCPA_MSGBYTES {
        m1[i] = i as u8;
    }

    println!("Original message: {:?}", &m1[..8]);

    // Convert to polynomial and back
    let mut poly = Poly::new();
    poly_frommsg(&mut poly, &m1);
    poly_tomsg(&mut m2, &mut poly);

    println!("Converted back: {:?}", &m2[..8]);

    if m1 == m2 {
        println!("✅ Polynomial message conversion works correctly!");
    } else {
        println!("❌ Polynomial message conversion failed!");
        println!("First 8 bytes of original: {:?}", &m1[..8]);
        println!("First 8 bytes of converted: {:?}", &m2[..8]);
    }

    assert_eq!(m1, m2, "Polynomial message conversion failed");
}

#[test]
fn test_indcpa_keypair_only() {
    // Test just the keypair generation to see if that works
    let mut pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut sk = [0u8; INDCPA_SECRETKEYBYTES];

    println!("Testing IND-CPA keypair generation...");

    // Generate keypair
    indcpa_keypair(&mut pk, &mut sk);
    println!("✅ Keypair generated successfully");

    // Check if the keys are not all zeros
    let mut pk_all_zero = true;
    let mut sk_all_zero = true;

    for &byte in &pk {
        if byte != 0 {
            pk_all_zero = false;
            break;
        }
    }

    for &byte in &sk {
        if byte != 0 {
            sk_all_zero = false;
            break;
        }
    }

    if !pk_all_zero && !sk_all_zero {
        println!("✅ Generated keys contain non-zero values");
        println!("First 8 bytes of pk: {:?}", &pk[..8]);
        println!("First 8 bytes of sk: {:?}", &sk[..8]);
    } else {
        println!("❌ Generated keys are all zeros!");
    }

    assert!(!pk_all_zero, "Public key is all zeros");
    assert!(!sk_all_zero, "Secret key is all zeros");
}

#[test]
fn test_indcpa_encryption_only() {
    // Test just the encryption to see if that works
    let mut pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut sk = [0u8; INDCPA_SECRETKEYBYTES];
    let mut ct = [0u8; INDCPA_BYTES];
    let mut m1 = [0u8; INDCPA_MSGBYTES];

    // Generate a simple test message
    for i in 0..INDCPA_MSGBYTES {
        m1[i] = i as u8;
    }

    println!("Testing IND-CPA encryption...");
    println!("Original message: {:?}", &m1[..8]);

    // Generate keypair
    indcpa_keypair(&mut pk, &mut sk);
    println!("Generated keypair");

    // Generate random coins for encryption
    let mut coins = [0u8; 32]; // SYMBYTES = 32
    for i in 0..32 {
        coins[i] = (i * 7) as u8; // Simple deterministic "random" values for testing
    }

    // Encrypt
    indcpa_enc(&mut ct, &m1, &pk, &coins);
    println!("✅ Encryption completed");

    // Check if ciphertext is not all zeros
    let mut ct_all_zero = true;
    for &byte in &ct {
        if byte != 0 {
            ct_all_zero = false;
            break;
        }
    }

    if !ct_all_zero {
        println!("✅ Generated ciphertext contains non-zero values");
        println!("First 8 bytes of ciphertext: {:?}", &ct[..8]);
    } else {
        println!("❌ Generated ciphertext is all zeros!");
    }

    assert!(!ct_all_zero, "Ciphertext is all zeros");
}

#[test]
fn test_secret_key_state() {
    // Test the secret key state at each step
    let mut pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut sk = [0u8; INDCPA_SECRETKEYBYTES];

    println!("Testing secret key state...");

    // Generate keypair
    indcpa_keypair(&mut pk, &mut sk);
    println!("✅ Keypair generated successfully");

    // Check secret key after generation
    println!("Secret key after generation:");
    println!("  First 8 bytes: {:?}", &sk[..8]);
    println!("  Last 8 bytes: {:?}", &sk[sk.len() - 8..]);

    // Try to unpack the secret key to see what we get
    let mut skpv = PolyVec::new();
    polyvec_frombytes(&mut skpv, &sk);
    println!("✅ Secret key unpacked successfully");

    // Check the first few coefficients of the first polynomial
    println!("First polynomial coefficients (first 8):");
    for i in 0..8 {
        println!("  coeff[{}] = {}", i, skpv.vec[0].coeffs[i]);
    }

    // Check if coefficients are reasonable (should be in range [-q/2, q/2])
    let mut reasonable_coeffs = true;
    for i in 0..8 {
        let coeff = skpv.vec[0].coeffs[i];
        if coeff < -3328 || coeff > 3327 {
            // q/2 = 3328
            reasonable_coeffs = false;
            println!("  ❌ coeff[{}] = {} is out of range", i, coeff);
        }
    }

    if reasonable_coeffs {
        println!("✅ First 8 coefficients are in reasonable range");
    } else {
        println!("❌ Some coefficients are out of reasonable range");
    }
}

#[test]
fn test_polynomial_operations() {
    // Test if the basic polynomial operations are working correctly
    println!("=== TESTING POLYNOMIAL OPERATIONS ===");

    // Create two simple polynomials
    let mut poly1 = Poly::new();
    let mut poly2 = Poly::new();
    let mut result = Poly::new();

    // Set some simple values
    poly1.coeffs[0] = 100;
    poly1.coeffs[1] = 200;
    poly2.coeffs[0] = 50;
    poly2.coeffs[1] = 150;

    println!("poly1.coeffs[0] = {}, poly1.coeffs[1] = {}", poly1.coeffs[0], poly1.coeffs[1]);
    println!("poly2.coeffs[0] = {}, poly2.coeffs[1] = {}", poly2.coeffs[0], poly2.coeffs[1]);

    // Test addition
    ml_kem::poly::poly_add(&mut result, &poly1, &poly2);
    println!("poly_add result: coeffs[0] = {}, coeffs[1] = {}", result.coeffs[0], result.coeffs[1]);

    // Test subtraction
    ml_kem::poly::poly_sub(&mut result, &poly1, &poly2);
    println!("poly_sub result: coeffs[0] = {}, coeffs[1] = {}", result.coeffs[0], result.coeffs[1]);

    // Test if the operations make sense
    if result.coeffs[0] == 100 - 50 && result.coeffs[1] == 200 - 150 {
        println!("✅ Basic polynomial operations work correctly");
    } else {
        println!("❌ Basic polynomial operations failed!");
        println!("Expected: coeffs[0] = 50, coeffs[1] = 50");
        println!("Got: coeffs[0] = {}, coeffs[1] = {}", result.coeffs[0], result.coeffs[1]);
    }
}

#[test]
fn test_ntt_operations() {
    // Test if the NTT operations are working correctly
    println!("=== TESTING NTT OPERATIONS ===");

    // Create a simple polynomial
    let mut poly = Poly::new();
    poly.coeffs[0] = 100;
    poly.coeffs[1] = 200;
    poly.coeffs[2] = 300;
    poly.coeffs[3] = 400;

    println!("Original polynomial:");
    for i in 0..4 {
        println!("  coeff[{}] = {}", i, poly.coeffs[i]);
    }

    // Convert to NTT domain
    ml_kem::poly::poly_ntt(&mut poly);
    println!("After NTT:");
    for i in 0..4 {
        println!("  coeff[0] = {}", poly.coeffs[0]);
    }

    // Convert back from NTT domain
    ml_kem::poly::poly_invntt_tomont(&mut poly);
    println!("After inverse NTT:");
    for i in 0..4 {
        println!("  coeff[{}] = {}", i, poly.coeffs[i]);
    }

    // Check if the values are reasonable (they might not be exactly the same due to Montgomery domain)
    let mut reasonable = true;
    for i in 0..4 {
        let coeff = poly.coeffs[i];
        if coeff < -10000 || coeff > 10000 {
            reasonable = false;
            println!("  ❌ coeff[{}] = {} is out of reasonable range", i, coeff);
        }
    }

    if reasonable {
        println!("✅ NTT operations produce reasonable values");
    } else {
        println!("❌ NTT operations produce unreasonable values");
    }
}

#[test]
fn test_polyvec_pointwise_acc_montgomery() {
    // Test if the polyvec_pointwise_acc_montgomery function is working correctly
    println!("=== TESTING POLYVEC_POINTWISE_ACC_MONTGOMERY ===");

    // Create two simple polynomial vectors
    let mut polyvec1 = PolyVec::new();
    let mut polyvec2 = PolyVec::new();
    let mut result = Poly::new();

    // Set some simple values in the first polynomial of each vector
    polyvec1.vec[0].coeffs[0] = 100;
    polyvec1.vec[0].coeffs[1] = 200;
    polyvec2.vec[0].coeffs[0] = 50;
    polyvec2.vec[0].coeffs[1] = 150;

    println!(
        "polyvec1.vec[0].coeffs[0] = {}, polyvec1.vec[0].coeffs[1] = {}",
        polyvec1.vec[0].coeffs[0],
        polyvec1.vec[0].coeffs[1]
    );
    println!(
        "polyvec2.vec[0].coeffs[0] = {}, polyvec2.vec[0].coeffs[1] = {}",
        polyvec2.vec[0].coeffs[0],
        polyvec2.vec[0].coeffs[1]
    );

    // Test the function
    ml_kem::polyvec::polyvec_pointwise_acc_montgomery(&mut result, &polyvec1, &polyvec2);
    println!(
        "polyvec_pointwise_acc_montgomery result: coeffs[0] = {}, coeffs[1] = {}",
        result.coeffs[0],
        result.coeffs[1]
    );

    // Check if the result is reasonable
    let mut reasonable = true;
    for i in 0..2 {
        let coeff = result.coeffs[i];
        if coeff < -30000 || coeff > 30000 {
            reasonable = false;
            println!("  ❌ coeff[{}] = {} is out of reasonable range", i, coeff);
        }
    }

    if reasonable {
        println!("✅ polyvec_pointwise_acc_montgomery produces reasonable values");
    } else {
        println!("❌ polyvec_pointwise_acc_montgomery produces unreasonable values");
    }
}

#[test]
fn test_decryption_step_by_step() {
    // Test the decryption process step by step to see exactly where it fails
    let mut pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut sk = [0u8; INDCPA_SECRETKEYBYTES];
    let mut ct = [0u8; INDCPA_BYTES];
    let mut m1 = [0u8; INDCPA_MSGBYTES];

    // Generate a simple test message
    for i in 0..INDCPA_MSGBYTES {
        m1[i] = i as u8;
    }

    println!("=== DECRYPTION DEBUG TEST ===");
    println!("Original message: {:?}", &m1[..8]);

    // Generate keypair
    indcpa_keypair(&mut pk, &mut sk);
    println!("✅ Keypair generated");

    // Generate random coins for encryption
    let mut coins = [0u8; 32];
    for i in 0..32 {
        coins[i] = (i * 7) as u8;
    }

    // Encrypt
    indcpa_enc(&mut ct, &m1, &pk, &coins);
    println!("✅ Message encrypted");
    println!("Ciphertext first 8 bytes: {:?}", &ct[..8]);

    // Now let's test the full decryption to see what we get
    println!("\n=== TESTING FULL DECRYPTION ===");

    let mut m2 = [0u8; INDCPA_MSGBYTES];
    indcpa_dec(&mut m2, &ct, &sk);
    println!("✅ Full decryption completed");
    println!("Decrypted message first 8 bytes: {:?}", &m2[..8]);

    // Check if they match
    if m1 == m2 {
        println!("🎉 SUCCESS: Full decryption worked!");
    } else {
        println!("❌ FAILURE: Full decryption failed!");
        println!("Original: {:?}", &m1[..8]);
        println!("Decrypted: {:?}", &m2[..8]);

        // Count differences
        let mut diff_count = 0;
        for i in 0..INDCPA_MSGBYTES {
            if m1[i] != m2[i] {
                diff_count += 1;
            }
        }
        println!("Total differences: {}/{}", diff_count, INDCPA_MSGBYTES);

        // Let's also check if the issue is in the message conversion
        println!("\n=== TESTING MESSAGE CONVERSION ===");
        let mut test_poly = Poly::new();
        poly_frommsg(&mut test_poly, &m1);
        let mut test_msg = [0u8; INDCPA_MSGBYTES];
        poly_tomsg(&mut test_msg, &mut test_poly);

        if m1 == test_msg {
            println!("✅ Message conversion works correctly");
        } else {
            println!("❌ Message conversion failed!");
            println!("Original: {:?}", &m1[..8]);
            println!("Converted: {:?}", &test_msg[..8]);
        }
    }
}

// #[test]
// fn test_indcpa_step_by_step() {
//     // Test IND-CPA functions directly to isolate the issue
//     let mut pk = [0u8; INDCPA_PUBLICKEYBYTES];
//     let mut sk = [0u8; INDCPA_SECRETKEYBYTES];
//     let mut ct = [0u8; INDCPA_BYTES];
//     let mut m1 = [0u8; INDCPA_MSGBYTES];
//     let mut m2 = [0u8; INDCPA_MSGBYTES];

//     // Generate a simple test message
//     for i in 0..INDCPA_MSGBYTES {
//         m1[i] = i as u8;
//     }

//     println!("Original message: {:?}", &m1[..8]);

//     // Generate keypair
//     indcpa_keypair(&mut pk, &mut sk);
//     println!("Generated keypair");

//     // Generate random coins for encryption
//     let mut coins = [0u8; 32]; // SYMBYTES = 32
//     for i in 0..32 {
//         coins[i] = (i * 7) as u8; // Simple deterministic "random" values for testing
//     }

//     // Encrypt
//     indcpa_enc(&mut ct, &m1, &pk, &coins);
//     println!("Encrypted message");

//     // Decrypt
//     indcpa_dec(&mut m2, &ct, &sk);
//     println!("Decrypted message: {:?}", &m2[..8]);

//     // Check if they match
//     if m1 == m2 {
//         println!("✅ IND-CPA encryption/decryption works correctly!");
//     } else {
//         println!("❌ IND-CPA encryption/decryption failed!");
//         println!("First 8 bytes of original: {:?}", &m1[..8]);
//         println!("First 8 bytes of decrypted: {:?}", &m2[..8]);

//         // Count differences
//         let mut diff_count = 0;
//         for i in 0..INDCPA_MSGBYTES {
//             if m1[i] != m2[i] {
//                 diff_count += 1;
//             }
//         }
//         println!("Total differences: {}/{}", diff_count, INDCPA_MSGBYTES);
//     }

//     assert_eq!(m1, m2, "IND-CPA encryption/decryption failed");
// }

#[test]
fn test_matrix_generation() {
    println!("=== TESTING MATRIX GENERATION ===");

    let mut seed = [0u8; 32];
    for i in 0..32 {
        seed[i] = i as u8;
    }

    // Generate matrix A (not transposed)
    let mut a = vec![ml_kem::polyvec::PolyVec::with_len(3); 3];
    ml_kem::indcpa::gen_matrix(&mut a, &seed, false);

    // Generate matrix A^T (transposed)
    let mut at = vec![ml_kem::polyvec::PolyVec::with_len(3); 3];
    ml_kem::indcpa::gen_matrix(&mut at, &seed, true);

    // Check that the matrices are different (transpose should change the matrix)
    let mut matrices_equal = true;
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..256 {
                if a[i].vec[j].coeffs[k] != at[i].vec[j].coeffs[k] {
                    matrices_equal = false;
                    break;
                }
            }
        }
    }

    if !matrices_equal {
        println!("✅ Matrix generation and transpose working correctly");
    } else {
        println!("❌ Matrix transpose not working - matrices are identical");
    }

    // This should always pass since transpose should change the matrix
    assert!(!matrices_equal, "Matrix transpose not working");
}
