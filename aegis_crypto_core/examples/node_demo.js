const fs = require("fs");
const path = require("path");
const wasm = require("../pkg");

async function runNodeDemo() {
    const wasmBytes = fs.readFileSync(path.join(__dirname, "..", "pkg", "aegis_crypto_core_bg.wasm"));
    await wasm.initSync(wasmBytes);

    console.log("\n--- Aegis Crypto Core Node.js Demo ---");

    // SPHINCS+ (SLH-DSA) Digital Signatures
    console.log("\n[SPHINCS+] Generating keypair, signing, and verifying...");
    const sphincsMessage = new TextEncoder().encode("Hello from Node.js! This is a SPHINCS+ test message.");
    const sphincsKeypair = wasm.sphincsplus_keygen();
    const sphincsSignature = wasm.sphincsplus_sign(sphincsKeypair.secret_key, sphincsMessage);
    const sphincsValid = wasm.sphincsplus_verify(sphincsKeypair.public_key, sphincsMessage, sphincsSignature);
    console.log(`SPHINCS+ Signature Valid: ${sphincsValid}`);
    if (!sphincsValid) console.error("SPHINCS+ verification failed!");

    // Kyber (ML-KEM) Key Encapsulation
    console.log("\n[Kyber] Generating keypair, encapsulating, and decapsulating...");
    const kyberKeypair = wasm.kyber_keygen();
    const encapsulated = wasm.kyber_encapsulate(kyberKeypair.public_key);
    const kyberCiphertext = encapsulated.ciphertext;
    const kyberSharedSecretEncaps = encapsulated.shared_secret;
    const kyberSharedSecretDecaps = wasm.kyber_decapsulate(kyberKeypair.secret_key, kyberCiphertext);
    console.log(`Kyber Shared Secret Match: ${kyberSharedSecretEncaps.toString() === kyberSharedSecretDecaps.toString()}`);
    if (kyberSharedSecretEncaps.toString() !== kyberSharedSecretDecaps.toString()) console.error("Kyber shared secrets do not match!");

    // Dilithium (ML-DSA) Digital Signatures
    console.log("\n[Dilithium] Generating keypair, signing, and verifying...");
    const dilithiumMessage = new TextEncoder().encode("Another message for Dilithium.");
    const dilithiumKeypair = wasm.dilithium_keygen();
    const dilithiumSignature = wasm.dilithium_sign(dilithiumKeypair.secret_key, dilithiumMessage);
    const dilithiumValid = wasm.dilithium_verify(dilithiumKeypair.public_key, dilithiumMessage, dilithiumSignature);
    console.log(`Dilithium Signature Valid: ${dilithiumValid}`);
    if (!dilithiumValid) console.error("Dilithium verification failed!");

    // Falcon (FN-DSA) Digital Signatures
    console.log("\n[Falcon] Generating keypair, signing, and verifying...");
    const falconMessage = new TextEncoder().encode("Falcon's flight.");
    const falconKeypair = wasm.falcon_keygen();
    const falconSignature = wasm.falcon_sign(falconKeypair.secret_key, falconMessage);
    const falconValid = wasm.falcon_verify(falconKeypair.public_key, falconMessage, falconSignature);
    console.log(`Falcon Signature Valid: ${falconValid}`);
    if (!falconValid) console.error("Falcon verification failed!");

    // Hash Functions
    console.log("\n[Hashing] Testing SHA3-256, SHA3-512, BLAKE3...");
    const hashData = new TextEncoder().encode("Data to be hashed.");
    const sha3_256_hash = wasm.sha3_256_hash(hashData);
    console.log(`SHA3-256 Hash: ${wasm.bytes_to_hex(sha3_256_hash)}`);
    const sha3_512_hash = wasm.sha3_512_hash(hashData);
    console.log(`SHA3-512 Hash: ${wasm.bytes_to_hex(sha3_512_hash)}`);
    const blake3_hash = wasm.blake3_hash(hashData);
    console.log(`BLAKE3 Hash: ${wasm.bytes_to_hex(blake3_hash)}`);

    // Utilities
    console.log("\n[Utilities] Testing hex conversion...");
    const originalHex = "deadbeef";
    const bytesFromHex = wasm.hex_to_bytes(originalHex);
    const hexFromBytes = wasm.bytes_to_hex(bytesFromHex);
    console.log(`Hex conversion roundtrip valid: ${originalHex === hexFromBytes}`);
    if (originalHex !== hexFromBytes) console.error("Hex conversion failed!");

    console.log("\n--- Node.js Demo Complete ---");
}

runNodeDemo();


