#!/bin/bash

# Generate KAT files for all supported algorithms in Aegis
# This script builds and runs the testvector generators for each algorithm

set -e

echo "Generating KAT files for all supported algorithms..."

# Create output directory
mkdir -p kat_files

# Function to generate KAT files for a scheme
generate_kat() {
    local type=$1
    local scheme=$2
    local implementation=${3:-clean}
    local script_dir=$(pwd)

    echo "Generating KAT files for $type/$scheme/$implementation"

    cd PQClean/test

    # Build the testvector generator
    make TYPE=$type SCHEME=$scheme IMPLEMENTATION=$implementation testvectors

    # Run the generator to create KAT files
    ${script_dir}/PQClean/bin/testvectors_${scheme}_${implementation} > ${script_dir}/kat_files/${scheme}_${implementation}.kat

    cd ${script_dir}
}

# KEM algorithms
echo "Generating KEM KAT files..."

# ML-KEM (Kyber) variants
generate_kat kem ml-kem-512
generate_kat kem ml-kem-768
generate_kat kem ml-kem-1024

# HQC variants
generate_kat kem hqc-128
generate_kat kem hqc-192
generate_kat kem hqc-256

# Classic McEliece variants
generate_kat kem mceliece348864
generate_kat kem mceliece460896
generate_kat kem mceliece6688128

# Signature algorithms
echo "Generating Signature KAT files..."

# ML-DSA (Dilithium) variants
generate_kat sign ml-dsa-44
generate_kat sign ml-dsa-65
generate_kat sign ml-dsa-87

# Falcon variants
generate_kat sign falcon-512
generate_kat sign falcon-1024

# SPHINCS+ variants (SHA2)
generate_kat sign sphincs-sha2-128f-simple
generate_kat sign sphincs-sha2-192f-simple
generate_kat sign sphincs-sha2-256f-simple

# SPHINCS+ variants (SHAKE)
generate_kat sign sphincs-shake-128f-simple
generate_kat sign sphincs-shake-192f-simple
generate_kat sign sphincs-shake-256f-simple

echo "KAT file generation complete!"
echo "Generated files are in kat_files/ directory"

# List generated files
echo "Generated KAT files:"
ls -la kat_files/
