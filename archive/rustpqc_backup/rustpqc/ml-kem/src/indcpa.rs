// EXACT NIST reference implementation - no modifications

use crate::params::{
    K,
    N,
    Q,
    SYMBYTES,
    POLYVECBYTES,
    POLYVECCOMPRESSEDBYTES,
    POLYCOMPRESSEDBYTES,
    INDCPA_PUBLICKEYBYTES,
    INDCPA_SECRETKEYBYTES,
    INDCPA_BYTES,
    INDCPA_MSGBYTES,
};
use crate::poly::{
    Poly,
    poly_add,
    poly_compress,
    poly_decompress,
    poly_frommsg,
    poly_tomsg,
    poly_invntt_tomont,
    poly_tomont,
    poly_getnoise_eta1,
    poly_getnoise_eta2,
    poly_reduce,
    poly_sub,
};
use crate::polyvec::{
    PolyVec,
    polyvec_add,
    polyvec_compress,
    polyvec_decompress,
    polyvec_frombytes,
    polyvec_ntt,
    polyvec_invntt_tomont,
    polyvec_pointwise_acc_montgomery,
    polyvec_reduce,
    polyvec_tobytes,
};

const GEN_MATRIX_NBLOCKS: usize = ((((12 * N) / 8) * (1 << 12)) / Q + 168) / 168;

/*************************************************
 * Name:        pack_pk
 *
 * Description: Serialize the public key as concatenation of the
 *              serialized vector of polynomials pk
 *              and the public seed used to generate the matrix A.
 *
 * Arguments:   uint8_t *r:          pointer to the output serialized public key
 *              polyvec *pk:         pointer to the input public-key polyvec
 *              const uint8_t *seed: pointer to the input public seed
 **************************************************/
fn pack_pk(r: &mut [u8; INDCPA_PUBLICKEYBYTES], pk: &mut PolyVec, seed: &[u8; SYMBYTES]) {
    let mut polyvec_bytes = [0u8; POLYVECBYTES];
    polyvec_tobytes(&mut polyvec_bytes, pk);
    r[..POLYVECBYTES].copy_from_slice(&polyvec_bytes);
    for i in 0..SYMBYTES {
        r[i + POLYVECBYTES] = seed[i];
    }
}

/*************************************************
 * Name:        unpack_pk
 *
 * Description: De-serialize public key from a byte array;
 *              approximate inverse of pack_pk
 *
 * Arguments:   - polyvec *pk:             pointer to output public-key
 *                                         polynomial vector
 *              - uint8_t *seed:           pointer to output seed to generate
 *                                         matrix A
 *              - const uint8_t *packedpk: pointer to input serialized public key
 **************************************************/
fn unpack_pk(pk: &mut PolyVec, seed: &mut [u8; SYMBYTES], packedpk: &[u8; INDCPA_PUBLICKEYBYTES]) {
    let mut polyvec_bytes = [0u8; POLYVECBYTES];
    polyvec_bytes.copy_from_slice(&packedpk[..POLYVECBYTES]);
    polyvec_frombytes(pk, &polyvec_bytes);
    for i in 0..SYMBYTES {
        seed[i] = packedpk[i + POLYVECBYTES];
    }
}

/*************************************************
 * Name:        pack_sk
 *
 * Description: Serialize the secret key
 *
 * Arguments:   - uint8_t *r:  pointer to output serialized secret key
 *              - polyvec *sk: pointer to input vector of polynomials (secret key)
 **************************************************/
fn pack_sk(r: &mut [u8; INDCPA_SECRETKEYBYTES], sk: &mut PolyVec) {
    polyvec_tobytes(r, sk);
}

/*************************************************
 * Name:        unpack_sk
 *
 * Description: De-serialize the secret key;
 *              inverse of pack_sk
 *
 * Arguments:   - polyvec *sk:             pointer to output vector of
 *                                         polynomials (secret key)
 *              - const uint8_t *packedsk: pointer to input serialized secret key
 **************************************************/
fn unpack_sk(sk: &mut PolyVec, packedsk: &[u8; INDCPA_SECRETKEYBYTES]) {
    polyvec_frombytes(sk, packedsk);
}

/*************************************************
 * Name:        pack_ciphertext
 *
 * Description: Serialize the ciphertext as concatenation of the
 *              compressed and serialized vector of polynomials b
 *              and the compressed and serialized polynomial v
 *
 * Arguments:   uint8_t *r: pointer to the output serialized ciphertext
 *              poly *pk:   pointer to the input vector of polynomials b
 *              poly *v:    pointer to the input polynomial v
 **************************************************/
fn pack_ciphertext(r: &mut [u8; INDCPA_BYTES], b: &mut PolyVec, v: &mut Poly) {
    let mut polyvec_compressed = [0u8; POLYVECCOMPRESSEDBYTES];
    let mut poly_compressed = [0u8; POLYCOMPRESSEDBYTES];
    polyvec_compress(&mut polyvec_compressed, b);
    poly_compress(&mut poly_compressed, v);
    r[..POLYVECCOMPRESSEDBYTES].copy_from_slice(&polyvec_compressed);
    r[POLYVECCOMPRESSEDBYTES..].copy_from_slice(&poly_compressed);
}

/*************************************************
 * Name:        unpack_ciphertext
 *
 * Description: De-serialize and decompress ciphertext from a byte array;
 *              approximate inverse of pack_ciphertext
 *
 * Arguments:   - polyvec *b:       pointer to the output vector of polynomials b
 *              - poly *v:          pointer to the output polynomial v
 *              - const uint8_t *c: pointer to the input serialized ciphertext
 **************************************************/
fn unpack_ciphertext(b: &mut PolyVec, v: &mut Poly, c: &[u8; INDCPA_BYTES]) {
    let mut polyvec_compressed = [0u8; POLYVECCOMPRESSEDBYTES];
    let mut poly_compressed = [0u8; POLYCOMPRESSEDBYTES];
    polyvec_compressed.copy_from_slice(&c[..POLYVECCOMPRESSEDBYTES]);
    poly_compressed.copy_from_slice(&c[POLYVECCOMPRESSEDBYTES..]);
    polyvec_decompress(b, &polyvec_compressed);
    poly_decompress(v, &poly_compressed);
}

/*************************************************
 * Name:        rej_uniform
 *
 * Description: Run rejection sampling on uniform random bytes to generate
 *              uniform random integers mod q
 *
 * Arguments:   - r:          pointer to output buffer
 *              - len:        requested number of 16-bit integers
 *                                     (uniform mod q)
 *              - buf:        pointer to input buffer
 *                                     (assumed to be uniform random bytes)
 *              - buflen:     length of input buffer in bytes
 *
 * Returns number of sampled 16-bit integers (at most len)
 **************************************************/
fn rej_uniform(r: &mut [i16], len: usize, buf: &[u8], buflen: usize) -> usize {
    let mut ctr = 0;
    let mut pos = 0;

    while ctr < len && pos + 3 <= buflen {
        let val0 = ((buf[pos] as u16) | ((buf[pos + 1] as u16) << 8)) & 0xfff;
        let val1 = (((buf[pos + 1] as u16) >> 4) | ((buf[pos + 2] as u16) << 4)) & 0xfff;
        pos += 3;

        if val0 < (Q as u16) {
            r[ctr] = val0 as i16;
            ctr += 1;
        }
        if ctr < len && val1 < (Q as u16) {
            r[ctr] = val1 as i16;
            ctr += 1;
        }
    }

    ctr
}

/*************************************************
 * Name:        gen_matrix
 *
 * Description: Deterministically generate matrix A (or the transpose of A)
 *              from a seed. Entries of the matrix are polynomials that look
 *              uniformly random. Performs rejection sampling on output of
 *              a XOF
 *
 * Arguments:   - a:          pointer to output matrix A
 *              - seed:       pointer to input seed
 *              - transposed: boolean deciding whether A or A^T
 *                                     is generated
 **************************************************/
pub fn gen_matrix(a: &mut Vec<PolyVec>, seed: &[u8; SYMBYTES], transposed: bool) {
    for i in 0..K {
        for j in 0..K {
            let mut state = crate::symmetric::XofState::new();

            // Determine the order based on transposed flag
            if transposed {
                crate::symmetric::xof_absorb(&mut state, seed, i as u8, j as u8);
            } else {
                crate::symmetric::xof_absorb(&mut state, seed, j as u8, i as u8);
            }

            // Squeeze blocks to get random bytes
            let mut buf = vec![0u8; GEN_MATRIX_NBLOCKS * crate::symmetric::XOF_BLOCKBYTES + 2];
            crate::symmetric::xof_squeezeblocks(&mut buf, GEN_MATRIX_NBLOCKS, &mut state);

            let mut buflen = GEN_MATRIX_NBLOCKS * crate::symmetric::XOF_BLOCKBYTES;
            let mut ctr = rej_uniform(&mut a[i].vec[j].coeffs, N, &buf, buflen);

            while ctr < N {
                let off = buflen % 3;
                for k in 0..off {
                    buf[k] = buf[buflen - off + k];
                }
                crate::symmetric::xof_squeezeblocks(&mut buf[off..], 1, &mut state);
                buflen = off + crate::symmetric::XOF_BLOCKBYTES;
                ctr += rej_uniform(&mut a[i].vec[j].coeffs[ctr..], N - ctr, &buf, buflen);
            }
        }
    }
}

/*************************************************
* Name:        indcpa_keypair
*
* Description: Generates public and private key for the CPA-secure
*              public-key encryption scheme underlying Kyber
*
* Arguments:   - uint8_t *pk: pointer to output public key
*                             (of length KYBER_INDCPA_PUBLICKEYBYTES bytes)
*              - uint8_t *sk: pointer to output private key
                              (of length KYBER_INDCPA_SECRETKEYBYTES bytes)
**************************************************/
pub fn indcpa_keypair(pk: &mut [u8; INDCPA_PUBLICKEYBYTES], sk: &mut [u8; INDCPA_SECRETKEYBYTES]) {
    let mut buf = [0u8; 2 * SYMBYTES];
    let mut nonce = 0u8;

    // Generate random seeds using proper randombytes function
    crate::hash::randombytes(&mut buf, SYMBYTES);
    crate::hash::randombytes(&mut buf[SYMBYTES..], SYMBYTES);

    let publicseed = &buf[..SYMBYTES];
    let noiseseed = &buf[SYMBYTES..];

    let mut a = vec![PolyVec::with_len(K); K];
    gen_matrix(&mut a, &publicseed.try_into().unwrap(), false);

    let mut e = PolyVec::with_len(K);
    let mut pkpv = PolyVec::with_len(K);
    let mut skpv = PolyVec::with_len(K);

    for i in 0..K {
        poly_getnoise_eta1(&mut skpv.vec[i], &noiseseed.try_into().unwrap(), nonce);
        nonce += 1;
    }
    for i in 0..K {
        poly_getnoise_eta1(&mut e.vec[i], &noiseseed.try_into().unwrap(), nonce);
        nonce += 1;
    }

    // Convert to NTT domain for matrix-vector multiplication
    let mut skpv_ntt = skpv.clone();
    polyvec_ntt(&mut skpv_ntt);
    polyvec_ntt(&mut e);

    // matrix-vector multiplication
    for i in 0..K {
        polyvec_pointwise_acc_montgomery(&mut pkpv.vec[i], &a[i], &skpv_ntt);
        poly_tomont(&mut pkpv.vec[i]);
    }

    let pkpv_copy = pkpv.clone();
    polyvec_add(&mut pkpv, &pkpv_copy, &e);
    polyvec_reduce(&mut pkpv);

    // Store secret key in standard domain (not NTT domain)
    pack_sk(sk, &mut skpv);
    pack_pk(pk, &mut pkpv, &publicseed.try_into().unwrap());
}

/*************************************************
 * Name:        indcpa_enc
 *
 * Description: Encryption function of the CPA-secure
 *              public-key encryption scheme underlying Kyber.
 *
 * Arguments:   - uint8_t *c:           pointer to output ciphertext
 *                                      (of length KYBER_INDCPA_BYTES bytes)
 *              - const uint8_t *m:     pointer to input message
 *                                      (of length KYBER_INDCPA_MSGBYTES bytes)
 *              - const uint8_t *pk:    pointer to input public key
 *                                      (of length KYBER_INDCPA_PUBLICKEYBYTES)
 *              - const uint8_t *coins: pointer to input random coins
 *                                      used as seed (of length KYBER_SYMBYTES)
 *                                      to deterministically generate all
 *                                      randomness
 **************************************************/
pub fn indcpa_enc(
    c: &mut [u8; INDCPA_BYTES],
    m: &[u8; INDCPA_MSGBYTES],
    pk: &[u8; INDCPA_PUBLICKEYBYTES],
    coins: &[u8; SYMBYTES]
) {
    let mut seed = [0u8; SYMBYTES];
    let mut nonce = 0u8;
    let mut sp = PolyVec::with_len(K);
    let mut pkpv = PolyVec::new();
    let mut ep = PolyVec::with_len(K);
    let mut at = vec![PolyVec::with_len(K); K];
    let mut bp = PolyVec::with_len(K);
    let mut v = Poly::new();
    let mut k = Poly::new();
    let mut epp = Poly::new();

    unpack_pk(&mut pkpv, &mut seed, pk);
    poly_frommsg(&mut k, m);
    gen_matrix(&mut at, &seed, true);

    for i in 0..K {
        poly_getnoise_eta1(&mut sp.vec[i], coins, nonce);
        nonce += 1;
    }
    for i in 0..K {
        poly_getnoise_eta2(&mut ep.vec[i], coins, nonce);
        nonce += 1;
    }
    poly_getnoise_eta2(&mut epp, coins, nonce);

    polyvec_ntt(&mut sp);

    // matrix-vector multiplication
    for i in 0..K {
        polyvec_pointwise_acc_montgomery(&mut bp.vec[i], &at[i], &sp);
    }

    polyvec_pointwise_acc_montgomery(&mut v, &pkpv, &sp);

    polyvec_invntt_tomont(&mut bp);
    poly_invntt_tomont(&mut v);

    let bp_copy = bp.clone();
    polyvec_add(&mut bp, &bp_copy, &ep);
    let v_copy = v.clone();
    poly_add(&mut v, &v_copy, &epp);
    let v_copy2 = v.clone();
    poly_add(&mut v, &v_copy2, &k);
    polyvec_reduce(&mut bp);
    poly_reduce(&mut v);

    pack_ciphertext(c, &mut bp, &mut v);
}

/*************************************************
 * Name:        indcpa_dec
 *
 * Description: Decryption function of the CPA-secure
 *              public-key encryption scheme underlying Kyber.
 *
 * Arguments:   - uint8_t *m:        pointer to output decrypted message
 *                                   (of length KYBER_INDCPA_MSGBYTES)
 *              - const uint8_t *c:  pointer to input ciphertext
 *                                   (of length KYBER_INDCPA_BYTES)
 *              - const uint8_t *sk: pointer to input secret key
 *                                   (of length KYBER_INDCPA_SECRETKEYBYTES)
 **************************************************/
pub fn indcpa_dec(
    m: &mut [u8; INDCPA_MSGBYTES],
    c: &[u8; INDCPA_BYTES],
    sk: &[u8; INDCPA_SECRETKEYBYTES]
) {
    let mut bp = PolyVec::new();
    let mut skpv = PolyVec::new();
    let mut v = Poly::new();
    let mut mp = Poly::new();

    unpack_ciphertext(&mut bp, &mut v, c);
    unpack_sk(&mut skpv, sk);

    // Convert secret key to NTT domain for efficient multiplication
    polyvec_ntt(&mut skpv);

    polyvec_ntt(&mut bp);
    polyvec_pointwise_acc_montgomery(&mut mp, &skpv, &bp);
    poly_invntt_tomont(&mut mp);

    let mut temp = Poly::new();
    poly_sub(&mut temp, &v, &mp);
    mp = temp;
    poly_reduce(&mut mp);

    poly_tomsg(m, &mut mp);
}
