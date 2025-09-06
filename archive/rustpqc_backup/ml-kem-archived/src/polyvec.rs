// EXACT NIST reference implementation - no modifications

use crate::params::{ K, N, Q, POLYBYTES, POLYVECBYTES, POLYVECCOMPRESSEDBYTES };
use crate::poly::{
    Poly,
    poly_tobytes,
    poly_frombytes,
    poly_ntt,
    poly_invntt_tomont,
    poly_basemul_montgomery,
    poly_add,
    poly_reduce,
    poly_csubq,
};

#[derive(Clone)]
pub struct PolyVec {
    pub vec: Vec<Poly>,
}

impl PolyVec {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(K);
        for _ in 0..K {
            vec.push(Poly::new());
        }
        PolyVec { vec }
    }

    pub fn with_len(len: usize) -> Self {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(Poly::new());
        }
        PolyVec { vec }
    }
}

impl Default for PolyVec {
    fn default() -> Self {
        Self::new()
    }
}

/*************************************************
 * Name:        polyvec_compress
 *
 * Description: Compress and serialize vector of polynomials
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (needs space for KYBER_POLYVECCOMPRESSEDBYTES)
 *              - polyvec *a: pointer to input vector of polynomials
 **************************************************/
pub fn polyvec_compress(r: &mut [u8; POLYVECCOMPRESSEDBYTES], a: &mut PolyVec) {
    polyvec_csubq(a);

    if POLYVECCOMPRESSEDBYTES == K * 320 {
        let mut t = [0u16; 4];
        for i in 0..K {
            for j in 0..N / 4 {
                for k in 0..4 {
                    t[k] =
                        (
                            ((((a.vec[i].coeffs[4 * j + k] as u32) << 10) + (Q as u32) / 2) /
                                (Q as u32)) as u16
                        ) & 0x3ff;
                }

                let offset = ((i * N) / 4 + j) * 5;
                r[offset + 0] = (t[0] >> 0) as u8;
                r[offset + 1] = ((t[0] >> 8) | (t[1] << 2)) as u8;
                r[offset + 2] = ((t[1] >> 6) | (t[2] << 4)) as u8;
                r[offset + 3] = ((t[2] >> 4) | (t[3] << 6)) as u8;
                r[offset + 4] = (t[3] >> 2) as u8;
            }
        }
    } else {
        panic!("Unsupported POLYVECCOMPRESSEDBYTES value");
    }
}

/*************************************************
 * Name:        polyvec_decompress
 *
 * Description: De-serialize and decompress vector of polynomials;
 *              approximate inverse of polyvec_compress
 *
 * Arguments:   - polyvec *r:       pointer to output vector of polynomials
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length KYBER_POLYVECCOMPRESSEDBYTES)
 **************************************************/
pub fn polyvec_decompress(r: &mut PolyVec, a: &[u8; POLYVECCOMPRESSEDBYTES]) {
    if POLYVECCOMPRESSEDBYTES == K * 320 {
        let mut t = [0u16; 4];
        for i in 0..K {
            for j in 0..N / 4 {
                let offset = ((i * N) / 4 + j) * 5;
                t[0] = ((a[offset + 0] as u16) >> 0) | ((a[offset + 1] as u16) << 8);
                t[1] = ((a[offset + 1] as u16) >> 2) | ((a[offset + 2] as u16) << 6);
                t[2] = ((a[offset + 2] as u16) >> 4) | ((a[offset + 3] as u16) << 4);
                t[3] = ((a[offset + 3] as u16) >> 6) | ((a[offset + 4] as u16) << 2);

                for k in 0..4 {
                    r.vec[i].coeffs[4 * j + k] =
                        ((((t[k] & 0x3ff) as u32) * (Q as u32) + 512) as i16) >> 10;
                }
            }
        }
    } else {
        panic!("Unsupported POLYVECCOMPRESSEDBYTES value");
    }
}

/*************************************************
 * Name:        polyvec_tobytes
 *
 * Description: Serialize vector of polynomials
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (needs space for KYBER_POLYVECBYTES)
 *              - polyvec *a: pointer to input vector of polynomials
 **************************************************/
pub fn polyvec_tobytes(r: &mut [u8; POLYVECBYTES], a: &mut PolyVec) {
    for i in 0..K {
        let start = i * POLYBYTES;
        let end = (i + 1) * POLYBYTES;
        let slice = &mut r[start..end];
        let mut poly_bytes = [0u8; POLYBYTES];
        poly_tobytes(&mut poly_bytes, &mut a.vec[i]);
        slice.copy_from_slice(&poly_bytes);
    }
}

/*************************************************
 * Name:        polyvec_frombytes
 *
 * Description: De-serialize vector of polynomials;
 *              inverse of polyvec_tobytes
 *
 * Arguments:   - uint8_t *r:       pointer to output byte array
 *              - const polyvec *a: pointer to input vector of polynomials
 *                                  (of length KYBER_POLYVECBYTES)
 **************************************************/
pub fn polyvec_frombytes(r: &mut PolyVec, a: &[u8; POLYVECBYTES]) {
    for i in 0..K {
        let start = i * POLYBYTES;
        let end = (i + 1) * POLYBYTES;
        let slice = &a[start..end];
        let mut poly_bytes = [0u8; POLYBYTES];
        poly_bytes.copy_from_slice(slice);
        poly_frombytes(&mut r.vec[i], &poly_bytes);
    }
}

/*************************************************
 * Name:        polyvec_ntt
 *
 * Description: Apply forward NTT to all elements of a vector of polynomials
 *
 * Arguments:   - polyvec *r: pointer to in/output vector of polynomials
 **************************************************/
pub fn polyvec_ntt(r: &mut PolyVec) {
    for i in 0..K {
        poly_ntt(&mut r.vec[i]);
    }
}

/*************************************************
 * Name:        polyvec_invntt_tomont
 *
 * Description: Apply inverse NTT to all elements of a vector of polynomials
 *              and multiply by Montgomery factor 2^16
 *
 * Arguments:   - polyvec *r: pointer to in/output vector of polynomials
 **************************************************/
pub fn polyvec_invntt_tomont(r: &mut PolyVec) {
    for i in 0..K {
        poly_invntt_tomont(&mut r.vec[i]);
    }
}

/*************************************************
 * Name:        polyvec_pointwise_acc_montgomery
 *
 * Description: Pointwise multiply elements of a and b, accumulate into r,
 *              and multiply by 2^-16.
 *
 * Arguments: - poly *r:          pointer to output polynomial
 *            - const polyvec *a: pointer to first input vector of polynomials
 *            - const polyvec *b: pointer to second input vector of polynomials
 **************************************************/
pub fn polyvec_pointwise_acc_montgomery(r: &mut Poly, a: &PolyVec, b: &PolyVec) {
    poly_basemul_montgomery(r, &a.vec[0], &b.vec[0]);
    for i in 1..K {
        let mut t = Poly::new();
        poly_basemul_montgomery(&mut t, &a.vec[i], &b.vec[i]);
        let mut temp = Poly::new();
        poly_add(&mut temp, r, &t);
        *r = temp;
    }
    poly_reduce(r);
}

/*************************************************
 * Name:        polyvec_reduce
 *
 * Description: Applies Barrett reduction to each coefficient
 *              of each element of a vector of polynomials
 *              for details of the Barrett reduction see comments in reduce.c
 *
 * Arguments:   - poly *r: pointer to input/output polynomial
 **************************************************/
pub fn polyvec_reduce(r: &mut PolyVec) {
    for i in 0..K {
        poly_reduce(&mut r.vec[i]);
    }
}

/*************************************************
 * Name:        polyvec_csubq
 *
 * Description: Applies conditional subtraction of q to each coefficient
 *              of each element of a vector of polynomials
 *              for details of conditional subtraction of q see comments in
 *              reduce.c
 *
 * Arguments:   - poly *r: pointer to input/output polynomial
 **************************************************/
pub fn polyvec_csubq(r: &mut PolyVec) {
    for i in 0..K {
        poly_csubq(&mut r.vec[i]);
    }
}

/*************************************************
 * Name:        polyvec_add
 *
 * Description: Add vectors of polynomials
 *
 * Arguments: - polyvec *r:       pointer to output vector of polynomials
 *            - const polyvec *a: pointer to first input vector of polynomials
 *            - const polyvec *b: pointer to second input vector of polynomials
 **************************************************/
pub fn polyvec_add(r: &mut PolyVec, a: &PolyVec, b: &PolyVec) {
    for i in 0..K {
        poly_add(&mut r.vec[i], &a.vec[i], &b.vec[i]);
    }
}
