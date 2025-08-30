use criterion::{ criterion_group, criterion_main, Criterion };
use std::hint::black_box;
use aegis_crypto_core::{
    // KEM algorithms
    kyber_keygen,
    kyber_encapsulate,
    kyber_decapsulate,
    hqc_keygen,
    hqc_encapsulate,
    hqc_decapsulate,
    classicmceliece_keygen,
    classicmceliece_encapsulate,
    classicmceliece_decapsulate,
    // Signature algorithms
    dilithium_keygen,
    dilithium_sign,
    dilithium_verify,
    falcon_keygen,
    falcon_sign,
    falcon_verify,
    sphincsplus_keygen,
    sphincsplus_sign,
    sphincsplus_verify,
};

fn bench_kyber_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kyber KEM Operations");

    // Key generation benchmark
    group.bench_function("kyber_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(kyber_keygen());
            black_box(keypair)
        })
    });

    // Encapsulation benchmark
    group.bench_function("kyber_encapsulate", |b| {
        let keypair = kyber_keygen();
        let public_key = keypair.public_key();
        b.iter(|| {
            let encapsulated = black_box(kyber_encapsulate(&public_key));
            black_box(encapsulated)
        })
    });

    // Decapsulation benchmark
    group.bench_function("kyber_decapsulate", |b| {
        let keypair = kyber_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation should succeed");
        let ciphertext = encapsulated.ciphertext();

        b.iter(|| {
            let decapsulated = black_box(kyber_decapsulate(&secret_key, &ciphertext));
            black_box(decapsulated)
        })
    });

    group.finish();
}

fn bench_hqc_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("HQC KEM Operations");

    // Key generation benchmark
    group.bench_function("hqc_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(hqc_keygen());
            black_box(keypair)
        })
    });

    // Encapsulation benchmark
    group.bench_function("hqc_encapsulate", |b| {
        let keypair = hqc_keygen();
        let public_key = keypair.public_key();
        b.iter(|| {
            let encapsulated = black_box(hqc_encapsulate(&public_key));
            black_box(encapsulated)
        })
    });

    // Decapsulation benchmark
    group.bench_function("hqc_decapsulate", |b| {
        let keypair = hqc_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation should succeed");
        let ciphertext = encapsulated.ciphertext();

        b.iter(|| {
            let decapsulated = black_box(hqc_decapsulate(&secret_key, &ciphertext));
            black_box(decapsulated)
        })
    });

    group.finish();
}

fn bench_classicmceliece_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Classic McEliece KEM Operations");

    // Key generation benchmark (with larger stack)
    group.bench_function("classicmceliece_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(classicmceliece_keygen());
            black_box(keypair)
        })
    });

    // Encapsulation benchmark
    group.bench_function("classicmceliece_encapsulate", |b| {
        let keypair = classicmceliece_keygen();
        let public_key = keypair.public_key();
        b.iter(|| {
            let encapsulated = black_box(classicmceliece_encapsulate(&public_key));
            black_box(encapsulated)
        })
    });

    // Decapsulation benchmark
    group.bench_function("classicmceliece_decapsulate", |b| {
        let keypair = classicmceliece_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = classicmceliece_encapsulate(&public_key).expect(
            "Encapsulation should succeed"
        );
        let ciphertext = encapsulated.ciphertext();

        b.iter(|| {
            let decapsulated = black_box(classicmceliece_decapsulate(&secret_key, &ciphertext));
            black_box(decapsulated)
        })
    });

    group.finish();
}

fn bench_dilithium_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dilithium Signature Operations");

    // Key generation benchmark
    group.bench_function("dilithium_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(dilithium_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("dilithium_sign", |b| {
        let keypair = dilithium_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Dilithium signature generation";

        b.iter(|| {
            let signature = black_box(dilithium_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("dilithium_verify", |b| {
        let keypair = dilithium_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Dilithium signature verification";
        let signed_message = dilithium_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(dilithium_verify(&public_key, &signed_message));
            black_box(is_valid)
        })
    });

    group.finish();
}

fn bench_falcon_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Falcon Signature Operations");

    // Key generation benchmark
    group.bench_function("falcon_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(falcon_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("falcon_sign", |b| {
        let keypair = falcon_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Falcon signature generation";

        b.iter(|| {
            let signature = black_box(falcon_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("falcon_verify", |b| {
        let keypair = falcon_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Falcon signature verification";
        let signature = falcon_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(falcon_verify(&public_key, message, &signature));
            black_box(is_valid)
        })
    });

    group.finish();
}

fn bench_sphincsplus_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("SPHINCS+ Signature Operations");

    // Key generation benchmark
    group.bench_function("sphincsplus_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(sphincsplus_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("sphincsplus_sign", |b| {
        let keypair = sphincsplus_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for SPHINCS+ signature generation";

        b.iter(|| {
            let signature = black_box(sphincsplus_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("sphincsplus_verify", |b| {
        let keypair = sphincsplus_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for SPHINCS+ signature verification";
        let signed_message = sphincsplus_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(sphincsplus_verify(&public_key, &signed_message));
            black_box(is_valid)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_kyber_operations,
    bench_hqc_operations,
    bench_classicmceliece_operations,
    bench_dilithium_operations,
    bench_falcon_operations,
    bench_sphincsplus_operations
);
criterion_main!(benches);
