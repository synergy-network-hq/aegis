/*
 * SecureMessagingAPI
 *
 * This class provides a high‑level interface for performing post‑quantum
 * secure messaging using the primitives exported by the Aegis Crypto Core
 * WASM package.  It supports key generation, encryption/decryption
 * (Kyber KEM + AES‑256‑GCM), and digital signatures using Dilithium or
 * Falcon.  The `createSecureMessage` and `verifySecureMessage` helpers
 * combine these primitives to produce authenticated, confidential
 * messages that can be safely exchanged over untrusted channels.
 *
 * The implementation relies on Node.js' `crypto` module for AES‑GCM
 * encryption when running in a Node environment.  In a browser
 * environment, Web Crypto API should be used instead (see the
 * `secure_messaging_browser_demo.html` for an example).
 */

const crypto = require('crypto');

class SecureMessagingAPI {
  constructor() {
    this.wasm = null;
    this.initialized = false;
  }

  /**
   * Initialise the WASM module.  This must be called before any other
   * cryptographic operation.  The `init` function is provided by the
   * generated `aegis_crypto_core.js` bundle in the `pkg` directory.
   *
   * @param {Function} initFn The `init` function exported from the WASM package.
   * @param {Uint8Array|Buffer} [wasmBytes] Optional raw wasm bytes.  If
   * omitted, the glue code will attempt to fetch the wasm file based on
   * import meta information.
   */
  async init(initFn, wasmBytes) {
    if (!this.initialized) {
      this.wasm = await initFn(wasmBytes);
      this.initialized = true;
    }
  }

  // -------------------------------------------------------------------------
  // Key generation helpers

  /**
   * Generate a Kyber key pair (encapsulation/decapsulation keys).
   *
   * @returns {Object} A key pair with `public_key` and `secret_key` fields.
   */
  generateKyberKeyPair() {
    this.#ensureInit();
    return this.wasm.kyber_keygen();
  }

  /**
   * Generate a Dilithium key pair (digital signature keys).
   *
   * @returns {Object} A key pair with `public_key` and `secret_key` fields.
   */
  generateDilithiumKeyPair() {
    this.#ensureInit();
    return this.wasm.dilithium_keygen();
  }

  /**
   * Generate a Falcon key pair (digital signature keys).
   *
   * @returns {Object} A key pair with `public_key` and `secret_key` fields.
   */
  generateFalconKeyPair() {
    this.#ensureInit();
    return this.wasm.falcon_keygen();
  }

  // -------------------------------------------------------------------------
  // Basic Kyber encryption/decryption

  /**
   * Encrypt a message for the recipient using Kyber KEM and AES‑256‑GCM.
   *
   * The process is as follows:
   * 1. Perform Kyber encapsulation with the recipient's public key to
   *    derive a shared secret and a KEM ciphertext.
   * 2. Use the first 32 bytes of the shared secret as the AES‑256 key.
   * 3. Generate a 96‑bit random nonce and encrypt the plaintext with
   *    AES‑GCM, producing the cipher text and authentication tag.
   *
   * @param {Uint8Array|Buffer} recipientPublicKey The recipient's Kyber public key.
   * @param {string|Buffer|Uint8Array} message The plaintext message to encrypt.
   * @returns {Object} An object containing the KEM ciphertext, AES IV,
   *   AES authentication tag and encrypted data.
   */
  encrypt(recipientPublicKey, message) {
    this.#ensureInit();
    // Ensure inputs are Buffers
    const pubKey = Buffer.from(recipientPublicKey);
    const msgBuf = Buffer.isBuffer(message) ? message : Buffer.from(message);
    // Kyber encapsulation
    const encaps = this.wasm.kyber_encapsulate(pubKey);
    const shared = Buffer.from(encaps.shared_secret);
    const kemCiphertext = Buffer.from(encaps.ciphertext);
    // Derive AES‑256 key from the shared secret (use first 32 bytes)
    const key = shared.slice(0, 32);
    const iv = crypto.randomBytes(12); // 96‑bit nonce
    const cipher = crypto.createCipheriv('aes-256-gcm', key, iv);
    const encrypted = Buffer.concat([cipher.update(msgBuf), cipher.final()]);
    const tag = cipher.getAuthTag();
    return {
      kemCiphertext: kemCiphertext,
      iv: iv,
      ciphertext: encrypted,
      tag: tag,
    };
  }

  /**
   * Decrypt a message using the recipient's Kyber secret key and AES‑GCM.
   *
   * @param {Uint8Array|Buffer} recipientSecretKey The recipient's Kyber secret key.
   * @param {Object} encryptedObj The object returned by `encrypt()`.
   * @returns {Buffer} The decrypted plaintext.
   */
  decrypt(recipientSecretKey, encryptedObj) {
    this.#ensureInit();
    const sk = Buffer.from(recipientSecretKey);
    const { kemCiphertext, iv, ciphertext, tag } = encryptedObj;
    // Recover the shared secret
    const shared = Buffer.from(
      this.wasm.kyber_decapsulate(sk, Buffer.from(kemCiphertext))
    );
    const key = shared.slice(0, 32);
    const decipher = crypto.createDecipheriv('aes-256-gcm', key, iv);
    decipher.setAuthTag(Buffer.from(tag));
    const decrypted = Buffer.concat([
      decipher.update(Buffer.from(ciphertext)),
      decipher.final(),
    ]);
    return decrypted;
  }

  // -------------------------------------------------------------------------
  // Digital signatures helpers

  /**
   * Sign a message using Dilithium.
   * @param {Uint8Array|Buffer} secretKey The Dilithium secret key.
   * @param {string|Buffer|Uint8Array} message The message to sign.
   * @returns {Buffer} The signature bytes.
   */
  signWithDilithium(secretKey, message) {
    this.#ensureInit();
    const sk = Buffer.from(secretKey);
    const msg = Buffer.isBuffer(message) ? message : Buffer.from(message);
    return Buffer.from(this.wasm.dilithium_sign(sk, msg));
  }

  /**
   * Verify a Dilithium signature.
   * @param {Uint8Array|Buffer} publicKey The Dilithium public key.
   * @param {string|Buffer|Uint8Array} message The original message.
   * @param {Uint8Array|Buffer} signature The signature to verify.
   * @returns {boolean} `true` if the signature is valid, `false` otherwise.
   */
  verifyDilithiumSignature(publicKey, message, signature) {
    this.#ensureInit();
    const pk = Buffer.from(publicKey);
    const msg = Buffer.isBuffer(message) ? message : Buffer.from(message);
    const sig = Buffer.from(signature);
    return this.wasm.dilithium_verify(pk, msg, sig);
  }

  /**
   * Sign a message using Falcon.
   * @param {Uint8Array|Buffer} secretKey The Falcon secret key.
   * @param {string|Buffer|Uint8Array} message The message to sign.
   * @returns {Buffer} The signature bytes.
   */
  signWithFalcon(secretKey, message) {
    this.#ensureInit();
    const sk = Buffer.from(secretKey);
    const msg = Buffer.isBuffer(message) ? message : Buffer.from(message);
    return Buffer.from(this.wasm.falcon_sign(sk, msg));
  }

  /**
   * Verify a Falcon signature.
   * @param {Uint8Array|Buffer} publicKey The Falcon public key.
   * @param {string|Buffer|Uint8Array} message The original message.
   * @param {Uint8Array|Buffer} signature The signature to verify.
   * @returns {boolean} `true` if the signature is valid, `false` otherwise.
   */
  verifyFalconSignature(publicKey, message, signature) {
    this.#ensureInit();
    const pk = Buffer.from(publicKey);
    const msg = Buffer.isBuffer(message) ? message : Buffer.from(message);
    const sig = Buffer.from(signature);
    return this.wasm.falcon_verify(pk, msg, sig);
  }

  // -------------------------------------------------------------------------
  // Combined secure messaging helpers

  /**
   * Create a secure message by encrypting the plaintext with Kyber KEM
   * and signing the resulting ciphertext bundle using the specified
   * signature scheme (`dilithium` or `falcon`).
   *
   * @param {Object} keys An object containing `senderSigningKey` (secret
   *   key for signing) and `recipientPublicKey` (Kyber public key).
   * @param {string|Buffer|Uint8Array} message The plaintext message to send.
   * @param {string} scheme Either `'dilithium'` or `'falcon'`.
   * @returns {Object} A secure message object containing the KEM
   *   ciphertext, AES IV, AES ciphertext, authentication tag, digital
   *   signature and the chosen scheme.
   */
  async createSecureMessage(keys, message, scheme) {
    this.#ensureInit();
    const { senderSigningKey, recipientPublicKey } = keys;
    const encrypted = this.encrypt(recipientPublicKey, message);
    // Prepare a byte sequence to sign: kemCiphertext || iv || ciphertext || tag
    const dataToSign = Buffer.concat([
      Buffer.from(encrypted.kemCiphertext),
      Buffer.from(encrypted.iv),
      Buffer.from(encrypted.ciphertext),
      Buffer.from(encrypted.tag),
    ]);
    let signature;
    if (scheme === 'dilithium') {
      signature = this.signWithDilithium(senderSigningKey, dataToSign);
    } else if (scheme === 'falcon') {
      signature = this.signWithFalcon(senderSigningKey, dataToSign);
    } else {
      throw new Error(`Unsupported signature scheme: ${scheme}`);
    }
    return {
      kemCiphertext: Buffer.from(encrypted.kemCiphertext),
      iv: Buffer.from(encrypted.iv),
      ciphertext: Buffer.from(encrypted.ciphertext),
      tag: Buffer.from(encrypted.tag),
      signature: Buffer.from(signature),
      scheme: scheme,
    };
  }

  /**
   * Verify and decrypt a secure message.  This function performs
   * signature verification using the sender's public key and then
   * decapsulates and decrypts the message if the signature is valid.
   *
   * @param {Object} keys An object containing `recipientSecretKey` (Kyber
   *   secret key) and `senderVerifyingKey` (public key for the chosen
   *   signature scheme).
   * @param {Object} secureMessage The secure message object returned by
   *   `createSecureMessage()`.
   * @returns {Object} An object with `message` (Buffer) and
   *   `signatureValid` (boolean).
   */
  async verifySecureMessage(keys, secureMessage) {
    this.#ensureInit();
    const { recipientSecretKey, senderVerifyingKey } = keys;
    const { kemCiphertext, iv, ciphertext, tag, signature, scheme } = secureMessage;
    const dataToVerify = Buffer.concat([
      Buffer.from(kemCiphertext),
      Buffer.from(iv),
      Buffer.from(ciphertext),
      Buffer.from(tag),
    ]);
    let isValid;
    if (scheme === 'dilithium') {
      isValid = this.verifyDilithiumSignature(senderVerifyingKey, dataToVerify, signature);
    } else if (scheme === 'falcon') {
      isValid = this.verifyFalconSignature(senderVerifyingKey, dataToVerify, signature);
    } else {
      throw new Error(`Unsupported signature scheme: ${scheme}`);
    }
    let decrypted = null;
    if (isValid) {
      decrypted = this.decrypt(recipientSecretKey, {
        kemCiphertext: kemCiphertext,
        iv: iv,
        ciphertext: ciphertext,
        tag: tag,
      });
    }
    return { message: decrypted, signatureValid: isValid };
  }

  // -------------------------------------------------------------------------
  // Private helper to ensure the WASM has been initialised.
  #ensureInit() {
    if (!this.initialized || !this.wasm) {
      throw new Error('SecureMessagingAPI has not been initialised. Call init() first.');
    }
  }
}

module.exports = SecureMessagingAPI;