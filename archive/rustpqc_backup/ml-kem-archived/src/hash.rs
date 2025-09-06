// EXACT NIST reference implementation - no modifications

use sha3::{ Sha3_256, Sha3_512, Shake256, digest::{ ExtendableOutput, XofReader } };
use sha3::digest::{ Digest, Update };

/*************************************************
 * Name:        hash_h
 *
 * Description: SHA3-256 hash function
 *
 * Arguments:   - uint8_t *out: pointer to output
 *              - const uint8_t *in: pointer to input
 *              - size_t inlen: length of input
 **************************************************/
pub fn hash_h(out: &mut [u8], in_data: &[u8], inlen: usize) {
    let mut hasher = Sha3_256::default();
    Digest::update(&mut hasher, &in_data[..inlen]);
    let result = Digest::finalize(hasher);
    out[..32].copy_from_slice(&result);
}

/*************************************************
 * Name:        hash_g
 *
 * Description: SHA3-512 hash function
 *
 * Arguments:   - uint8_t *out: pointer to output
 *              - const uint8_t *in: pointer to input
 *              - size_t inlen: length of input
 **************************************************/
pub fn hash_g(out: &mut [u8], in_data: &[u8], inlen: usize) {
    let mut hasher = Sha3_512::default();
    Digest::update(&mut hasher, &in_data[..inlen]);
    let result = Digest::finalize(hasher);
    out[..64].copy_from_slice(&result);
}

/*************************************************
 * Name:        kdf
 *
 * Description: SHAKE256-based key derivation function
 *
 * Arguments:   - uint8_t *out: pointer to output
 *              - const uint8_t *in: pointer to input
 *              - size_t inlen: length of input
 **************************************************/
pub fn kdf(out: &mut [u8], in_data: &[u8], inlen: usize) {
    let mut hasher = Shake256::default();
    hasher.update(&in_data[..inlen]);
    let mut reader = hasher.finalize_xof();
    reader.read(out);
}

/*************************************************
 * Name:        verify
 *
 * Description: Compare two arrays for equality in constant time.
 *
 * Arguments:   const uint8_t *a: pointer to first byte array
 *              const uint8_t *b: pointer to second byte array
 *              size_t len:       length of the byte arrays
 *
 * Returns 0 if the byte arrays are equal, 1 otherwise
 **************************************************/
pub fn verify(a: &[u8], b: &[u8], len: usize) -> i32 {
    let mut r: u8 = 0;

    for i in 0..len {
        r |= a[i] ^ b[i];
    }

    (-(r as i64) >> 63) as i32
}

/*************************************************
 * Name:        cmov
 *
 * Description: Copy len bytes from x to r if b is 1;
 *              don't modify x if b is 0. Requires b to be in {0,1};
 *              assumes two's complement representation of negative integers.
 *              Runs in constant time.
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *              - const uint8_t *x: pointer to input byte array
 *              - size_t len: Amount of bytes to be copied
 *              - uint8_t b: Condition bit; has to be in {0,1}
 **************************************************/
pub fn cmov(r: &mut [u8], x: &[u8], len: usize, b: u8) {
    let b_neg = -(b as i8) as u8;
    for i in 0..len {
        r[i] ^= b_neg & (r[i] ^ x[i]);
    }
}

/*************************************************
 * Name:        randombytes
 *
 * Description: Generate random bytes
 *
 * Arguments:   - uint8_t *out: pointer to output
 *              - size_t outlen: number of bytes to generate
 **************************************************/
pub fn randombytes(out: &mut [u8], outlen: usize) {
    getrandom::getrandom(&mut out[..outlen]).expect("Failed to generate random bytes");
}
