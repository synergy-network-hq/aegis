// EXACT NIST reference implementation - no modifications

use crate::params::N;
use crate::reduce::{ montgomery_reduce, barrett_reduce };

const ZETAS: [i16; 128] = [
    2285, 2571, 2970, 1812, 1493, 1422, 287, 202, 3158, 622, 1577, 182, 962, 2127, 1855, 1468, 573, 2004,
    264, 383, 2500, 1458, 1727, 3199, 2648, 1017, 732, 608, 1787, 411, 3124, 1758, 1223, 652, 2777, 1015,
    2036, 1491, 3047, 1785, 516, 3321, 3009, 2663, 1711, 2167, 126, 1469, 2476, 3239, 3058, 830,
    107, 1908, 3082, 2378, 2931, 961, 1821, 2604, 448, 2264, 677, 2054, 2226, 430, 555, 843, 2078, 871,
    1550, 105, 422, 587, 177, 3094, 3038, 2869, 1574, 1653, 3083, 778, 1159, 3182, 2552, 1483,
    2727, 1119, 1739, 644, 2457, 349, 418, 329, 3173, 3254, 817, 1097, 603, 610, 1322, 2044, 1864, 384,
    2114, 3193, 1218, 1994, 2455, 220, 2142, 1670, 2144, 1799, 2051, 794, 1819, 2475, 2459, 478,
    3221, 3021, 996, 991, 958, 1869, 1522, 1628,
];

pub const ZETAS_INV: [i16; 128] = [
    1701, 1807, 1460, 2371, 2338, 2333, 308, 108, 2851, 870, 854, 1510, 2535, 1278, 1530, 1185,
    1659, 1187, 3109, 874, 1335, 2111, 136, 1215, 2945, 1465, 1285, 2007, 2719, 2726, 2232, 2512,
    75, 156, 3000, 2911, 2980, 872, 2685, 1590, 2210, 602, 1846, 777, 147, 2170, 2551, 246, 1676, 1755,
    460, 291, 235, 3152, 2742, 2907, 3224, 1779, 2458, 1251, 2486, 2774, 2899, 1103, 1275, 2652,
    1065, 2881, 725, 1508, 2368, 398, 951, 247, 1421, 3222, 2499, 271, 90, 853, 1860, 3203, 1162, 1618,
    666, 320, 8, 2813, 1544, 282, 1838, 1293, 2314, 552, 2677, 2106, 1571, 205, 2918, 1542, 2721, 2597,
    2312, 681, 130, 1602, 1871, 829, 2946, 3065, 1325, 2756, 1861, 1474, 1202, 2367, 3147, 1752,
    2707, 171, 3127, 3042, 1907, 1836, 1517, 359, 758, 1441,
];

/*************************************************
 * Name:        fqmul
 *
 * Description: Multiplication followed by Montgomery reduction
 *
 * Arguments:   - int16_t a: first factor
 *              - int16_t b: second factor
 *
 * Returns 16-bit integer congruent to a*b*R^{-1} mod q
 **************************************************/
pub fn fqmul(a: i16, b: i16) -> i16 {
    montgomery_reduce((a as i32) * (b as i32))
}

/*************************************************
 * Name:        ntt
 *
 * Description: Inplace number-theoretic transform (NTT) in Rq
 *              input is in standard order, output is in bitreversed order
 *
 * Arguments:   - int16_t r[256]: pointer to input/output vector of elements
 *                                of Zq
 **************************************************/
pub fn ntt(r: &mut [i16; N]) {
    let mut k = 1;
    let mut len = 128;
    while len >= 2 {
        let mut start = 0;
        while start < 256 {
            let zeta = ZETAS[k];
            k += 1;
            let mut j = start;
            while j < start + len {
                let t = fqmul(zeta, r[j + len]);
                r[j + len] = r[j].wrapping_sub(t);
                r[j] = r[j].wrapping_add(t);
                j += 1;
            }
            start = j + len;
        }
        len >>= 1;
    }
}

/*************************************************
 * Name:        invntt_tomont
 *
 * Description: Inplace inverse number-theoretic transform in Rq and
 *              multiplication by Montgomery factor 2^16.
 *              Input is in bitreversed order, output is in standard order
 *
 * Arguments:   - int16_t r[256]: pointer to input/output vector of elements
 *                                of Zq
 **************************************************/
pub fn invntt(r: &mut [i16; N]) {
    let mut k = 0;
    let mut len = 2;
    while len <= 128 {
        let mut start = 0;
        while start < 256 {
            let zeta = ZETAS_INV[k];
            k += 1;
            let mut j = start;
            while j < start + len {
                let t = r[j];
                r[j] = barrett_reduce(t.wrapping_add(r[j + len]));
                r[j + len] = t.wrapping_sub(r[j + len]);
                r[j + len] = fqmul(zeta, r[j + len]);
                j += 1;
            }
            start = j + len;
        }
        len <<= 1;
    }

    for j in 0..256 {
        r[j] = fqmul(r[j], ZETAS_INV[127]);
    }
}

/*************************************************
 * Name:        basemul
 *
 * Description: Multiplication of polynomials in Zq[X]/(X^2-zeta)
 *              used for multiplication of elements in Rq in NTT domain
 *
 * Arguments:   - int16_t r[2]:       pointer to the output polynomial
 *              - const int16_t a[2]: pointer to the first factor
 *              - int16_t zeta:       integer defining the reduction polynomial
 **************************************************/
pub fn basemul(r: &mut [i16], a: &[i16], b: &[i16], zeta: i16) {
    r[0] = fqmul(a[1], b[1]);
    r[0] = fqmul(r[0], zeta);
    r[0] = r[0].wrapping_add(fqmul(a[0], b[0]));

    r[1] = fqmul(a[0], b[1]);
    r[1] = r[1].wrapping_add(fqmul(a[1], b[0]));
}
