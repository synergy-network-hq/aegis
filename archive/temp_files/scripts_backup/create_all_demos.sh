#!/bin/bash

# Script to create all remaining demo application files
# This creates the basic structure for all demos that haven't been built yet

echo "🚀 Creating all remaining demo application files..."

# List of remaining non-WASM demos (excluding the 3 already created)
REMAINING_DEMOS=(
    "iot-security"
    "financial-security"
    "supply-chain-security"
    "healthcare-data-protection"
    "government-communications"
    "smart-contract-security"
    "digital-identity"
    "quantum-resistant-vpn"
    "post-quantum-database"
    "ml-model-protection"
    "secure-voting-system"
    "post-quantum-blockchain"
    "quantum-resistant-iot"
    "post-quantum-cloud-storage"
)

# List of WASM demos
WASM_DEMOS=(
    "quantum-key-distribution"
    "advanced-messaging"
    "real-time-crypto"
    "interactive-learning"
)

# Create remaining non-WASM demo files
for demo in "${REMAINING_DEMOS[@]}"; do
    echo "📁 Creating $demo demo..."

    # Create HTML file
    cat > "demos/$demo/$demo.html" << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DEMO_TITLE - Aegis PQC Demo</title>
    <link rel="stylesheet" href="../portal/portal.css">
    <style>
        .demo-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            padding: 40px;
            text-align: center;
        }

        .demo-content {
            background: white;
            border-radius: 20px;
            padding: 60px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1);
            max-width: 800px;
        }

        .demo-icon {
            font-size: 4em;
            margin-bottom: 20px;
        }

        .demo-title {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #333;
        }

        .demo-description {
            font-size: 1.2em;
            color: #666;
            margin-bottom: 30px;
            line-height: 1.6;
        }

        .demo-status {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 15px;
            margin-bottom: 30px;
        }

        .back-button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 30px;
            border-radius: 25px;
            cursor: pointer;
            font-size: 16px;
            font-weight: 600;
            transition: all 0.3s ease;
        }

        .back-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }
    </style>
</head>
<body>
    <div class="demo-container">
        <div class="demo-content">
            <div class="demo-icon">🚧</div>
            <h1 class="demo-title">DEMO_TITLE Demo</h1>
            <p class="demo-description">
                This demo application is currently under development and will showcase
                Aegis's post-quantum cryptography capabilities for DEMO_TITLE use cases.
            </p>

            <div class="demo-status">
                <h3>🔒 PQC Security Features (Coming Soon)</h3>
                <p>This demo will implement multiple PQC algorithms including:</p>
                <ul style="text-align: left; display: inline-block;">
                    <li>ML-KEM (Kyber) for key exchange</li>
                    <li>ML-DSA (Dilithium) for digital signatures</li>
                    <li>FN-DSA (Falcon) for compact signatures</li>
                    <li>SLH-DSA (SPHINCS+) for hash-based signatures</li>
                    <li>HQC-KEM for additional key encapsulation</li>
                </ul>
            </div>

            <p style="color: #666; font-size: 0.9em;">
                Expected completion: Q2 2024<br>
                This demo will provide real cryptographic operations with performance metrics,
                security analysis, and interactive features suitable for investor presentations.
            </p>
        </div>

        <button class="back-button" onclick="goBack()">← Back to Portal</button>
    </div>

    <script>
        function goBack() {
            window.location.href = '../portal/index.html';
        }
    </script>
</body>
</html>
EOF

    # Replace placeholder with actual demo name
    sed -i "s/DEMO_TITLE/${demo//-/ }/g" "demos/$demo/$demo.html"

    # Create JavaScript file
    cat > "demos/$demo/$demo.js" << EOF
// ${demo//-/ } Demo - Aegis PQC Demo
// This demo is currently under development

console.log('${demo//-/ } Demo - Coming Soon!');

// TODO: Implement demo functionality
// - PQC algorithm integration
// - Interactive features
// - Performance metrics
// - Security analysis
// - Real cryptographic operations
EOF

    echo "✅ Created $demo demo files"
done

# Create WASM demo files
for demo in "${WASM_DEMOS[@]}"; do
    echo "📁 Creating $demo WASM demo..."

    # Create HTML file
    cat > "demos/$demo/$demo.html" << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DEMO_TITLE - Aegis WASM Demo</title>
    <link rel="stylesheet" href="../portal/portal.css">
    <style>
        .demo-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            padding: 40px;
            text-align: center;
        }

        .demo-content {
            background: white;
            border-radius: 20px;
            padding: 60px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1);
            max-width: 800px;
        }

        .demo-icon {
            font-size: 4em;
            margin-bottom: 20px;
        }

        .demo-title {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #333;
        }

        .demo-description {
            font-size: 1.2em;
            color: #666;
            margin-bottom: 30px;
            line-height: 1.6;
        }

        .demo-status {
            background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
            color: white;
            padding: 20px;
            border-radius: 15px;
            margin-bottom: 30px;
        }

        .back-button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 30px;
            border-radius: 25px;
            cursor: pointer;
            font-size: 16px;
            font-weight: 600;
            transition: all 0.3s ease;
        }

        .back-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }
    </style>
</head>
<body>
    <div class="demo-container">
        <div class="demo-content">
            <div class="demo-icon">🌐</div>
            <h1 class="demo-title">DEMO_TITLE WASM Demo</h1>
            <p class="demo-description">
                This WebAssembly (WASM) demo application will showcase Aegis's
                post-quantum cryptography capabilities running directly in web browsers.
            </p>

            <div class="demo-status">
                <h3>🚧 WASM Features (Coming Soon)</h3>
                <p>This demo will leverage WebAssembly for:</p>
                <ul style="text-align: left; display: inline-block;">
                    <li>Native performance in browsers</li>
                    <li>Cross-platform compatibility</li>
                    <li>Real-time cryptographic operations</li>
                    <li>Interactive learning experiences</li>
                    <li>Client-side security processing</li>
                </ul>
            </div>

            <p style="color: #666; font-size: 0.9em;">
                Expected completion: Q3 2024<br>
                This demo will provide browser-based PQC operations with WASM optimizations,
                making it accessible to users without installing additional software.
            </p>
        </div>

        <button class="back-button" onclick="goBack()">← Back to Portal</button>
    </div>

    <script>
        function goBack() {
            window.location.href = '../portal/index.html';
        }
    </script>
</body>
</html>
EOF

    # Replace placeholder with actual demo name
    sed -i "s/DEMO_TITLE/${demo//-/ }/g" "demos/$demo/$demo.html"

    # Create JavaScript file
    cat > "demos/$demo/$demo.js" << EOF
// ${demo//-/ } WASM Demo - Aegis PQC Demo
// This WASM demo is currently under development

console.log('${demo//-/ } WASM Demo - Coming Soon!');

// TODO: Implement WASM demo functionality
// - WebAssembly integration
// - PQC algorithm implementation
// - Browser-based cryptographic operations
// - Performance optimizations
// - Cross-platform compatibility
EOF

    echo "✅ Created $demo WASM demo files"
done

echo ""
echo "🎉 All demo application files have been created!"
echo "📁 Total demos created: $(( ${#REMAINING_DEMOS[@]} + ${#WASM_DEMOS[@]} ))"
echo "🔐 Non-WASM demos: ${#REMAINING_DEMOS[@]}"
echo "🌐 WASM demos: ${#WASM_DEMOS[@]}"
echo ""
echo "📝 Next steps:"
echo "1. Update the portal to include all new demos"
echo "2. Implement the actual functionality for each demo"
echo "3. Add PQC algorithm integration"
echo "4. Create comprehensive documentation"
echo ""
echo "🚀 Ready to build the future of cryptography!"


