# Aegis Demo Scripts

This directory contains the startup script for the Aegis Post-Quantum Cryptography Demo Portal.

## Available Scripts

### `start_portal.sh`

**Complete demo portal startup script**

This is the main script that handles everything needed to run the demo portal.

**Usage:**

```bash
./start_portal.sh
```

### `start.sh`

**Convenience script**

A shorter alias for the main startup script.

**Usage:**

```bash
./start.sh
```

**Features:**
* Automatically builds WebAssembly modules if needed
* Starts demo portal server on port 8080
* Launches browser automatically
* Provides access to all 27 demo applications
* Includes proper cleanup on Ctrl+C
* Shows comprehensive information about available demos

**Access:** http://localhost:8080/aegis_crypto_core/demos/portal/portal.html

## Demo Applications

The portal provides access to 27 post-quantum cryptography demo applications:

### Ready Demos (5)

* **Secure Messaging** - End-to-end encrypted chat
* **Blockchain Wallet** - PQC-secured cryptocurrency wallet
* **Document Signing** - Digital document signing & verification
* **Digital Identity** - Digital identity and authentication system


### Coming Soon Demos (22)

* IoT Security, Financial Security, Government Communications
* Healthcare Data Protection, ML Model Protection
* Supply Chain Security, Post-Quantum Cloud Storage
* Post-Quantum Database, Post-Quantum Blockchain
* Smart Contract Security, Secure Voting System
* Quantum-Resistant VPN, Real-Time Crypto
* Quantum Key Distribution, Quantum-Resistant IoT
* Interactive Learning, Advanced Messaging
* And more...

## Requirements

* **Python 3** - For the HTTP server
* **Modern Web Browser** - For viewing the demos
* **Port 8080** - Must be available
* **Rust & wasm-pack** - For building WebAssembly modules (auto-installed if needed)

## What the Script Does

1. **Checks Dependencies** - Verifies Python 3 and port availability
2. **Builds WASM** - Compiles Rust code to WebAssembly if needed
3. **Starts Server** - Launches the demo portal HTTP server
4. **Opens Browser** - Automatically launches your default browser
5. **Provides Info** - Shows all available demos and features

## Troubleshooting

### Port Already in Use

If you get a "port already in use" error:

```bash
# Find what's using port 8080
lsof -i :8080

# Kill the process (replace PID with actual process ID)
kill -9 PID
```

### Script Not Executable

If you get permission errors:

```bash
chmod +x start_portal.sh
```

### WASM Build Issues

If WASM build fails:

```bash
# Install wasm-pack manually
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or use cargo to install
cargo install wasm-pack
```

### Portal Not Loading

1. Check that all files exist in the portal directory
2. Verify Python 3 is installed
3. Check browser console for any errors
4. Ensure WASM modules are built in `aegis_crypto_core/pkg/`

## File Structure

```
demo-scripts/
├── start_portal.sh     # Complete startup script
├── start.sh            # Convenience alias
└── README.md           # This documentation

../portal/
├── portal.html         # Main portal page
├── portal.css          # Portal styling
└── portal.js           # Portal functionality

../../pkg/
├── aegis_crypto_core.js        # WASM JavaScript bindings
├── aegis_crypto_core_bg.wasm   # WebAssembly binary
└── ...                         # Other WASM files
```

## Quick Start

1. **Navigate to the demo-scripts directory:**

```bash
   cd aegis_crypto_core/demos/demo-scripts
   ```

2. **Run the startup script:**

```bash
   ./start_portal.sh
   ```

3. **Your browser will open automatically** to the demo portal

4. **Press Ctrl+C** to stop the server when done

## Support

For issues or questions about the demo portal, please check:
1. This README file
2. The portal's built-in help and documentation
3. The main project documentation
4. Check the terminal output for any error messages
