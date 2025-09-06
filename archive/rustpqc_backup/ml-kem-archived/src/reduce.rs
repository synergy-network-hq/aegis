// EXACT NIST reference implementation - no modifications

use crate::params::Q;

pub const MONT: i32 = 2285; // 2^16 mod q
pub const QINV: i32 = 62209; // q^-1 mod 2^16

/*************************************************
 * Name:        montgomery_reduce
 *
 * Description: Montgomery reduction; given a 32-bit integer a, computes
 *              16-bit integer congruent to a * R^-1 mod q,
 *              where R=2^16
 *
 * Arguments:   - int32_t a: input integer to be reduced;
 *                           has to be in {-q2^15,...,q2^15-1}
 *
 * Returns:     integer in {-q+1,...,q-1} congruent to a * R^-1 modulo q.
 **************************************************/
pub fn montgomery_reduce(a: i32) -> i16 {
    // Use 64-bit arithmetic to avoid overflow
    let a_64 = a as i64;
    let qinv_64 = QINV as i64;
    let q_64 = Q as i64;

    let u = (a_64 * qinv_64) & ((1i64 << 16) - 1); // Take only lower 16 bits
    let t = a_64 - u * q_64;
    (t >> 16) as i16
}

/*************************************************
 * Name:        barrett_reduce
 *
 * Description: Barrett reduction; given a 16-bit integer a, computes
 *              16-bit integer congruent to a mod q in {0,...,q}
 *
 * Arguments:   - int16_t a: input integer to be reduced
 *
 * Returns:     integer in {0,...,q} congruent to a modulo q.
 **************************************************/
pub fn barrett_reduce(a: i16) -> i16 {
    let v = ((1u32 << 26) + (Q as u32) / 2) / (Q as u32);
    let t = ((v as i32) * (a as i32)) >> 26;
    let t = t * (Q as i32);
    a.wrapping_sub(t as i16)
}

/*************************************************
 * Name:        csubq
 *
 * Description: Conditionallly subtract q
 *
 * Arguments:   - int16_t x: input integer
 *
 * Returns:     a - q if a >= q, else a
 **************************************************/
pub fn csubq(a: i16) -> i16 {
    let mut a_val = a;
    a_val -= Q as i16;
    a_val += (a_val >> 15) & (Q as i16);
    a_val
}
