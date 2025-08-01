// tests/runner.rs

fn main() {
    // Use std::process::Command to invoke cargo test with the current environment,
    // so the increased stack size applies for the spawned test thread only.

    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8 MB stack
        .spawn(|| {
            // Run tests programmatically by invoking cargo test CLI command
            let status = std::process::Command::new("cargo")
                .args(&["test", "--", "--nocapture"])
                .status()
                .expect("Failed to run tests");

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
