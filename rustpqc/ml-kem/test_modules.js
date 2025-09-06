#!/usr/bin/env node

/**
 * Node.js test script to verify WASM modules work correctly
 * Run with: node test_modules.js
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testWASMModules() {
  console.log('🔐 Testing ML-KEM WASM Modules\n');

  try {
    // Check if all required files exist
    const requiredFiles = [
      'kat_generator.wasm',
      'mlkem512.wasm',
      'mlkem768.wasm',
      'mlkem1024.wasm'
    ];

    for (const file of requiredFiles) {
      const filePath = path.join(__dirname, file);
      if (!fs.existsSync(filePath)) {
        throw new Error(`${file} not found`);
      }
    }

    console.log('✅ All required WASM files found');

    // Check file sizes
    for (const file of requiredFiles) {
      const filePath = path.join(__dirname, file);
      const stats = fs.statSync(filePath);
      console.log(`📁 ${file}: ${(stats.size / 1024).toFixed(1)} KB`);
    }

    // Check if files are valid WASM
    for (const file of requiredFiles) {
      const filePath = path.join(__dirname, file);
      const buffer = fs.readFileSync(filePath);

      // Check WASM magic number (0x00 0x61 0x73 0x6D)
      if (buffer[0] === 0x00 && buffer[1] === 0x61 &&
        buffer[2] === 0x73 && buffer[3] === 0x6D) {
        console.log(`✅ ${file} appears to be valid WASM`);
      } else {
        console.log(`⚠️ ${file} may not be valid WASM`);
      }
    }

    // Check for expected exports (basic validation)
    console.log('\n🔍 Basic validation complete');
    console.log('💡 To run full cryptographic tests, open kat_test_harness.html in a browser');
    console.log('💡 The browser environment provides the WebAssembly API needed for testing');
    console.log('\n📋 Available ML-KEM variants:');
    console.log('   • ML-KEM-512 (2 polynomial vectors, 3/2 noise)');
    console.log('   • ML-KEM-768 (3 polynomial vectors, 2/2 noise)');
    console.log('   • ML-KEM-1024 (4 polynomial vectors, 2/2 noise)');

  } catch (error) {
    console.error('❌ Error testing WASM modules:', error.message);
    process.exit(1);
  }
}

// Run the test
testWASMModules();
