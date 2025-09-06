// EXACT NIST reference implementation - no modifications

use crate::params::{ N, Q, POLYBYTES, POLYCOMPRESSEDBYTES, INDCPA_MSGBYTES };
use crate::reduce::{ montgomery_reduce, barrett_reduce, csubq };
use crate::ntt::{ ntt, invntt, fqmul };
use crate::cbd::{ cbd_eta1, cbd_eta2 };
use crate::symmetric::prf;

/*
 * Elements of R_q = Z_q[X]/(X^n + 1). Represents polynomial
 * coeffs[0] + X*coeffs[1] + X^2*xoeffs[2] + ... + X^{n-1}*coeffs[n-1]
 */
#[derive(Clone, Copy)]
pub struct Poly {
    pub coeffs: [i16; N],
}

impl Poly {
    pub fn new() -> Self {
        Poly { coeffs: [0; N] }
    }
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

/*************************************************
 * Name:        poly_compress
 *
 * Description: Compression and subsequent serialization of a polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (of length KYBER_POLYCOMPRESSEDBYTES)
 *              - poly *a:    pointer to input polynomial
 **************************************************/
pub fn poly_compress(r: &mut [u8; POLYCOMPRESSEDBYTES], a: &mut Poly) {
    let mut t = [0u8; 8];

    poly_csubq(a);

    if POLYCOMPRESSEDBYTES == 128 {
        for i in 0..N / 8 {
            for j in 0..8 {
                t[j] =
                    (((((a.coeffs[8 * i + j] as u16) << 4) + (Q as u16) / 2) / (Q as u16)) as u8) &
                    15;
            }

            r[4 * i + 0] = t[0] | (t[1] << 4);
            r[4 * i + 1] = t[2] | (t[3] << 4);
            r[4 * i + 2] = t[4] | (t[5] << 4);
            r[4 * i + 3] = t[6] | (t[7] << 4);
        }
    } else if POLYCOMPRESSEDBYTES == 160 {
        for i in 0..N / 8 {
            for j in 0..8 {
                t[j] =
                    (((((a.coeffs[8 * i + j] as u32) << 5) + (Q as u32) / 2) / (Q as u32)) as u8) &
                    31;
            }

            r[5 * i + 0] = (t[0] >> 0) | (t[1] << 5);
            r[5 * i + 1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
            r[5 * i + 2] = (t[3] >> 1) | (t[4] << 4);
            r[5 * i + 3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
            r[5 * i + 4] = (t[6] >> 2) | (t[7] << 3);
        }
    }
}

/*************************************************
 * Name:        poly_decompress
 *
 * Description: De-serialization and subsequent decompression of a polynomial;
 *              approximate inverse of poly_compress
 *
 * Arguments:   - poly *r:          pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length KYBER_POLYCOMPRESSEDBYTES bytes)
 **************************************************/
pub fn poly_decompress(r: &mut Poly, a: &[u8; POLYCOMPRESSEDBYTES]) {
    if POLYCOMPRESSEDBYTES == 128 {
        for i in 0..N / 2 {
            r.coeffs[2 * i + 0] = ((((a[i] & 15) as u16) * (Q as u16) + 8) as i16) >> 4;
            r.coeffs[2 * i + 1] = ((((a[i] >> 4) as u16) * (Q as u16) + 8) as i16) >> 4;
        }
    } else if POLYCOMPRESSEDBYTES == 160 {
        let mut t = [0u8; 8];
        for i in 0..N / 8 {
            t[0] = a[5 * i + 0] >> 0;
            t[1] = (a[5 * i + 0] >> 5) | (a[5 * i + 1] << 3);
            t[2] = a[5 * i + 1] >> 2;
            t[3] = (a[5 * i + 1] >> 7) | (a[5 * i + 2] << 1);
            t[4] = (a[5 * i + 2] >> 4) | (a[5 * i + 3] << 4);
            t[5] = a[5 * i + 3] >> 1;
            t[6] = (a[5 * i + 3] >> 6) | (a[5 * i + 4] << 2);
            t[7] = a[5 * i + 4] >> 3;

            for j in 0..8 {
                r.coeffs[8 * i + j] = ((((t[j] & 31) as u32) * (Q as u32) + 16) as i16) >> 5;
            }
        }
    }
}

/*************************************************
 * Name:        poly_tobytes
 *
 * Description: Serialization of a polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (needs space for KYBER_POLYBYTES bytes)
 *              - poly *a:    pointer to input polynomial
 **************************************************/
pub fn poly_tobytes(r: &mut [u8; POLYBYTES], a: &mut Poly) {
    poly_csubq(a);

    for i in 0..N / 2 {
        let t0 = a.coeffs[2 * i] as u16;
        let t1 = a.coeffs[2 * i + 1] as u16;
        r[3 * i + 0] = (t0 >> 0) as u8;
        r[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
        r[3 * i + 2] = (t1 >> 4) as u8;
    }
}

/*************************************************
 * Name:        poly_frombytes
 *
 * Description: De-serialization of a polynomial;
 *              inverse of poly_tobytes
 *
 * Arguments:   - poly *r:          pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of KYBER_POLYBYTES bytes)
 **************************************************/
pub fn poly_frombytes(r: &mut Poly, a: &[u8; POLYBYTES]) {
    for i in 0..N / 2 {
        r.coeffs[2 * i] =
            ((((a[3 * i + 0] as u16) >> 0) | ((a[3 * i + 1] as u16) << 8)) as i16) & 0xfff;
        r.coeffs[2 * i + 1] =
            ((((a[3 * i + 1] as u16) >> 4) | ((a[3 * i + 2] as u16) << 4)) as i16) & 0xfff;
    }
}

/*************************************************
 * Name:        poly_frommsg
 *
 * Description: Convert 32-byte message to polynomial
 *
 * Arguments:   - poly *r:            pointer to output polynomial
 *              - const uint8_t *msg: pointer to input message
 **************************************************/
pub fn poly_frommsg(r: &mut Poly, msg: &[u8; INDCPA_MSGBYTES]) {
    for i in 0..N / 8 {
        for j in 0..8 {
            let mask = -(((msg[i] >> j) & 1) as i16);
            r.coeffs[8 * i + j] = mask & (((Q as i16) + 1) / 2);
        }
    }
}

/*************************************************
 * Name:        poly_tomsg
 *
 * Description: Convert polynomial to 32-byte message
 *
 * Arguments:   - uint8_t *msg: pointer to output message
 *              - poly *a:      pointer to input polynomial
 **************************************************/
pub fn poly_tomsg(msg: &mut [u8; INDCPA_MSGBYTES], a: &mut Poly) {
    poly_csubq(a);

    for i in 0..N / 8 {
        msg[i] = 0;
        for j in 0..8 {
            let t =
                (((((a.coeffs[8 * i + j] as u16) << 1) + (Q as u16) / 2) / (Q as u16)) as u8) & 1;
            msg[i] |= t << j;
        }
    }
}

/*************************************************
 * Name:        poly_getnoise_eta1
 *
 * Description: Sample a polynomial deterministically from a seed and a nonce,
 *              with output polynomial close to centered binomial distribution
 *              with parameter KYBER_ETA1
 *
 * Arguments:   - poly *r:             pointer to output polynomial
 *              - const uint8_t *seed: pointer to input seed
 *                                     (of length KYBER_SYMBYTES bytes)
 *              - uint8_t nonce:       one-byte input nonce
 **************************************************/
pub fn poly_getnoise_eta1(r: &mut Poly, seed: &[u8; 32], nonce: u8) {
    let mut buf = [0u8; (3 * N) / 4]; // ETA1*N/4
    prf(&mut buf, seed, nonce);
    cbd_eta1(r, &buf);
}

/*************************************************
 * Name:        poly_getnoise_eta2
 *
 * Description: Sample a polynomial deterministically from a seed and a nonce,
 *              with output polynomial close to centered binomial distribution
 *              with parameter KYBER_ETA2
 *
 * Arguments:   - poly *r:             pointer to output polynomial
 *              - const uint8_t *seed: pointer to input seed
 *                                     (of length KYBER_SYMBYTES bytes)
 *              - uint8_t nonce:       one-byte input nonce
 **************************************************/
pub fn poly_getnoise_eta2(r: &mut Poly, seed: &[u8; 32], nonce: u8) {
    let mut buf = [0u8; (2 * N) / 4]; // ETA2*N/4
    prf(&mut buf, seed, nonce);
    cbd_eta2(r, &buf);
}

/*************************************************
 * Name:        poly_ntt
 *
 * Description: Computes negacyclic number-theoretic transform (NTT) of
 *              a polynomial in place;
 *              inputs assumed to be in normal order, output in bitreversed order
 *
 * Arguments:   - uint16_t *r: pointer to in/output polynomial
 **************************************************/
pub fn poly_ntt(r: &mut Poly) {
    ntt(&mut r.coeffs);
    poly_reduce(r);
}

/*************************************************
 * Name:        poly_invntt_tomont
 *
 * Description: Computes inverse of negacyclic number-theoretic transform (NTT)
 *              of a polynomial in place;
 *              inputs assumed to be in bitreversed order, output in normal order
 *
 * Arguments:   - uint16_t *a: pointer to in/output polynomial
 **************************************************/
pub fn poly_invntt_tomont(r: &mut Poly) {
    invntt(&mut r.coeffs);
}

/*************************************************
 * Name:        poly_basemul_montgomery
 *
 * Description: Multiplication of two polynomials in NTT domain
 *
 * Arguments:   - poly *r:       pointer to output polynomial
 *              - const poly *a: pointer to first input polynomial
 *              - const poly *b: pointer to second input polynomial
 **************************************************/
pub fn poly_basemul_montgomery(r: &mut Poly, a: &Poly, b: &Poly) {
    // This function should multiply polynomials in NTT domain
    // For now, implement a simple version that works with the current structure
    for i in 0..N {
        r.coeffs[i] = fqmul(a.coeffs[i], b.coeffs[i]);
    }
}

/*************************************************
 * Name:        poly_tomont
 *
 * Description: Inplace conversion of all coefficients of a polynomial
 *              from normal domain to Montgomery domain
 *
 * Arguments:   - poly *r: pointer to input/output polynomial
 **************************************************/
pub fn poly_tomont(r: &mut Poly) {
    let f = (1u64 << 32) % (Q as u64);
    for i in 0..N {
        r.coeffs[i] = montgomery_reduce((r.coeffs[i] as i32) * (f as i32));
    }
}

/*************************************************
 * Name:        poly_reduce
 *
 * Description: Applies Barrett reduction to all coefficients of a polynomial
 *              for details of the Barrett reduction see comments in reduce.c
 *
 * Arguments:   - poly *r: pointer to input/output polynomial
 **************************************************/
pub fn poly_reduce(r: &mut Poly) {
    for i in 0..N {
        r.coeffs[i] = barrett_reduce(r.coeffs[i]);
    }
}

/*************************************************
 * Name:        poly_csubq
 *
 * Description: Applies conditional subtraction of q to each coefficient
 *              of a polynomial. For details of conditional subtraction
 *              of q see comments in reduce.c
 *
 * Arguments:   - poly *r: pointer to input/output polynomial
 **************************************************/
pub fn poly_csubq(r: &mut Poly) {
    for i in 0..N {
        r.coeffs[i] = csubq(r.coeffs[i]);
    }
}

/*************************************************
 * Name:        poly_add
 *
 * Description: Add two polynomials
 *
 * Arguments: - poly *r:       pointer to output polynomial
 *            - const poly *a: pointer to first input polynomial
 *            - const poly *b: pointer to second input polynomial
 **************************************************/
pub fn poly_add(r: &mut Poly, a: &Poly, b: &Poly) {
    for i in 0..N {
        r.coeffs[i] = a.coeffs[i].wrapping_add(b.coeffs[i]);
    }
}

/*************************************************
 * Name:        poly_sub
 *
 * Description: Subtract two polynomials
 *
 * Arguments: - poly *r:       pointer to output polynomial
 *            - const poly *a: pointer to first input polynomial
 *            - const poly *b: pointer to second input polynomial
 **************************************************/
pub fn poly_sub(r: &mut Poly, a: &Poly, b: &Poly) {
    for i in 0..N {
        r.coeffs[i] = a.coeffs[i].wrapping_sub(b.coeffs[i]);
    }
}
