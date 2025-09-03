#!/bin/bash

# Aegis Backend Server Startup Script
# This script starts the backend API server in the foreground

echo "🚀 Starting Aegis Backend API Server..."
echo "========================================"

# Check if port is already in use
if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null ; then
    echo "❌ Port 3000 is already in use. Please stop any existing server first."
    exit 1
fi

echo "📍 Backend server will be available at: http://localhost:3000"
echo "🌐 Frontend should be served separately on port 8080"
echo ""
echo "Press Ctrl+C to stop the server"
echo "========================================"

# Start backend server in foreground
cd aegis_crypto_core
cargo run --bin web_api_server --features "kyber,falcon"
