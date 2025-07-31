pub mod dilithium;
pub mod hash;
pub mod kyber;
pub mod falcon;
pub mod utils;

// The `js_bindings` module exposes a JavaScript‑friendly API on top of the
// low‑level functions.  It is compiled unconditionally when building the
// WebAssembly target so that its exports are available via `wasm-pack`.
pub mod js_bindings;

// The Python bindings are conditionally compiled when the
// `python-bindings` feature is enabled.  See `Cargo.toml` for more
// details.  The module contains PyO3 wrappers that expose the
// algorithms to Python as a native extension.
#[cfg(feature = "python-bindings")]
pub mod python_bindings;
// -----------------------------------------------------------------------------
// SPHINCS+ (FIPS 205) support
//
// The SPHINCS+ module has been temporarily disabled.  The upstream
// `pqc_sphincsplus` crate currently contains critical bugs that cause invalid
// signatures and failed verifications.  Until those issues are resolved we
// avoid compiling or re-exporting the SPHINCS+ module to prevent build
// failures and runtime panics.  Any code that previously relied on
// `sphincsplus_*` functions should be considered unsupported for the time
// being.  See the project README and update documents for more details.
//
// pub mod sphincsplus;

pub use dilithium::*;
pub use kyber::*;
pub use falcon::*;
// pub use sphincsplus::*;

// #[cfg(test)]
// mod sphincsplus_tests;


