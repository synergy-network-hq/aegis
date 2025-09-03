# 🚀 Aegis Demo Portal Setup Guide

This guide will help you get the Aegis Demo Portal running quickly and easily.

## 🎯 Quick Start (Recommended)

### Option 1: Use the Startup Script (Easiest)

```bash
# Navigate to the project directory
cd aegis_crypto_core

# Run the startup script
./start_demo.sh
```

This script will:
* Start the backend API server (Rust) on port 3000
* Start the frontend portal server (Python) on port 8080
* Check that both servers are running
* Keep running until you press Ctrl+C

### Option 2: Manual Setup (More Control)

```bash
# Terminal 1: Start Backend Server
cd aegis_crypto_core
cargo run --bin web_api_server --features "kyber,falcon"

# Terminal 2: Start Frontend Server
cd aegis_crypto_core/examples/portal
python3 -m http.server 8080
```

## 🌐 Access Your Demo

Once both servers are running:

* **Main Portal**: http://localhost:8080
* **Secure Messaging Demo**: http://localhost:8080/secure-messaging.html
* **Backend API**: http://localhost:3000

## 🛑 Stopping the Servers

### If using the startup script:

* Press **Ctrl+C** in the terminal where you ran the script
* Both servers will stop automatically

### If running manually:

* Press **Ctrl+C** in each terminal where a server is running

## 🔧 Troubleshooting

### Port Already in Use

If you see "Port XXXX is already in use":

```bash
# Stop any existing servers
pkill -f web_api_server
pkill -f "python3 -m http.server"

# Or find and kill specific processes
lsof -ti:3000 | xargs kill
lsof -ti:8080 | xargs kill
```

### Backend Server Won't Start

```bash
# Check if Rust is installed
rustc --version

# Check if dependencies are installed
cd aegis_crypto_core
cargo check --bin web_api_server --features "kyber,falcon"
```

### Frontend Server Won't Start

```bash
# Check if Python is installed
python3 --version

# Check if the portal directory exists
ls -la examples/portal/
```

## 📁 File Structure

```
aegis_crypto_core/
├── start_demo.sh          # Startup script for both servers
├── start_backend.sh       # Startup script for backend only
├── examples/portal/       # Frontend files
│   ├── index.html         # Main portal page
│   ├── secure-messaging.html  # Secure messaging demo
│   ├── portal.js          # Portal JavaScript
│   └── secure-messaging.js    # Demo JavaScript
└── src/bin/web_api_server.rs  # Backend API server
```

## 🎮 Using the Demo

1. **Open the Portal**: Navigate to http://localhost:8080
2. **Choose a Demo**: Click on any demo card (Secure Messaging is ready)
3. **Interact**: Send messages, generate keys, view crypto operations
4. **Explore**: Each demo showcases real PQC algorithms

## 🔐 PQC Algorithms Demonstrated

* **Kyber-512**: Key Encapsulation Mechanism (KEM)
* **Falcon-512**: Digital Signatures
* **SHA3-256**: Cryptographic Hashing

## 📝 Notes

* The backend server compiles Rust code, so the first run may take a few minutes
* Both servers must be running for the demo to work properly
* The frontend communicates with the backend via HTTP API calls
* All cryptographic operations are real - no simulation!

## 🆘 Need Help?

If you encounter issues:
1. Check that both servers are running
2. Verify ports 3000 and 8080 are available
3. Check the browser console for JavaScript errors
4. Ensure you're using the correct URLs

Happy exploring! 🚀🔐
