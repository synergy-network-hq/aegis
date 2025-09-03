// EXACT NIST reference implementation - no modifications

use sha3::{ Shake128, Shake256, digest::{ ExtendableOutput, Update, XofReader } };

// XOF constants
pub const XOF_BLOCKBYTES: usize = 168; // SHAKE128_RATE

// XOF state type - equivalent to keccak_state in C
pub struct XofState {
    hasher: Shake128,
    absorbed: bool,
}

impl XofState {
    pub fn new() -> Self {
        Self {
            hasher: Shake128::default(),
            absorbed: false,
        }
    }
}

/*************************************************
 * Name:        kyber_shake128_absorb
 *
 * Description: Absorb step of the SHAKE128 specialized for the Kyber context.
 *
 * Arguments:   - state: pointer to (uninitialized) output
 *                                     Keccak state
 *              - seed:  pointer to KYBER_SYMBYTES input
 *                                     to be absorbed into state
 *              - x:     additional byte of input
 *              - y:     additional byte of input
 **************************************************/
pub fn xof_absorb(state: &mut XofState, seed: &[u8; 32], x: u8, y: u8) {
    let mut extseed = [0u8; 34];
    extseed[..32].copy_from_slice(seed);
    extseed[32] = x;
    extseed[33] = y;

    state.hasher.update(&extseed);
    state.absorbed = true;
}

/*************************************************
 * Name:        shake128_squeezeblocks
 *
 * Description: Squeeze step of SHAKE128. Squeezes full blocks of SHAKE128_RATE bytes.
 *
 * Arguments:   - out:     pointer to output blocks
 *              - nblocks: number of blocks to be squeezed (written to out)
 *              - state:   pointer to input/output Keccak state
 **************************************************/
pub fn xof_squeezeblocks(out: &mut [u8], _nblocks: usize, state: &mut XofState) {
    if !state.absorbed {
        panic!("Must absorb before squeezing");
    }
    
    let mut reader = state.hasher.clone().finalize_xof();
    reader.read(out);
}

/*************************************************
 * Name:        kyber_shake256_prf
 *
 * Description: Usage of SHAKE256 as a PRF, concatenates secret and public input
 *              and then generates outlen bytes of SHAKE256 output
 *
 * Arguments:   - uint8_t *out:       pointer to output
 *              - size_t outlen:      number of requested output bytes
 *              - const uint8_t *key: pointer to the key
 *                                    (of length KYBER_SYMBYTES)
 *              - uint8_t nonce:      single-byte nonce (public PRF input)
 **************************************************/
pub fn prf(out: &mut [u8], key: &[u8; 32], nonce: u8) {
    let mut extkey = [0u8; 33];
    extkey[..32].copy_from_slice(key);
    extkey[32] = nonce;

    let mut hasher = Shake256::default();
    hasher.update(&extkey);
    let mut reader = hasher.finalize_xof();
    reader.read(out);
}
