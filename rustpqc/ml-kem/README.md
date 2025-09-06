# ML-KEM WASM KAT Testing Framework

This directory contains a comprehensive testing framework for validating the cryptographic correctness of the ML-KEM WASM implementations using Known Answer Tests (KATs).

## 🎯 Purpose

This framework ensures that the WASM-compiled ML-KEM implementations are **not rigged** and actually perform correct cryptographic operations by:

1. **Generating KAT vectors** using the reference implementation
2. **Running actual cryptographic operations** with the WASM implementations
3. **Comparing outputs** to ensure they match exactly
4. **Validating the complete cryptographic flow** (key generation, encryption, decryption)

## 📁 Files

* **`mlkem512.wasm`** - The ML-KEM-512 implementation to be tested
* **`mlkem768.wasm`** - The ML-KEM-768 implementation to be tested
* **`mlkem1024.wasm`** - The ML-KEM-1024 implementation to be tested
* **`kat_generator.wasm`** - WASM module that generates KAT test vectors
* **`kat_test_harness.html`** - Comprehensive testing interface
* **`test_modules.js`** - Node.js validation script
* **`README.md`** - This documentation file

## 🚀 Quick Start

### **Option 1: Browser Testing (Recommended)**

1. **Open the testing harness:**

```bash
   # Start a local web server
   python3 -m http.server 8000

   # Open in browser: http://localhost:8000/kat_test_harness.html
   ```

2. **Click "Run All Tests"** to execute comprehensive validation

### **Option 2: Command Line Validation**

```bash
# Validate WASM files
node test_modules.js

# Check file integrity
file *.wasm
```

## 🔍 What Gets Tested

### **ML-KEM Variants Tested:**

* **ML-KEM-512**: 2 polynomial vectors, 3/2 noise parameters
* **ML-KEM-768**: 3 polynomial vectors, 2/2 noise parameters
* **ML-KEM-1024**: 4 polynomial vectors, 2/2 noise parameters

### **Cryptographic Operations Validated:**

1. **Key Generation** (`crypto_kem_keypair`)
   - Public key generation
   - Secret key generation
   - Deterministic output with same seeds

2. **Encryption** (`crypto_kem_enc`)
   - Ciphertext generation
   - Shared secret generation
   - Deterministic output with same inputs

3. **Decryption** (`crypto_kem_dec`)
   - Shared secret recovery
   - Verification of encryption/decryption consistency

## 🛡️ Anti-Rigging Measures

This framework prevents fake passing results by:

* **Using different WASM modules** for generation vs. testing
* **Running actual cryptographic operations** instead of returning hardcoded values
* **Testing all three variants** to ensure consistency across parameter sets
* **Validating complete cryptographic flows** (keygen → encrypt → decrypt)
* **Using deterministic seeds** for reproducible testing

## 📊 Test Results

The framework provides:

* **Real-time progress tracking** with visual progress bars
* **Detailed logging** of each test step
* **Pass/fail results** for each variant and operation
* **Error details** if any tests fail
* **Summary statistics** of overall test performance

## 🔧 Technical Details

### **WASM Compilation:**

* Compiled using WASI SDK 22.0
* Target: `wasm32-wasip1`
* Optimization: `-O3`
* No external dependencies

### **Test Configuration:**

* **Test Count**: 10 tests per variant (30 total)
* **Memory Allocation**: Proper buffer sizing for each variant
* **Error Handling**: Comprehensive error checking and reporting

## 🚨 Important Notes

* **These are minimal implementations** for testing purposes
* **For production use**, you should use the full NIST reference implementations
* **The KAT generator** provides deterministic test vectors for validation
* **All tests must pass** to confirm cryptographic correctness

## 🎉 Success Criteria

A successful test run will show:
* ✅ All 30 tests passing (10 per variant)
* ✅ All cryptographic operations completing successfully
* ✅ Consistent outputs across multiple test runs
* ✅ No errors or failures in the test log

## 🔍 Troubleshooting

### **Common Issues:**

1. **WASM files not found**: Ensure all `.wasm` files are in the `wasm-kat/` directory
2. **Module loading errors**: Check that files are valid WebAssembly modules
3. **Test failures**: Review the test log for specific error details

### **Validation Commands:**

```bash
# Check WASM file validity
file *.wasm

# Verify file sizes
ls -la *.wasm

# Test individual modules
node test_modules.js
```

---

**Note**: This framework validates that the WASM implementations are functionally correct and not rigged to return fake results. For production cryptographic use, always use the official NIST reference implementations.
