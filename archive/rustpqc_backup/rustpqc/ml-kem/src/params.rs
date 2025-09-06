// EXACT NIST reference implementation - no modifications

pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; /* size in bytes of hashes, and seeds */
pub const SSBYTES: usize = 32; /* size in bytes of shared key */

// Parameter sets
pub const K: usize = 3; // KYBER_K = 3 for ML-KEM-768

pub const POLYBYTES: usize = 384;
pub const POLYVECBYTES: usize = K * POLYBYTES;

pub const ETA1: usize = 2; // For ML-KEM-768 (KYBER_K = 3)
pub const POLYCOMPRESSEDBYTES: usize = 128;
pub const POLYVECCOMPRESSEDBYTES: usize = K * 320; // 3 * 320 = 960

pub const ETA2: usize = 2;

pub const INDCPA_MSGBYTES: usize = SYMBYTES;
pub const INDCPA_PUBLICKEYBYTES: usize = POLYVECBYTES + SYMBYTES;
pub const INDCPA_SECRETKEYBYTES: usize = POLYVECBYTES;
pub const INDCPA_BYTES: usize = POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;

pub const PUBLICKEYBYTES: usize = INDCPA_PUBLICKEYBYTES;
/* 32 bytes of additional space to save H(pk) */
pub const SECRETKEYBYTES: usize = INDCPA_SECRETKEYBYTES + INDCPA_PUBLICKEYBYTES + 2 * SYMBYTES;
pub const CIPHERTEXTBYTES: usize = INDCPA_BYTES;

pub struct ParameterSet {
    pub k: usize,
    pub eta1: usize,
    pub eta2: usize,
    pub polybytes: usize,
    pub polyvecbytes: usize,
    pub polycompressedbytes: usize,
    pub polyveccompressedbytes: usize,
    pub indcpa_publickeybytes: usize,
    pub indcpa_secretkeybytes: usize,
    pub indcpa_bytes: usize,
    pub publickeybytes: usize,
    pub secretkeybytes: usize,
    pub ciphertextbytes: usize,
}

pub const MLKEM_768: ParameterSet = ParameterSet {
    k: K,
    eta1: ETA1,
    eta2: ETA2,
    polybytes: POLYBYTES,
    polyvecbytes: POLYVECBYTES,
    polycompressedbytes: POLYCOMPRESSEDBYTES,
    polyveccompressedbytes: POLYVECCOMPRESSEDBYTES,
    indcpa_publickeybytes: INDCPA_PUBLICKEYBYTES,
    indcpa_secretkeybytes: INDCPA_SECRETKEYBYTES,
    indcpa_bytes: INDCPA_BYTES,
    publickeybytes: PUBLICKEYBYTES,
    secretkeybytes: SECRETKEYBYTES,
    ciphertextbytes: CIPHERTEXTBYTES,
};
