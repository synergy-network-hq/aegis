use aegis_crypto_core::hash::{ sha3_256_hash, blake3_hash };
use aegis_crypto_core::utils::{ hex_to_bytes, bytes_to_hex };

fn main() {
    println!("🧪 Testing Aegis Crypto Core Functionality...\n");

    // Test 1: Hash Functions
    println!("1. Testing Hash Functions:");
    let test_data = b"Hello, Aegis Crypto!";

    // SHA3-256
    let sha3_hash = sha3_256_hash(test_data);
    println!("   SHA3-256: {}", bytes_to_hex(&sha3_hash));

    // Blake3
    let blake3_result = blake3_hash(test_data);
    println!("   Blake3: {}", bytes_to_hex(&blake3_result));

    // Test 2: Utility Functions
    println!("\n2. Testing Utility Functions:");
    let hex_string = "48656c6c6f2c2041656769732043727970746f21"; // "Hello, Aegis Crypto!"
    let bytes = hex_to_bytes(hex_string).unwrap();
    let back_to_hex = bytes_to_hex(&bytes);
    println!("   Hex to bytes and back: {} -> {}", hex_string, back_to_hex);

    // Test 3: Verify hash consistency
    println!("\n3. Testing Hash Consistency:");
    let sha3_hash2 = sha3_256_hash(test_data);
    let blake3_result2 = blake3_hash(test_data);

    if sha3_hash == sha3_hash2 {
        println!("   ✅ SHA3-256 hash is consistent");
    } else {
        println!("   ❌ SHA3-256 hash is inconsistent");
    }

    if blake3_result == blake3_result2 {
        println!("   ✅ Blake3 hash is consistent");
    } else {
        println!("   ❌ Blake3 hash is inconsistent");
    }

    // Test 4: Different input produces different hash
    println!("\n4. Testing Hash Uniqueness:");
    let different_data = b"Hello, Aegis Crypto!!"; // Note the extra '!'
    let different_sha3 = sha3_256_hash(different_data);
    let different_blake3 = blake3_hash(different_data);

    if sha3_hash != different_sha3 {
        println!("   ✅ SHA3-256 produces different hashes for different inputs");
    } else {
        println!("   ❌ SHA3-256 produces same hash for different inputs");
    }

    if blake3_result != different_blake3 {
        println!("   ✅ Blake3 produces different hashes for different inputs");
    } else {
        println!("   ❌ Blake3 produces same hash for different inputs");
    }

    println!("\n🎉 Core functionality test completed!");
}
