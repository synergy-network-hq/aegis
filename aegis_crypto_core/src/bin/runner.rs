fn main() {
    let stack_size = 8 * 1024 * 1024; // 8 MB

    let handle = std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            println!("Running ClassicMcEliece256 test with a big stack...");
            aegis_crypto_core::classicmceliece::core::test_classicmceliece256_keygen_encap_decap_demo();
            println!("Test passed!");
        })
        .unwrap();

    // Will propagate panic if test fails (e.g. assert fails)
    if let Err(e) = handle.join() {
        eprintln!("Test failed with panic: {e:?}");
        std::process::exit(1);
    }
}
