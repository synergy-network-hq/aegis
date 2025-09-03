# 🚀 Aegis Demo Applications

This folder contains all the demo applications showcasing Aegis's Post-Quantum Cryptography (PQC) capabilities. The demos are organized by type and provide interactive examples of real-world PQC implementations.

## 📁 Folder Structure

```
demos/
├── portal/                    # Main demo portal interface
├── demo-docs/                 # Documentation for all demos
├── demo-scripts/              # Startup and management scripts
├── secure-messaging/          # End-to-end encrypted messaging
├── blockchain-wallet/         # PQC-secured cryptocurrency wallet
├── document-signing/          # Digital document signing & verification
├── iot-security/             # IoT device security gateway
├── financial-security/        # Financial transaction security
├── supply-chain-security/     # Supply chain integrity verification
├── healthcare-data-protection/ # Medical data privacy & security
├── government-communications/  # Secure government messaging
├── smart-contract-security/   # Blockchain smart contract security
├── digital-identity/          # Digital identity management
├── quantum-resistant-vpn/     # Post-quantum VPN implementation
├── post-quantum-database/     # Database encryption & security
├── ml-model-protection/       # Machine learning model security
├── secure-voting-system/      # Quantum-resistant voting
├── quantum-key-distribution/  # QKD protocol implementation
├── post-quantum-blockchain/   # Advanced blockchain security
├── quantum-resistant-iot/     # IoT network security
└── post-quantum-cloud-storage/ # Cloud storage security
```

## 🎯 Demo Categories

### **Non-WASM Demos (15 total)**

These demos run natively using Rust and the `pqcrypto` crate suite:

01. **Secure Messaging** - End-to-end encrypted chat with Kyber KEM and Falcon signatures
02. **Blockchain Wallet** - PQC-secured cryptocurrency wallet with multiple algorithms
03. **Document Signing** - Digital document signing and verification system
04. **IoT Security** - IoT device security gateway with secure registration
05. **Financial Security** - Financial transaction security with fraud detection
06. **Supply Chain Security** - Supply chain integrity verification system
07. **Healthcare Data Protection** - Medical data privacy and security
08. **Government Communications** - Secure government messaging system
09. **Smart Contract Security** - Blockchain smart contract security
10. **Digital Identity** - Digital identity management system
11. **Quantum-Resistant VPN** - Post-quantum VPN implementation
12. **Post-Quantum Database** - Database encryption and security
13. **ML Model Protection** - Machine learning model security
14. **Secure Voting System** - Quantum-resistant voting system
15. **Post-Quantum Blockchain** - Advanced blockchain security features

### **WASM Demos (6 total)**

These demos run in web browsers using WebAssembly (coming soon):

01. **Quantum Key Distribution** - QKD protocol implementation
02. **Quantum-Resistant IoT** - IoT network security
03. **Post-Quantum Cloud Storage** - Cloud storage security
04. **Advanced Messaging** - Enhanced messaging with WASM optimizations
05. **Real-time Crypto** - Real-time cryptographic operations
06. **Interactive Learning** - Educational PQC demonstrations

## 🚀 Getting Started

### **Quick Start**

```bash
# Navigate to the demos folder
cd demos

# Start all services
./demo-scripts/start-demo.sh
```

### **Manual Start**

```bash
# Terminal 1: Start Backend API Server
cd demos/demo-scripts
./start-backend.sh

# Terminal 2: Start Frontend Portal
cd demos/portal
python3 -m http.server 8080
```

## 🌐 Accessing Demos

01. **Main Portal**: http://localhost:8080
02. **Individual Demos**: Navigate through the portal or access directly:
   - Secure Messaging: http://localhost:8080/secure-messaging.html
   - Blockchain Wallet: http://localhost:8080/blockchain-wallet.html
   - Document Signing: http://localhost:8080/document-signing.html
   - And more...

## 🔐 PQC Algorithms Used

* **ML-KEM (Kyber)**: Key Encapsulation Mechanism variants (512, 768, 1024)
* **ML-DSA (Dilithium)**: Digital Signature Algorithm variants (44, 65, 87)
* **FN-DSA (Falcon)**: Digital Signature Algorithm variants (512, 1024)
* **SLH-DSA (SPHINCS+)**: Digital Signature Algorithm variants
* **HQC-KEM**: Key Encapsulation Mechanism variants (128, 192, 256)
* **SHA3-256**: Cryptographic hashing

## 📚 Documentation

* **Demo Setup Guide**: `demo-docs/DEMO_SETUP.md`
* **Individual Demo Docs**: `demo-docs/` folder contains detailed documentation for each demo
* **API Documentation**: Available at http://localhost:3000/api/docs when backend is running

## 🛠️ Development Status

* ✅ **Complete**: Secure Messaging, Blockchain Wallet, Document Signing, IoT Security, Financial Security
* 🔄 **In Progress**: Supply Chain Security, Healthcare Data Protection
* 📋 **Planned**: Government Communications, Smart Contract Security, Digital Identity
* 🚧 **WASM**: All WASM demos are planned for future development

## 🎮 Interactive Features

Each demo includes:
* Real cryptographic operations (no simulation)
* Interactive user interfaces
* Real-time PQC status monitoring
* Performance metrics and benchmarks
* Educational explanations of algorithms
* Security analysis and best practices

## 🔧 Technical Requirements

* **Backend**: Rust with `pqcrypto` crates
* **Frontend**: HTML5, CSS3, JavaScript ES6+
* **Server**: Python 3.7+ for frontend serving
* **Ports**: 3000 (backend), 8080 (frontend)
* **Browser**: Modern browser with ES6+ support

## 📝 Notes

* All cryptographic operations use real, production-ready PQC implementations
* Demos are designed for both educational and demonstration purposes
* Performance metrics are suitable for investor presentations and research grants
* Security analysis follows industry best practices and NIST standards

## 🆘 Support

For issues or questions:
01. Check the demo documentation in `demo-docs/`
02. Verify both backend and frontend servers are running
03. Check browser console for JavaScript errors
04. Ensure correct ports are available and accessible

---

**Happy exploring the future of cryptography! 🚀🔐**
