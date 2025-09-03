#!/bin/bash

# Aegis Demo Portal Startup Script
# This script starts both the backend API server and frontend portal server

echo "🚀 Starting Aegis Demo Portal..."
echo "=================================="

# Function to cleanup background processes
cleanup() {
    echo ""
    echo "🛑 Shutting down servers..."
    pkill -f web_api_server
    pkill -f "python3 -m http.server"
    echo "✅ Servers stopped. Goodbye!"
    exit 0
}

# Set up signal handlers for Ctrl+C
trap cleanup SIGINT SIGTERM

# Check if ports are already in use
if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null ; then
    echo "❌ Port 3000 is already in use. Please stop any existing servers first."
    exit 1
fi

if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
    echo "❌ Port 8080 is already in use. Please stop any existing servers first."
    exit 1
fi

echo "📍 Starting Backend API Server (Rust) on port 3000..."
echo "📍 Starting Frontend Portal Server (Python) on port 8080..."
echo ""
echo "🌐 Access your demo at: http://localhost:8080"
echo "🔗 Backend API: http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop both servers"
echo "=================================="

# Start backend server in background
cd aegis_crypto_core
cargo run --bin web_api_server --features "kyber,falcon" &
BACKEND_PID=$!

# Wait a moment for backend to start
sleep 3

# Start frontend server in background
cd examples/portal
python3 -m http.server 8080 &
FRONTEND_PID=$!

# Wait for both servers to be ready
echo "⏳ Waiting for servers to start..."
sleep 2

# Check if servers are running
if curl -s http://localhost:3000/api/status > /dev/null; then
    echo "✅ Backend server is running on http://localhost:3000"
else
    echo "❌ Backend server failed to start"
    cleanup
fi

if curl -s http://localhost:8080 > /dev/null; then
    echo "✅ Frontend server is running on http://localhost:8080"
else
    echo "❌ Frontend server failed to start"
    cleanup
fi

echo ""
echo "🎉 Both servers are running successfully!"
echo "🌐 Open your browser and go to: http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop both servers"

# Keep script running and wait for Ctrl+C
wait
