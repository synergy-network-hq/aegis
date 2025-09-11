#!/usr/bin/env node

/**
 * AEGIS Performance Benchmark Suite
 * Benchmarks all post-quantum cryptographic algorithms and implementations
 */

import init, {
  // ML-KEM functions
  mlkem512_keygen, mlkem512_encaps, mlkem512_decaps,
  mlkem768_keygen, mlkem768_encaps, mlkem768_decaps,
  mlkem1024_keygen, mlkem1024_encaps, mlkem1024_decaps,

  // ML-DSA functions
  mldsa44_keygen, mldsa44_sign, mldsa44_verify,
  mldsa65_keygen, mldsa65_sign, mldsa65_verify,
  mldsa87_keygen, mldsa87_sign, mldsa87_verify,

  // Falcon functions
  falcon512_keygen, falcon512_sign, falcon512_verify,
  falcon1024_keygen, falcon1024_sign, falcon1024_verify,

  // SPHINCS+ functions
  slhdsa_sha2_128f_keygen, slhdsa_sha2_128f_sign, slhdsa_sha2_128f_verify,
  slhdsa_sha2_192f_keygen, slhdsa_sha2_192f_sign, slhdsa_sha2_192f_verify,
  slhdsa_sha2_256f_keygen, slhdsa_sha2_256f_sign, slhdsa_sha2_256f_verify,

  // HQC-KEM functions
  hqc128_keygen, hqc128_encaps, hqc128_decaps,
  hqc192_keygen, hqc192_encaps, hqc192_decaps,
  hqc256_keygen, hqc256_encaps, hqc256_decaps
} from 'aegis-crypto-core';

// Benchmark configuration
const BENCHMARK_CONFIG = {
  iterations: 100,
  warmup: 10
};

// Test algorithms configuration
const ALGORITHMS = {
  'ML-KEM-512': {
    keygen: mlkem512_keygen,
    encaps: mlkem512_encaps,
    decaps: mlkem512_decaps,
    type: 'kem'
  },
  'ML-KEM-768': {
    keygen: mlkem768_keygen,
    encaps: mlkem768_encaps,
    decaps: mlkem768_decaps,
    type: 'kem'
  },
  'ML-KEM-1024': {
    keygen: mlkem1024_keygen,
    encaps: mlkem1024_encaps,
    decaps: mlkem1024_decaps,
    type: 'kem'
  },
  'ML-DSA-44': {
    keygen: mldsa44_keygen,
    sign: mldsa44_sign,
    verify: mldsa44_verify,
    type: 'signature'
  },
  'ML-DSA-65': {
    keygen: mldsa65_keygen,
    sign: mldsa65_sign,
    verify: mldsa65_verify,
    type: 'signature'
  },
  'ML-DSA-87': {
    keygen: mldsa87_keygen,
    sign: mldsa87_sign,
    verify: mldsa87_verify,
    type: 'signature'
  },
  'Falcon-512': {
    keygen: falcon512_keygen,
    sign: falcon512_sign,
    verify: falcon512_verify,
    type: 'signature'
  },
  'Falcon-1024': {
    keygen: falcon1024_keygen,
    sign: falcon1024_sign,
    verify: falcon1024_verify,
    type: 'signature'
  },
  'SPHINCS+-SHA2-128f': {
    keygen: slhdsa_sha2_128f_keygen,
    sign: slhdsa_sha2_128f_sign,
    verify: slhdsa_sha2_128f_verify,
    type: 'signature'
  },
  'SPHINCS+-SHA2-192f': {
    keygen: slhdsa_sha2_192f_keygen,
    sign: slhdsa_sha2_192f_sign,
    verify: slhdsa_sha2_192f_verify,
    type: 'signature'
  },
  'SPHINCS+-SHA2-256f': {
    keygen: slhdsa_sha2_256f_keygen,
    sign: slhdsa_sha2_256f_sign,
    verify: slhdsa_sha2_256f_verify,
    type: 'signature'
  },
  'HQC-128': {
    keygen: hqc128_keygen,
    encaps: hqc128_encaps,
    decaps: hqc128_decaps,
    type: 'kem'
  },
  'HQC-192': {
    keygen: hqc192_keygen,
    encaps: hqc192_encaps,
    decaps: hqc192_decaps,
    type: 'kem'
  },
  'HQC-256': {
    keygen: hqc256_keygen,
    encaps: hqc256_encaps,
    decaps: hqc256_decaps,
    type: 'kem'
  }
};

async function main() {
  console.log('⚡ AEGIS Performance Benchmark Suite');
  console.log('====================================\n');

  // Initialize WASM module
  console.log('Initializing WASM module...');
  await init();
  console.log('✅ WASM module initialized\n');

  const results = {};

  // Benchmark each algorithm
  for (const [algorithmName, config] of Object.entries(ALGORITHMS)) {
    console.log(`🔬 Benchmarking ${algorithmName}...`);
    results[algorithmName] = await benchmarkAlgorithm(algorithmName, config);
    console.log('');
  }

  // Display results
  displayResults(results);
}

async function benchmarkAlgorithm(algorithmName, config) {
  const results = {
    keygen: { times: [], avg: 0, min: Infinity, max: 0 },
    sign: { times: [], avg: 0, min: Infinity, max: 0 },
    verify: { times: [], avg: 0, min: Infinity, max: 0 },
    encaps: { times: [], avg: 0, min: Infinity, max: 0 },
    decaps: { times: [], avg: 0, min: Infinity, max: 0 }
  };

  let keyPair = null;
  let message = null;
  let signature = null;
  let ciphertext = null;
  let sharedSecret = null;

  // Warmup
  console.log(`  Warming up...`);
  for (let i = 0; i < BENCHMARK_CONFIG.warmup; i++) {
    keyPair = config.keygen();
    if (config.type === 'signature') {
      message = new TextEncoder().encode(`Test message ${i}`);
      signature = config.sign(keyPair.secretKey, message);
    } else {
      const encapResult = config.encaps(keyPair.publicKey);
      ciphertext = encapResult.ciphertext;
      sharedSecret = encapResult.sharedSecret;
    }
  }

  // Benchmark key generation
  console.log(`  Benchmarking key generation...`);
  for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
    const start = performance.now();
    keyPair = config.keygen();
    const end = performance.now();
    const time = end - start;
    results.keygen.times.push(time);
    results.keygen.min = Math.min(results.keygen.min, time);
    results.keygen.max = Math.max(results.keygen.max, time);
  }
  results.keygen.avg = results.keygen.times.reduce((a, b) => a + b, 0) / results.keygen.times.length;

  if (config.type === 'signature') {
    // Benchmark signing
    console.log(`  Benchmarking signing...`);
    message = new TextEncoder().encode('Benchmark test message');
    for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
      const start = performance.now();
      signature = config.sign(keyPair.secretKey, message);
      const end = performance.now();
      const time = end - start;
      results.sign.times.push(time);
      results.sign.min = Math.min(results.sign.min, time);
      results.sign.max = Math.max(results.sign.max, time);
    }
    results.sign.avg = results.sign.times.reduce((a, b) => a + b, 0) / results.sign.times.length;

    // Benchmark verification
    console.log(`  Benchmarking verification...`);
    for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
      const start = performance.now();
      config.verify(keyPair.publicKey, message, signature);
      const end = performance.now();
      const time = end - start;
      results.verify.times.push(time);
      results.verify.min = Math.min(results.verify.min, time);
      results.verify.max = Math.max(results.verify.max, time);
    }
    results.verify.avg = results.verify.times.reduce((a, b) => a + b, 0) / results.verify.times.length;

  } else {
    // Benchmark encapsulation
    console.log(`  Benchmarking encapsulation...`);
    for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
      const start = performance.now();
      const encapResult = config.encaps(keyPair.publicKey);
      const end = performance.now();
      const time = end - start;
      results.encaps.times.push(time);
      results.encaps.min = Math.min(results.encaps.min, time);
      results.encaps.max = Math.max(results.encaps.max, time);
      ciphertext = encapResult.ciphertext;
      sharedSecret = encapResult.sharedSecret;
    }
    results.encaps.avg = results.encaps.times.reduce((a, b) => a + b, 0) / results.encaps.times.length;

    // Benchmark decapsulation
    console.log(`  Benchmarking decapsulation...`);
    for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
      const start = performance.now();
      config.decaps(ciphertext, keyPair.secretKey);
      const end = performance.now();
      const time = end - start;
      results.decaps.times.push(time);
      results.decaps.min = Math.min(results.decaps.min, time);
      results.decaps.max = Math.max(results.decaps.max, time);
    }
    results.decaps.avg = results.decaps.times.reduce((a, b) => a + b, 0) / results.decaps.times.length;
  }

  return results;
}

function displayResults(results) {
  console.log('📊 BENCHMARK RESULTS');
  console.log('===================\n');

  // Create table header
  console.log('Algorithm'.padEnd(20) + 'Operation'.padEnd(12) + 'Avg (ms)'.padEnd(10) + 'Min (ms)'.padEnd(10) + 'Max (ms)'.padEnd(10) + 'Ops/sec');
  console.log('-'.repeat(80));

  // Display results for each algorithm
  for (const [algorithmName, result] of Object.entries(results)) {
    const operations = ['keygen'];

    if (result.sign.avg > 0) {
      operations.push('sign', 'verify');
    } else {
      operations.push('encaps', 'decaps');
    }

    for (const operation of operations) {
      const data = result[operation];
      if (data.avg > 0) {
        const opsPerSec = Math.round(1000 / data.avg);
        console.log(
          algorithmName.padEnd(20) +
          operation.padEnd(12) +
          data.avg.toFixed(2).padEnd(10) +
          data.min.toFixed(2).padEnd(10) +
          data.max.toFixed(2).padEnd(10) +
          opsPerSec.toString()
        );
      }
    }
    console.log('');
  }

  // Performance summary
  console.log('🏆 PERFORMANCE SUMMARY');
  console.log('=====================\n');

  // Fastest key generation
  let fastestKeygen = { name: '', time: Infinity };
  for (const [name, result] of Object.entries(results)) {
    if (result.keygen.avg < fastestKeygen.time) {
      fastestKeygen = { name, time: result.keygen.avg };
    }
  }
  console.log(`Fastest Key Generation: ${fastestKeygen.name} (${fastestKeygen.time.toFixed(2)}ms)`);

  // Fastest signing (if applicable)
  let fastestSign = { name: '', time: Infinity };
  for (const [name, result] of Object.entries(results)) {
    if (result.sign.avg > 0 && result.sign.avg < fastestSign.time) {
      fastestSign = { name, time: result.sign.avg };
    }
  }
  if (fastestSign.name) {
    console.log(`Fastest Signing: ${fastestSign.name} (${fastestSign.time.toFixed(2)}ms)`);
  }

  // Fastest verification (if applicable)
  let fastestVerify = { name: '', time: Infinity };
  for (const [name, result] of Object.entries(results)) {
    if (result.verify.avg > 0 && result.verify.avg < fastestVerify.time) {
      fastestVerify = { name, time: result.verify.avg };
    }
  }
  if (fastestVerify.name) {
    console.log(`Fastest Verification: ${fastestVerify.name} (${fastestVerify.time.toFixed(2)}ms)`);
  }

  // Fastest encapsulation (if applicable)
  let fastestEncaps = { name: '', time: Infinity };
  for (const [name, result] of Object.entries(results)) {
    if (result.encaps.avg > 0 && result.encaps.avg < fastestEncaps.time) {
      fastestEncaps = { name, time: result.encaps.avg };
    }
  }
  if (fastestEncaps.name) {
    console.log(`Fastest Encapsulation: ${fastestEncaps.name} (${fastestEncaps.time.toFixed(2)}ms)`);
  }

  // Fastest decapsulation (if applicable)
  let fastestDecaps = { name: '', time: Infinity };
  for (const [name, result] of Object.entries(results)) {
    if (result.decaps.avg > 0 && result.decaps.avg < fastestDecaps.time) {
      fastestDecaps = { name, time: result.decaps.avg };
    }
  }
  if (fastestDecaps.name) {
    console.log(`Fastest Decapsulation: ${fastestDecaps.name} (${fastestDecaps.time.toFixed(2)}ms)`);
  }

  console.log('\n✅ Benchmark completed successfully!');
}

// Run the benchmark
main().catch(console.error);
