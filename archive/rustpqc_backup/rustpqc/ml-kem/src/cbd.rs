// EXACT NIST reference implementation - no modifications

use crate::params::{ N, ETA1, ETA2 };
use crate::poly::Poly;

/*************************************************
 * Name:        load32_littleendian
 *
 * Description: load 4 bytes into a 32-bit integer
 *              in little-endian order
 *
 * Arguments:   - const uint8_t *x: pointer to input byte array
 *
 * Returns 32-bit unsigned integer loaded from x
 **************************************************/
fn load32_littleendian(x: &[u8; 4]) -> u32 {
    let mut r: u32 = 0;
    r |= x[0] as u32;
    r |= (x[1] as u32) << 8;
    r |= (x[2] as u32) << 16;
    r |= (x[3] as u32) << 24;
    r
}

/*************************************************
 * Name:        cbd2
 *
 * Description: Given an array of uniformly random bytes, compute
 *              polynomial with coefficients distributed according to
 *              a centered binomial distribution with parameter eta=2
 *
 * Arguments:   - poly *r:            pointer to output polynomial
 *              - const uint8_t *buf: pointer to input byte array
 **************************************************/
fn cbd2(r: &mut Poly, buf: &[u8]) {
    for i in 0..N / 8 {
        let t = load32_littleendian(&[buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]]);
        let mut d = t & 0x55555555;
        d += (t >> 1) & 0x55555555;

        for j in 0..8 {
            let a = (d >> (4 * j + 0)) & 0x3;
            let b = (d >> (4 * j + 2)) & 0x3;
            r.coeffs[8 * i + j] = ((a as i32) - (b as i32)) as i16;
        }
    }
}

pub fn cbd_eta1(r: &mut Poly, buf: &[u8]) {
    if ETA1 == 2 {
        cbd2(r, buf);
    } else {
        panic!("This implementation requires eta1 = 2 for ML-KEM-768");
    }
}

pub fn cbd_eta2(r: &mut Poly, buf: &[u8]) {
    if ETA2 != 2 {
        panic!("This implementation requires eta2 = 2");
    } else {
        cbd2(r, buf);
    }
}
