// Conditional wasm_bindgen macro
#[cfg(feature = "wasm")]
macro_rules! wasm_bindgen {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        $($tt)*
    };
}

#[cfg(not(feature = "wasm"))]
macro_rules! wasm_bindgen {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

#[cfg(feature = "kyber")]
pub mod kyber;
#[cfg(feature = "dilithium")]
pub mod dilithium;
#[cfg(feature = "falcon")]
pub mod falcon;
#[cfg(feature = "sphincsplus")]
pub mod sphincsplus;
#[cfg(feature = "hqc")]
pub mod hqc;
#[cfg(feature = "classicmceliece")]
pub mod classicmceliece;

// Pure Rust implementations from rustpqc folder
// #[cfg(feature = "rustpqc-kyber")]
// pub mod rustpqc_kyber;
// #[cfg(feature = "rustpqc-dilithium")]
// pub mod rustpqc_dilithium;

// NIST Reference WASM implementations
#[cfg(feature = "nist-wasm")]
pub mod nist_wasm_mlkem;
#[cfg(feature = "nist-wasm")]
pub mod nist_wasm_mldsa;
#[cfg(feature = "nist-wasm")]
pub mod wasm_loader;

/// Trait definitions for unified algorithm interfaces.
pub mod traits;

pub mod hash;
pub mod utils;
pub mod performance;
pub mod blockchain;

// The `js_bindings` module exposes a JavaScript‑friendly API on top of the
// low‑level functions.  It is compiled unconditionally when building the
// WebAssembly target so that its exports are available via `wasm-pack`.
pub mod js_bindings;

// The Python bindings are conditionally compiled when the
// `python-bindings` feature is enabled.  See `Cargo.toml` for more
// details.  The module contains PyO3 wrappers that expose the
// algorithms to Python as a native extension.
// #[cfg(feature = "python-bindings")]
// pub mod python_bindings;

#[cfg(feature = "kyber")]
pub use kyber::*;
#[cfg(feature = "dilithium")]
pub use dilithium::*;
#[cfg(feature = "falcon")]
pub use falcon::*;
#[cfg(feature = "sphincsplus")]
pub use sphincsplus::*;
#[cfg(feature = "hqc")]
pub use hqc::*;
#[cfg(feature = "classicmceliece")]
pub use classicmceliece::*;

// Re-export pure Rust implementations
// #[cfg(feature = "rustpqc-kyber")]
// pub use rustpqc_kyber::*;
// #[cfg(feature = "rustpqc-dilithium")]
// pub use rustpqc_dilithium::*;

// Re-export NIST Reference WASM implementations
#[cfg(feature = "nist-wasm")]
pub use nist_wasm_mlkem::*;
#[cfg(feature = "nist-wasm")]
pub use nist_wasm_mldsa::*;
#[cfg(feature = "nist-wasm")]
pub use wasm_loader::*;
