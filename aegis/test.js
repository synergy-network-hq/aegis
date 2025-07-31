const fs = require("fs");
const path = require("path");
const wasm = require("./pkg");

async function runTests() {
    const wasmBytes = fs.readFileSync(path.join(__dirname, "pkg", "aegis_crypto_core_bg.wasm"));
    await wasm.initSync(wasmBytes);

    console.log("Testing Dilithium...");
    try {
        const keypair = wasm.dilithium_keygen();
        const message = new TextEncoder().encode("test message");
        const signature = wasm.dilithium_sign(keypair.secret_key, message);
        const isValid = wasm.dilithium_verify(keypair.public_key, message, signature);
        console.log("Dilithium verification:", isValid);
        if (!isValid) throw new Error("Dilithium verification failed");
    } catch (e) {
        console.error("Dilithium test failed:", e);
    }

    console.log("\nTesting SHA3/BLAKE3...");
    try {
        const data = new TextEncoder().encode("hello world");
        const sha3_256_hash = wasm.sha3_256_hash(data);
        console.log("SHA3-256 hash:", wasm.bytes_to_hex(sha3_256_hash));
        const sha3_512_hash = wasm.sha3_512_hash(data);
        console.log("SHA3-512 hash:", wasm.bytes_to_hex(sha3_512_hash));
        const blake3_hash = wasm.blake3_hash(data);
        console.log("BLAKE3 hash:", wasm.bytes_to_hex(blake3_hash));
    } catch (e) {
        console.error("Hashing test failed:", e);
    }

    console.log("\nTesting Kyber...");
    try {
        const keypair = wasm.kyber_keygen();
        const public_key = keypair.public_key;
        const encapsulated = wasm.kyber_encapsulate(public_key);
        const ciphertext = encapsulated.ciphertext;
        const shared_secret_encaps = encapsulated.shared_secret;
        console.log("Kyber ciphertext length:", ciphertext.length);
        console.log("Kyber shared secret (encapsulation) length:", shared_secret_encaps.length);
        const shared_secret_decaps = wasm.kyber_decapsulate(keypair.secret_key, ciphertext);
        console.log("Kyber shared secret (decapsulation) length:", shared_secret_decaps.length);
        if (shared_secret_decaps.length === 0) throw new Error("Kyber decapsulation failed");
        if (shared_secret_encaps.toString() !== shared_secret_decaps.toString()) throw new Error("Kyber shared secrets do not match");
    } catch (e) {
        console.error("Kyber test failed:", e);
    }

    console.log("\nTesting Falcon...");
    try {
        const keypair = wasm.falcon_keygen();
        const message = new TextEncoder().encode("another test message");
        const signature = wasm.falcon_sign(keypair.secret_key, message);
        const isValid = wasm.falcon_verify(keypair.public_key, message, signature);
        console.log("Falcon verification:", isValid);
        if (!isValid) throw new Error("Falcon verification failed");
    } catch (e) {
        console.error("Falcon test failed:", e);
    }

    console.log("\nTesting Utilities...");
    try {
        const hex_string = "0102030405";
        const bytes = wasm.hex_to_bytes(hex_string);
        const back_to_hex = wasm.bytes_to_hex(bytes);
        console.log("Hex to bytes and back:", back_to_hex === hex_string);
        if (back_to_hex !== hex_string) throw new Error("Hex utility test failed");
    } catch (e) {
        console.error("Utilities test failed:", e);
    }
}

runTests();


