import { readFileSync } from 'fs';
import init, { kyber_keygen, kyber_encapsulate, kyber_decapsulate } from './pkg/aegis_crypto_core.js';

async function test() {
    const wasm_bytes = readFileSync('./pkg/aegis_crypto_core_bg.wasm');
    await init(wasm_bytes);

    console.log("Testing Kyber...");

    // Generate keypair
    const keypair = kyber_keygen();
    console.log("Public key length:", keypair.public_key.length);
    console.log("Secret key length:", keypair.secret_key.length);

    // Encapsulate
    try {
        const ciphertext = kyber_encapsulate(keypair.public_key);
        console.log("Ciphertext length:", ciphertext.length);
        console.log("Ciphertext (first 32 bytes):", Array.from(ciphertext.slice(0, 32)).map(b => b.toString(16).padStart(2, '0')).join(''));

        // Decapsulate
        try {
            const shared_secret = kyber_decapsulate(keypair.secret_key, ciphertext);
            console.log("Shared secret length:", shared_secret.length);
            console.log("Shared secret:", Array.from(shared_secret).map(b => b.toString(16).padStart(2, '0')).join(''));
            console.log("Kyber test: SUCCESS");
        } catch (e) {
            console.log("Decapsulation failed:", e);
        }
    } catch (e) {
        console.log("Encapsulation failed:", e);
    }
}

test();
