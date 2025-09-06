// Aegis Demo Portal - Main Portal JavaScript
// This file manages the main demo portal interface with comprehensive demo data

class DemoPortal {
  constructor() {
    this.currentFilter = 'all';
    this.searchTerm = '';
    this.demos = this.initializeDemoData();
    this.initializeDemo();
  }

  initializeDemoData() {
    return [
      // READY DEMOS
      {
        id: 'secure-messaging',
        title: 'Quantum-Secure Chat Platform',
        category: 'Communication',
        status: 'ready',
        icon: '💬',
        description: 'End-to-end encrypted messaging platform using post-quantum cryptography. Features real-time chat with ML-KEM key exchange and ML-DSA digital signatures for secure communication.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SHA3-256'],
        url: '../secure-messaging/secure-messaging.html',
        features: ['Real-time encryption', 'Key exchange', 'Digital signatures', 'Message authentication']
      },
      {
        id: 'blockchain-wallet',
        title: 'Post-Quantum Crypto Wallet',
        category: 'Financial',
        status: 'ready',
        icon: '💰',
        description: 'Post-quantum secure cryptocurrency wallet with advanced key management. Implements quantum-resistant algorithms for wallet security, transaction signing, and key derivation.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'SLH-DSA-SHA2-256f'],
        url: '../blockchain-wallet/blockchain-wallet.html',
        features: ['Quantum-resistant keys', 'Transaction signing', 'Multi-signature support', 'Hardware wallet integration']
      },
      {
        id: 'document-signing',
        title: 'Quantum-Resistant Document Signer',
        category: 'Authentication',
        status: 'ready',
        icon: '📝',
        description: 'Digital document signing and verification system using post-quantum digital signatures. Ensures document integrity and authenticity with quantum-resistant cryptographic proofs.',
        algorithms: ['ML-DSA-65', 'SLH-DSA-SHA2-192f', 'SHA3-256'],
        url: '../document-signing/document-signing.html',
        features: ['Document integrity', 'Digital signatures', 'Timestamp verification', 'Batch signing']
      },
      {
        id: 'digital-identity',
        title: 'Quantum-Safe Identity Manager',
        category: 'Authentication',
        status: 'ready',
        icon: '🆔',
        description: 'Comprehensive digital identity management system with post-quantum authentication. Features identity verification, credential management, and secure access control.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'FN-DSA-512'],
        url: '../digital-identity/digital-identity.html',
        features: ['Identity verification', 'Credential management', 'Access control', 'Privacy protection']
      },

      // COMING SOON DEMOS
      {
        id: 'iot-security',
        title: 'Quantum-Safe IoT Guardian',
        category: 'IoT',
        status: 'coming-soon',
        icon: '🏠',
        description: 'Comprehensive IoT device security framework using lightweight post-quantum cryptography. Designed for resource-constrained devices with optimized algorithms and secure communication protocols.',
        algorithms: ['ML-KEM-512', 'ML-DSA-44', 'SLH-DSA-SHA2-128f'],
        url: '#',
        features: ['Lightweight algorithms', 'Device authentication', 'Secure updates', 'Network protection']
      },
      {
        id: 'financial-security',
        title: 'Quantum-Resistant Banking Suite',
        category: 'Financial',
        status: 'ready',
        icon: '🏦',
        description: 'Advanced financial security platform for banking and fintech applications. Implements quantum-resistant algorithms for payment processing, fraud detection, and regulatory compliance.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SLH-DSA-SHA2-256f'],
        url: '../financial-security/financial-security.html',
        features: ['Payment security', 'Fraud detection', 'Regulatory compliance', 'Risk assessment']
      },
      {
        id: 'government-communications',
        title: 'Classified Government Network',
        category: 'Government',
        status: 'ready',
        icon: '🏛️',
        description: 'Secure government communication system with highest security standards. Features classified information protection, secure file sharing, and compliance with government security requirements.',
        algorithms: ['ML-KEM-768', 'ML-DSA-87', 'FN-DSA-1024'],
        url: '../government-communications/government-communications.html',
        features: ['Classified communications', 'Secure file sharing', 'Access control', 'Audit trails']
      },
      {
        id: 'healthcare-data-protection',
        title: 'HIPAA-Compliant Health Vault',
        category: 'Healthcare',
        status: 'ready',
        icon: '🏥',
        description: 'HIPAA-compliant healthcare data protection system using post-quantum cryptography. Ensures patient data privacy, secure medical records, and regulatory compliance.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SLH-DSA-SHA2-256f'],
        url: '../healthcare-data-protection/healthcare-data-protection.html',
        features: ['HIPAA compliance', 'Patient privacy', 'Medical records', 'Data encryption']
      },
      {
        id: 'ml-model-protection',
        title: 'AI Model Fortress',
        category: 'AI/ML',
        status: 'coming-soon',
        icon: '🤖',
        description: 'Machine learning model protection and secure inference using post-quantum cryptography. Features model encryption, secure training, and privacy-preserving inference.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'HQC-KEM-192'],
        url: '#',
        features: ['Model encryption', 'Secure training', 'Privacy-preserving inference', 'Intellectual property protection']
      },
      {
        id: 'supply-chain-security',
        title: 'Anti-Counterfeit Supply Chain',
        category: 'Logistics',
        status: 'coming-soon',
        icon: '🚚',
        description: 'End-to-end supply chain security with post-quantum cryptography. Tracks products from origin to destination with tamper-proof records and secure authentication.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SLH-DSA-SHA2-192f'],
        url: '#',
        features: ['Product tracking', 'Tamper detection', 'Authentication', 'Chain of custody']
      },
      {
        id: 'post-quantum-cloud-storage',
        title: 'Quantum-Proof Cloud Vault',
        category: 'Cloud',
        status: 'coming-soon',
        icon: '☁️',
        description: 'Quantum-resistant cloud storage solution with advanced encryption and secure file sharing. Features zero-knowledge architecture and client-side encryption.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'SLH-DSA-SHA2-256f'],
        url: '#',
        features: ['Zero-knowledge storage', 'Client-side encryption', 'Secure sharing', 'Data integrity']
      },
      {
        id: 'post-quantum-database',
        title: 'Quantum-Safe Database Engine',
        category: 'Database',
        status: 'coming-soon',
        icon: '🗄️',
        description: 'Quantum-resistant database system with encrypted queries and secure data processing. Features homomorphic encryption and secure multi-party computation.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'HQC-KEM-256'],
        url: '#',
        features: ['Encrypted queries', 'Homomorphic encryption', 'Secure computation', 'Data privacy']
      },
      {
        id: 'post-quantum-blockchain',
        title: 'Quantum-Resistant Blockchain',
        category: 'Blockchain',
        status: 'coming-soon',
        icon: '⛓️',
        description: 'Next-generation blockchain platform with post-quantum consensus and smart contracts. Features quantum-resistant mining and secure transaction processing.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'SLH-DSA-SHA2-256f'],
        url: '#',
        features: ['Quantum-resistant consensus', 'Smart contracts', 'Secure mining', 'Transaction privacy']
      },
      {
        id: 'smart-contract-security',
        title: 'DeFi Security Shield',
        category: 'Blockchain',
        status: 'coming-soon',
        icon: '📋',
        description: 'Enhanced smart contract security using post-quantum cryptography. Features secure contract execution, quantum-resistant verification, and automated security auditing.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SLH-DSA-SHA2-192f'],
        url: '#',
        features: ['Secure execution', 'Quantum-resistant verification', 'Automated auditing', 'Vulnerability detection']
      },
      {
        id: 'secure-voting-system',
        title: 'Tamper-Proof Voting Platform',
        category: 'Government',
        status: 'ready',
        icon: '🗳️',
        description: 'Cryptographically secure electronic voting system with post-quantum protection. Ensures vote privacy, verifiability, and protection against quantum attacks.',
        algorithms: ['ML-DSA-65', 'SLH-DSA-SHA2-256f', 'SHA3-256'],
        url: '../secure-voting-system/secure-voting-system.html',
        features: ['Vote privacy', 'Verifiability', 'Audit trails', 'Quantum resistance']
      },
      {
        id: 'quantum-resistant-vpn',
        title: 'Quantum-Safe VPN Tunnel',
        category: 'Network',
        status: 'coming-soon',
        icon: '🔒',
        description: 'Next-generation VPN with post-quantum cryptography for secure tunneling. Features quantum-resistant key exchange and authenticated encryption.',
        algorithms: ['ML-KEM-768', 'ML-DSA-65', 'SLH-DSA-SHA2-192f'],
        url: '#',
        features: ['Quantum-resistant tunneling', 'Key exchange', 'Authenticated encryption', 'Traffic obfuscation']
      },
      {
        id: 'real-time-crypto',
        title: 'Lightning-Fast Crypto Engine',
        category: 'Performance',
        status: 'coming-soon',
        icon: '⚡',
        description: 'High-performance real-time cryptographic operations optimized for post-quantum algorithms. Features hardware acceleration and optimized implementations.',
        algorithms: ['ML-KEM-512/768', 'ML-DSA-44/65', 'SLH-DSA-SHA2-128f/192f'],
        url: '#',
        features: ['Hardware acceleration', 'Real-time processing', 'Optimized algorithms', 'Performance monitoring']
      },
      {
        id: 'quantum-key-distribution',
        title: 'Quantum Communication Lab',
        category: 'Quantum',
        status: 'coming-soon',
        icon: '🔑',
        description: 'Hybrid quantum-classical key distribution system combining QKD with post-quantum cryptography for ultimate security and practical deployment.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'QKD Protocols'],
        url: '#',
        features: ['Quantum key distribution', 'Hybrid security', 'Long-distance QKD', 'Key management']
      },
      {
        id: 'quantum-resistant-iot',
        title: 'Smart City Security Grid',
        category: 'IoT',
        status: 'coming-soon',
        icon: '🌐',
        description: 'Specialized IoT security framework for quantum-resistant communication in smart cities and industrial IoT environments with optimized resource usage.',
        algorithms: ['ML-KEM-512', 'ML-DSA-44', 'SLH-DSA-SHA2-128f'],
        url: '#',
        features: ['Smart city security', 'Industrial IoT', 'Resource optimization', 'Scalable deployment']
      },
      {
        id: 'interactive-learning',
        title: 'PQC Learning Academy',
        category: 'Education',
        status: 'coming-soon',
        icon: '🎓',
        description: 'Educational platform for learning post-quantum cryptography through interactive demonstrations, tutorials, and hands-on exercises.',
        algorithms: ['All NIST Algorithms', 'Educational Examples', 'Visual Demonstrations'],
        url: '#',
        features: ['Interactive tutorials', 'Visual demonstrations', 'Hands-on exercises', 'Progress tracking']
      },
      {
        id: 'advanced-messaging',
        title: 'Enterprise Secure Messenger',
        category: 'Communication',
        status: 'coming-soon',
        icon: '📨',
        description: 'Advanced secure messaging platform with post-quantum cryptography, featuring group messaging, file sharing, and advanced privacy features.',
        algorithms: ['ML-KEM-1024', 'ML-DSA-87', 'SLH-DSA-SHA2-256f'],
        url: '#',
        features: ['Group messaging', 'File sharing', 'Advanced privacy', 'Message scheduling']
      }
    ];
  }

  async initializeDemo() {
    console.log('🔬 Initializing Demo Portal...');

    // Initialize UI
    this.setupEventListeners();
    this.renderDemoCards();
    this.updateStats();

    console.log('✅ Demo Portal initialized successfully!');
  }

  setupEventListeners() {
    // Filter buttons
    document.querySelectorAll('.filter-tab').forEach(btn => {
      btn.addEventListener('click', (e) => {
        this.setActiveFilter(e.target.dataset.filter);
      });
    });

    // Search functionality
    const searchInput = document.getElementById('searchInput');
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        this.searchTerm = e.target.value.toLowerCase();
        this.filterDemos();
      });
    }
  }

  setActiveFilter(filter) {
    this.currentFilter = filter;

    // Update active button
    document.querySelectorAll('.filter-tab').forEach(btn => {
      btn.classList.remove('active');
    });
    document.querySelector(`[data-filter="${filter}"]`).classList.add('active');

    this.filterDemos();
  }

  filterDemos() {
    const demoCards = document.querySelectorAll('.demo-card');
    let visibleCount = 0;
    let readyCount = 0;

    demoCards.forEach(card => {
      const demoId = card.dataset.demoId;
      const demo = this.demos.find(d => d.id === demoId);

      if (!demo) return;

      const status = demo.status;
      const category = demo.category.toLowerCase();
      const title = demo.title.toLowerCase();
      const description = demo.description.toLowerCase();
      const algorithms = demo.algorithms.join(' ').toLowerCase();

      let shouldShow = true;

      // Apply filter
      if (this.currentFilter !== 'all') {
        if (this.currentFilter === 'ready' && status !== 'ready') {
          shouldShow = false;
        } else if (this.currentFilter === 'coming-soon' && status !== 'coming-soon') {
          shouldShow = false;
        } else if (this.currentFilter === 'in-development' && status !== 'in-development') {
          shouldShow = false;
        } else if (this.currentFilter === 'non-wasm' && category !== 'reference') {
          shouldShow = false;
        } else if (this.currentFilter === 'wasm' && category === 'reference') {
          shouldShow = false;
        }
      }

      // Apply search
      if (shouldShow && this.searchTerm) {
        const searchableText = `${title} ${description} ${algorithms} ${category}`;
        if (!searchableText.includes(this.searchTerm)) {
          shouldShow = false;
        }
      }

      // Show/hide card with animation
      if (shouldShow) {
        card.style.display = 'block';
        card.style.opacity = '0';
        card.style.transform = 'translateY(20px)';
        setTimeout(() => {
          card.style.transition = 'all 0.3s ease';
          card.style.opacity = '1';
          card.style.transform = 'translateY(0)';
        }, visibleCount * 50);
        visibleCount++;
        if (status === 'ready') readyCount++;
      } else {
        card.style.display = 'none';
      }
    });

    this.updateStats(visibleCount, readyCount);
  }

  updateStats(totalVisible = null, readyVisible = null) {
    const cryptoSpeed = document.getElementById('cryptoSpeed');
    const totalDemos2 = document.getElementById('totalDemos2');
    const readyDemos = document.getElementById('readyDemos');
    const inDevelopment = document.getElementById('inDevelopment');

    if (cryptoSpeed) {
      // Simulate crypto operation speed (2.1ms average)
      const speed = 2.1;
      cryptoSpeed.textContent = `${speed}ms`;
    }

    if (totalDemos2) {
      const total = totalVisible !== null ? totalVisible : this.demos.length;
      this.animateNumber(totalDemos2, parseInt(totalDemos2.textContent) || 0, total);
    }

    if (readyDemos) {
      const ready = readyVisible !== null ? readyVisible : this.demos.filter(d => d.status === 'ready').length;
      this.animateNumber(readyDemos, parseInt(readyDemos.textContent) || 0, ready);
    }

    if (inDevelopment) {
      const inDev = this.demos.filter(d => d.status === 'coming-soon').length;
      this.animateNumber(inDevelopment, parseInt(inDevelopment.textContent) || 0, inDev);
    }
  }

  animateNumber(element, start, end) {
    const duration = 500;
    const startTime = performance.now();

    const animate = (currentTime) => {
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / duration, 1);

      const current = Math.round(start + (end - start) * progress);
      element.textContent = current;

      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };

    requestAnimationFrame(animate);
  }

  renderDemoCards() {
    const demosGrid = document.querySelector('.demos-grid');
    if (!demosGrid) return;

    demosGrid.innerHTML = '';

    this.demos.forEach((demo, index) => {
      const card = this.createDemoCard(demo, index);
      demosGrid.appendChild(card);
    });
  }

  createDemoCard(demo, index) {
    const card = document.createElement('div');
    card.className = `demo-card ${demo.status}`;
    card.dataset.demoId = demo.id;
    card.dataset.category = demo.category.toLowerCase();

    const algorithmsHtml = demo.algorithms.map(alg =>
      `<span class="algorithm-badge">${alg}</span>`
    ).join('');

    const buttonHtml = demo.status === 'ready'
      ? `<a href="${demo.url}" class="btn btn-primary">Launch Demo</a>`
      : `<button class="btn btn-secondary" disabled>Coming Soon</button>`;

    card.innerHTML = `
      <div class="demo-header">
        <div class="demo-icon">${demo.icon}</div>
        <div class="demo-title">${demo.title}</div>
        <div class="demo-status ${demo.status}">${demo.status.replace('-', ' ').toUpperCase()}</div>
      </div>
      <div class="demo-category">${demo.category}</div>
      <div class="demo-description">${demo.description}</div>
      <div class="demo-algorithms">
        ${algorithmsHtml}
      </div>
      <div class="demo-actions">
        ${buttonHtml}
      </div>
    `;

    // Add click handler for demo info
    card.addEventListener('click', (e) => {
      if (!e.target.closest('.btn')) {
        this.showDemoInfo(demo.id);
      }
    });

    return card;
  }

  showDemoInfo(demoId) {
    const demo = this.demos.find(d => d.id === demoId);
    if (demo) {
      const featuresHtml = demo.features.map(feature => `• ${feature}`).join('\n');
      const info = `
🔬 ${demo.title}
📂 Category: ${demo.category}
📊 Status: ${demo.status.replace('-', ' ').toUpperCase()}

${demo.description}

🔧 Features:
${featuresHtml}

🔐 Algorithms: ${demo.algorithms.join(', ')}

${demo.status === 'ready' ? '🌐 Ready to launch!' : '⏳ Coming soon...'}
      `;
      alert(info);
    }
  }
}

// Initialize demo when page loads
document.addEventListener('DOMContentLoaded', () => {
  window.demoPortal = new DemoPortal();
});

console.log('🚀 Demo Portal loaded successfully!');
