/*
 * Secure Messaging Demo
 *
 * This script demonstrates how to use the SecureMessagingAPI class to
 * perform Kyber key encapsulation/decapsulation, AES‑GCM encryption,
 * Dilithium/Falcon digital signatures, and combined secure messaging.
 *
 * To run this demo:
 * 1. Build the Aegis WASM package with `wasm-pack build --target nodejs`.
 * 2. From the root of this repository, navigate to `aegis_crypto_core`
 *    and run `node examples/secure_messaging_demo.js`.
 */

const path = require('path');
const fs = require('fs');
const SecureMessagingAPI = require('./secure_messaging_api');

// Import the `init` function and the low‑level cryptographic functions from
// the generated WASM package.  When building with `wasm-pack` using the
// `nodejs` target, the package will reside in `../pkg` relative to this
// file.  The `init` function accepts the path to the `.wasm` file and
// returns a promise that resolves to the wasm exports.
const init = require('../pkg/aegis_crypto_core.js');

async function runDemo() {
  console.log('--- Secure Messaging Demo ---');
  const api = new SecureMessagingAPI();
  const wasmPath = path.resolve(__dirname, '../pkg/aegis_crypto_core_bg.wasm');
  const wasmBytes = fs.readFileSync(wasmPath);
  // Initialise the WASM module
  await api.init(init, wasmBytes);
  console.log('WASM module initialised');

  // Generate key pairs for Alice and Bob
  const aliceKyber = api.generateKyberKeyPair();
  const aliceDilithium = api.generateDilithiumKeyPair();
  const aliceFalcon = api.generateFalconKeyPair();
  const bobKyber = api.generateKyberKeyPair();
  const bobDilithium = api.generateDilithiumKeyPair();
  const bobFalcon = api.generateFalconKeyPair();
  console.log('Generated key pairs for Alice and Bob');

  // Basic encryption/decryption
  const plaintext = 'Hello Bob! This is a secret message from Alice.';
  const encrypted = api.encrypt(bobKyber.public_key, plaintext);
  const decrypted = api.decrypt(bobKyber.secret_key, encrypted).toString();
  console.log('Original message:   ', plaintext);
  console.log('Decrypted message:  ', decrypted);
  console.log('Encryption success: ', plaintext === decrypted);

  // Digital signatures (Dilithium and Falcon)
  const messageToSign = 'This message will be signed.';
  const dilithiumSig = api.signWithDilithium(aliceDilithium.secret_key, messageToSign);
  const dilithiumValid = api.verifyDilithiumSignature(
    aliceDilithium.public_key,
    messageToSign,
    dilithiumSig
  );
  console.log('Dilithium signature valid:', dilithiumValid);
  const falconSig = api.signWithFalcon(aliceFalcon.secret_key, messageToSign);
  const falconValid = api.verifyFalconSignature(
    aliceFalcon.public_key,
    messageToSign,
    falconSig
  );
  console.log('Falcon signature valid:', falconValid);

  // Combined secure message using Dilithium
  const combinedMessageDilithium = 'Confidential and authenticated using Dilithium.';
  const secureMsgDilithium = await api.createSecureMessage(
    {
      senderSigningKey: aliceDilithium.secret_key,
      recipientPublicKey: bobKyber.public_key,
    },
    combinedMessageDilithium,
    'dilithium'
  );
  const verifiedDilithium = await api.verifySecureMessage(
    {
      recipientSecretKey: bobKyber.secret_key,
      senderVerifyingKey: aliceDilithium.public_key,
    },
    secureMsgDilithium
  );
  console.log('Dilithium secure message decrypted:', verifiedDilithium.message.toString());
  console.log('Dilithium signature valid:', verifiedDilithium.signatureValid);

  // Combined secure message using Falcon
  const combinedMessageFalcon = 'Confidential and authenticated using Falcon.';
  const secureMsgFalcon = await api.createSecureMessage(
    {
      senderSigningKey: aliceFalcon.secret_key,
      recipientPublicKey: bobKyber.public_key,
    },
    combinedMessageFalcon,
    'falcon'
  );
  const verifiedFalcon = await api.verifySecureMessage(
    {
      recipientSecretKey: bobKyber.secret_key,
      senderVerifyingKey: aliceFalcon.public_key,
    },
    secureMsgFalcon
  );
  console.log('Falcon secure message decrypted:', verifiedFalcon.message.toString());
  console.log('Falcon signature valid:', verifiedFalcon.signatureValid);
}

runDemo().catch((err) => {
  console.error(err);
});