// EXACT NIST reference implementation - no modifications

use crate::params::{
    SYMBYTES,
    PUBLICKEYBYTES,
    SECRETKEYBYTES,
    CIPHERTEXTBYTES,
    INDCPA_PUBLICKEYBYTES,
    INDCPA_SECRETKEYBYTES,
    INDCPA_BYTES,
    INDCPA_MSGBYTES,
};
use crate::indcpa::{ indcpa_keypair, indcpa_enc, indcpa_dec };
use crate::hash::{ hash_h, hash_g, kdf, verify, cmov, randombytes };

/*************************************************
 * Name:        crypto_kem_keypair
 *
 * Description: Generates public and private key
 *              for CCA-secure Kyber key encapsulation mechanism
 *
 * Arguments:   - unsigned char *pk: pointer to output public key
 *                (an already allocated array of CRYPTO_PUBLICKEYBYTES bytes)
 *              - unsigned char *sk: pointer to output private key
 *                (an already allocated array of CRYPTO_SECRETKEYBYTES bytes)
 *
 * Returns 0 (success)
 **************************************************/
pub fn crypto_kem_keypair(pk: &mut [u8; PUBLICKEYBYTES], sk: &mut [u8; SECRETKEYBYTES]) -> i32 {
    let mut indcpa_pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut indcpa_sk = [0u8; INDCPA_SECRETKEYBYTES];

    indcpa_keypair(&mut indcpa_pk, &mut indcpa_sk);

    // Copy IND-CPA public key to output
    pk[..INDCPA_PUBLICKEYBYTES].copy_from_slice(&indcpa_pk);

    // Copy IND-CPA secret key to output
    sk[..INDCPA_SECRETKEYBYTES].copy_from_slice(&indcpa_sk);

    // Copy public key to secret key
    for i in 0..INDCPA_PUBLICKEYBYTES {
        sk[i + INDCPA_SECRETKEYBYTES] = pk[i];
    }

    hash_h(&mut sk[SECRETKEYBYTES - 2 * SYMBYTES..], pk, PUBLICKEYBYTES);
    randombytes(&mut sk[SECRETKEYBYTES - SYMBYTES..], SYMBYTES);

    0
}

/*************************************************
 * Name:        crypto_kem_enc
 *
 * Description: Generates cipher text and shared
 *              secret for given public key
 *
 * Arguments:   - unsigned char *ct: pointer to output cipher text
 *                (an already allocated array of CRYPTO_CIPHERTEXTBYTES bytes)
 *              - unsigned char *ss: pointer to output shared secret
 *                (an already allocated array of CRYPTO_BYTES bytes)
 *              - const unsigned char *pk: pointer to input public key
 *                (an already allocated array of CRYPTO_PUBLICKEYBYTES bytes)
 *
 * Returns 0 (success)
 **************************************************/
pub fn crypto_kem_enc(
    ct: &mut [u8; CIPHERTEXTBYTES],
    ss: &mut [u8; SYMBYTES],
    pk: &[u8; PUBLICKEYBYTES]
) -> i32 {
    let mut buf = [0u8; 2 * SYMBYTES];
    let mut kr = [0u8; 2 * SYMBYTES];

    randombytes(&mut buf[..SYMBYTES], SYMBYTES);
    let buf_copy = buf[..SYMBYTES].to_vec();
    hash_h(&mut buf[..SYMBYTES], &buf_copy, SYMBYTES);

    hash_h(&mut buf[SYMBYTES..], pk, PUBLICKEYBYTES);
    hash_g(&mut kr, &buf, 2 * SYMBYTES);

    // Create properly sized buffers for IND-CPA functions
    let mut indcpa_ct = [0u8; INDCPA_BYTES];
    let mut indcpa_pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut indcpa_coins = [0u8; SYMBYTES];

    // Copy data to IND-CPA buffers
    indcpa_pk.copy_from_slice(&pk[..INDCPA_PUBLICKEYBYTES]);
    indcpa_coins.copy_from_slice(&kr[SYMBYTES..]);

    // coins are in kr+SYMBYTES
    indcpa_enc(
        &mut indcpa_ct,
        &buf[..INDCPA_MSGBYTES].try_into().unwrap(),
        &indcpa_pk,
        &indcpa_coins
    );

    // Copy IND-CPA ciphertext to output
    ct.copy_from_slice(&indcpa_ct);

    hash_h(&mut kr[SYMBYTES..], ct, CIPHERTEXTBYTES);
    kdf(ss, &kr, 2 * SYMBYTES);

    0
}

/*************************************************
 * Name:        crypto_kem_dec
 *
 * Description: Generates shared secret for given
 *              cipher text and private key
 *
 * Arguments:   - unsigned char *ss: pointer to output shared secret
 *                (an already allocated array of CRYPTO_BYTES bytes)
 *              - const unsigned char *ct: pointer to input cipher text
 *                (an already allocated array of CRYPTO_CIPHERTEXTBYTES bytes)
 *              - const unsigned char *sk: pointer to input private key
 *                (an already allocated array of CRYPTO_SECRETKEYBYTES bytes)
 *
 * Returns 0.
 *
 * On failure, ss will contain a pseudo-random value.
 **************************************************/
pub fn crypto_kem_dec(
    ss: &mut [u8; SYMBYTES],
    ct: &[u8; CIPHERTEXTBYTES],
    sk: &[u8; SECRETKEYBYTES]
) -> i32 {
    let mut buf = [0u8; 2 * SYMBYTES];
    let mut kr = [0u8; 2 * SYMBYTES];
    let mut cmp = [0u8; CIPHERTEXTBYTES];
    let pk = &sk[INDCPA_SECRETKEYBYTES..INDCPA_SECRETKEYBYTES + INDCPA_PUBLICKEYBYTES];

    // Create properly sized buffers for IND-CPA functions
    let mut indcpa_ct = [0u8; INDCPA_BYTES];
    let mut indcpa_sk = [0u8; INDCPA_SECRETKEYBYTES];
    let mut indcpa_msg = [0u8; INDCPA_MSGBYTES];

    // Copy data to IND-CPA buffers
    indcpa_ct.copy_from_slice(&ct[..INDCPA_BYTES]);
    indcpa_sk.copy_from_slice(&sk[..INDCPA_SECRETKEYBYTES]);

    indcpa_dec(&mut indcpa_msg, &indcpa_ct, &indcpa_sk);

    // Copy decrypted message to buf
    buf[..INDCPA_MSGBYTES].copy_from_slice(&indcpa_msg);

    // Multitarget countermeasure for coins + contributory KEM
    for i in 0..SYMBYTES {
        buf[SYMBYTES + i] = sk[SECRETKEYBYTES - 2 * SYMBYTES + i];
    }

    hash_g(&mut kr, &buf, 2 * SYMBYTES);

    // Create buffer for re-encryption
    let mut indcpa_pk = [0u8; INDCPA_PUBLICKEYBYTES];
    let mut indcpa_coins = [0u8; SYMBYTES];

    // Copy data to IND-CPA buffers
    indcpa_pk.copy_from_slice(pk);
    indcpa_coins.copy_from_slice(&kr[SYMBYTES..]);

    // coins are in kr+SYMBYTES
    indcpa_enc(&mut cmp, &indcpa_msg, &indcpa_pk, &indcpa_coins);

    let fail = verify(ct, &cmp, CIPHERTEXTBYTES) != 0;

    hash_h(&mut kr[SYMBYTES..], ct, CIPHERTEXTBYTES);

    cmov(&mut kr, &sk[SECRETKEYBYTES - SYMBYTES..], SYMBYTES, fail as u8);

    kdf(ss, &kr, 2 * SYMBYTES);

    0
}
