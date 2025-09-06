#!/bin/bash

# Aegis Demo Portal - Complete Startup Script
# This script builds WASM, starts the portal server, and launches the browser automatically

echo "🚀 Starting Aegis Post-Quantum Cryptography Demo Portal..."
echo "=========================================================="

# Function to cleanup background processes
cleanup() {
    echo ""
    echo "🛑 Shutting down servers..."
    pkill -f "python3 -m http.server"
    echo "✅ Server stopped. Goodbye!"
    exit 0
}

# Set up signal handlers for Ctrl+C
trap cleanup SIGINT SIGTERM

# Check if port is already in use
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
    echo "❌ Port 8080 is already in use. Please stop any existing server first."
    echo "💡 To stop existing servers: pkill -f 'python3 -m http.server'"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

echo "📁 Project root: $PROJECT_ROOT"
echo "📁 Portal directory: $SCRIPT_DIR/../portal"
echo ""

# Step 1: Build WASM if needed
echo "🔨 Step 1: Building WebAssembly modules..."
cd "$PROJECT_ROOT"

# Check if WASM build is needed
if [ ! -f "aegis_crypto_core/pkg/aegis_crypto_core.js" ] || [ ! -f "aegis_crypto_core/pkg/aegis_crypto_core_bg.wasm" ]; then
    echo "   Building WASM modules (this may take a few minutes)..."

    # Check if wasm-pack is installed
    if ! command -v wasm-pack &> /dev/null; then
        echo "❌ wasm-pack is not installed. Installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi

    # Build WASM
    cd aegis_crypto_core
    wasm-pack build --release --target web --out-dir pkg

    if [ $? -eq 0 ]; then
        echo "✅ WASM build completed successfully"
    else
        echo "❌ WASM build failed. Continuing with existing build..."
    fi
else
    echo "✅ WASM modules already built"
fi

echo ""

# Step 2: Start the portal server
echo "🌐 Step 2: Starting Demo Portal Server..."
cd "$PROJECT_ROOT"

# Start server in background (from project root to access all demo folders)
python3 -m http.server 8080 &
SERVER_PID=$!

# Wait for server to be ready
echo "⏳ Waiting for server to start..."
sleep 3

# Check if server is running
if curl -s http://localhost:8080 > /dev/null; then
    echo "✅ Demo portal server is running on http://localhost:8080"
else
    echo "❌ Demo portal server failed to start"
    cleanup
fi

echo ""

# Step 3: Launch browser automatically
echo "🌍 Step 3: Launching browser..."
PORTAL_URL="http://localhost:8080/aegis_crypto_core/demos/portal/portal.html"

# Try to open in default browser
if command -v xdg-open &> /dev/null; then
    xdg-open "$PORTAL_URL" &
    echo "✅ Browser launched with xdg-open"
elif command -v open &> /dev/null; then
    open "$PORTAL_URL" &
    echo "✅ Browser launched with open"
elif command -v start &> /dev/null; then
    start "$PORTAL_URL" &
    echo "✅ Browser launched with start"
else
    echo "⚠️  Could not auto-launch browser. Please open manually:"
    echo "   $PORTAL_URL"
fi

echo ""
echo "🎉 Aegis Demo Portal is ready!"
echo "=========================================================="
echo "🌐 Portal URL: $PORTAL_URL"
echo "📋 Available demos:"
echo "   • Secure Messaging (Ready)"
echo "   • Blockchain Wallet (Ready)"
echo "   • Document Signing (Ready)"
echo "   • Digital Identity (Ready)"
echo "   • 22 additional demos (Coming Soon)"
echo ""
echo "🔧 Features:"
echo "   • Post-quantum cryptography algorithms"
echo "   • NIST standardized implementations"
echo "   • Interactive demonstrations"
echo "   • Real-time encryption/decryption"
echo ""
echo "Press Ctrl+C to stop the server"
echo "=========================================================="

# Keep script running and wait for Ctrl+C
wait
