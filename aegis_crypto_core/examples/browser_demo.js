import init, * as wasm from "../pkg/aegis_crypto_core.js";

let wasmInitialized = false;

async function initWasm() {
    if (!wasmInitialized) {
        await init();
        wasmInitialized = true;
    }
}

window.runSphincsPlusDemo = async function() {
    await initWasm();
    const output = document.getElementById("sphincsOutput");
    output.textContent = "Running SPHINCS+ demo...\n";

    try {
        const message = new TextEncoder().encode("Hello from the browser! This is a SPHINCS+ test message.");
        const keypair = wasm.sphincsplus_keygen();
        output.textContent += `SPHINCS+ Public Key Length: ${keypair.public_key.length}\n`;
        output.textContent += `SPHINCS+ Secret Key Length: ${keypair.secret_key.length}\n`;

        const signature = wasm.sphincsplus_sign(keypair.secret_key, message);
        output.textContent += `SPHINCS+ Signature Length: ${signature.length}\n`;

        const isValid = wasm.sphincsplus_verify(keypair.public_key, message, signature);
        output.textContent += `SPHINCS+ Signature Valid: ${isValid}\n`;

        // Test with tampered message
        const tamperedMessage = new TextEncoder().encode("Tampered message");
        const isTamperedValid = wasm.sphincsplus_verify(keypair.public_key, tamperedMessage, signature);
        output.textContent += `SPHINCS+ Tampered Signature Valid: ${isTamperedValid}\n`;

        output.textContent += "SPHINCS+ demo completed successfully!";
    } catch (error) {
        output.textContent += `Error: ${error.message}`;
    }
};

window.runKyberDemo = async function() {
    await initWasm();
    const output = document.getElementById("kyberOutput");
    output.textContent = "Running Kyber demo...\n";

    try {
        const keypair = wasm.kyber_keygen();
        output.textContent += `Kyber Public Key Length: ${keypair.public_key.length}\n`;
        output.textContent += `Kyber Secret Key Length: ${keypair.secret_key.length}\n`;

        const encapsulated = wasm.kyber_encapsulate(keypair.public_key);
        const ciphertext = encapsulated.ciphertext;
        const sharedSecretEncaps = encapsulated.shared_secret;
        output.textContent += `Kyber Ciphertext Length: ${ciphertext.length}\n`;
        output.textContent += `Kyber Shared Secret (Encaps) Length: ${sharedSecretEncaps.length}\n`;

        const sharedSecretDecaps = wasm.kyber_decapsulate(keypair.secret_key, ciphertext);
        output.textContent += `Kyber Shared Secret (Decaps) Length: ${sharedSecretDecaps.length}\n`;

        const secretsMatch = sharedSecretEncaps.toString() === sharedSecretDecaps.toString();
        output.textContent += `Kyber Shared Secrets Match: ${secretsMatch}\n`;

        output.textContent += "Kyber demo completed successfully!";
    } catch (error) {
        output.textContent += `Error: ${error.message}`;
    }
};

window.runDilithiumDemo = async function() {
    await initWasm();
    const output = document.getElementById("dilithiumOutput");
    output.textContent = "Running Dilithium demo...\n";

    try {
        const message = new TextEncoder().encode("Dilithium test message from the browser.");
        const keypair = wasm.dilithium_keygen();
        output.textContent += `Dilithium Public Key Length: ${keypair.public_key.length}\n`;
        output.textContent += `Dilithium Secret Key Length: ${keypair.secret_key.length}\n`;

        const signature = wasm.dilithium_sign(keypair.secret_key, message);
        output.textContent += `Dilithium Signature Length: ${signature.length}\n`;

        const isValid = wasm.dilithium_verify(keypair.public_key, message, signature);
        output.textContent += `Dilithium Signature Valid: ${isValid}\n`;

        output.textContent += "Dilithium demo completed successfully!";
    } catch (error) {
        output.textContent += `Error: ${error.message}`;
    }
};

window.runFalconDemo = async function() {
    await initWasm();
    const output = document.getElementById("falconOutput");
    output.textContent = "Running Falcon demo...\n";

    try {
        const message = new TextEncoder().encode("Falcon test message from the browser.");
        const keypair = wasm.falcon_keygen();
        output.textContent += `Falcon Public Key Length: ${keypair.public_key.length}\n`;
        output.textContent += `Falcon Secret Key Length: ${keypair.secret_key.length}\n`;

        const signature = wasm.falcon_sign(keypair.secret_key, message);
        output.textContent += `Falcon Signature Length: ${signature.length}\n`;

        const isValid = wasm.falcon_verify(keypair.public_key, message, signature);
        output.textContent += `Falcon Signature Valid: ${isValid}\n`;

        output.textContent += "Falcon demo completed successfully!";
    } catch (error) {
        output.textContent += `Error: ${error.message}`;
    }
};

window.runHashAndUtilsDemo = async function() {
    await initWasm();
    const output = document.getElementById("hashUtilsOutput");
    output.textContent = "Running Hash & Utilities demo...\n";

    try {
        const data = new TextEncoder().encode("Data to be hashed in the browser.");

        const sha3_256_hash = wasm.sha3_256_hash(data);
        output.textContent += `SHA3-256 Hash: ${wasm.bytes_to_hex(sha3_256_hash)}\n`;

        const sha3_512_hash = wasm.sha3_512_hash(data);
        output.textContent += `SHA3-512 Hash: ${wasm.bytes_to_hex(sha3_512_hash)}\n`;

        const blake3_hash = wasm.blake3_hash(data);
        output.textContent += `BLAKE3 Hash: ${wasm.bytes_to_hex(blake3_hash)}\n`;

        // Utilities
        const originalHex = "deadbeef";
        const bytesFromHex = wasm.hex_to_bytes(originalHex);
        const hexFromBytes = wasm.bytes_to_hex(bytesFromHex);
        output.textContent += `Hex conversion roundtrip valid: ${originalHex === hexFromBytes}\n`;

        output.textContent += "Hash & Utilities demo completed successfully!";
    } catch (error) {
        output.textContent += `Error: ${error.message}`;
    }
};

