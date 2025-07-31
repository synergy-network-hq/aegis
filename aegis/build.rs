use std::{env, fs};
use std::path::PathBuf;

fn main() {
    // 1) Find the workspace root (one level above this crate)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .expect("aegis/build.rs must live one level inside workspace")
        .to_path_buf();
    let pqcrypto_root = workspace_root.join("pqcrypto");

    // 2) Which pqcrypto crates + variants we want to compile
    let implementations: &[(&str, &[&str])] = &[
        ("pqcrypto-mlkem", &["ml-kem-512", "ml-kem-768", "ml-kem-1024"]),
        ("pqcrypto-mldsa", &["ml-dsa-44", "ml-dsa-65", "ml-dsa-87"]),
        (
            "pqcrypto-sphincsplus",
            &[
                "sphincs-sha2-128f-simple", "sphincs-sha2-128s-simple",
                "sphincs-sha2-192f-simple", "sphincs-sha2-192s-simple",
                "sphincs-sha2-256f-simple", "sphincs-sha2-256s-simple",
                "sphincs-shake-128f-simple", "sphincs-shake-128s-simple",
                "sphincs-shake-192f-simple", "sphincs-shake-192s-simple",
                "sphincs-shake-256f-simple", "sphincs-shake-256s-simple",
            ],
        ),
        ("pqcrypto-falcon", &["falcon-512", "falcon-1024"]),
        ("pqcrypto-hqc", &["hqc-128", "hqc-192", "hqc-256"]),
        (
            "pqcrypto-classicmceliece",
            &[
                "mceliece348864", "mceliece348864f", "mceliece460896",
                "mceliece460896f", "mceliece6688128", "mceliece6688128f",
                "mceliece6960119", "mceliece6960119f", "mceliece8192128",
                "mceliece8192128f",
            ],
        ),
    ];

    // 3) For each crate & variant: scan for .c files, then compile into a static lib
    for (crate_name, variants) in implementations {
        let crate_root = pqcrypto_root.join(crate_name);
        // PQClean "common" headers live here
        let common_dir = crate_root.join("pqclean").join("common");

        for variant in *variants {
            let clean_dir = crate_root
                .join("pqclean")
                .join(variant)
                .join("clean");

            // Re-run build.rs if anything in this clean subdir changes
            println!("cargo:rerun-if-changed={}", clean_dir.display());
            println!("cargo:warning=🔍 scanning for C files in {}", clean_dir.display());

            let mut c_files = Vec::new();
            if clean_dir.exists() {
                for entry in fs::read_dir(&clean_dir).unwrap() {
                    let path = entry.unwrap().path();
                    if path.extension().and_then(|e| e.to_str()) == Some("c") {
                        println!("cargo:warning=  • found C source: {}", path.display());
                        c_files.push(path);
                    }
                }
            } else {
                println!("cargo:warning=⚠ clean dir not found: {}", clean_dir.display());
            }

            if !c_files.is_empty() {
                let libname = format!("lib{}_clean.a", variant.replace('-', "_"));
                println!("cargo:warning=🔨 compiling {} into {}", variant, libname);

                // Start the cc::Build
                let mut build = cc::Build::new();
                build
                    .files(&c_files)
                    .include(&clean_dir)   // for headers next to your .c
                    .flag_if_supported("-O3");

                // ALSO add the common/ dir so fips202.h, randombytes.h, compat.h, etc are found
                if common_dir.exists() {
                    println!("cargo:warning=  • adding include path: {}", common_dir.display());
                    build.include(&common_dir);
                } else {
                    println!("cargo:warning=  • common dir not found: {}", common_dir.display());
                }

                build.compile(&libname);
            }
        }
    }
}
