// Aegis Demo Portal - Main Portal JavaScript
// This file manages the main demo portal interface

class DemoPortal {
  constructor() {
    this.demos = [
      // Non-WASM Demos (15 total)
      {
        id: 'secure-messaging',
        title: 'Secure Messaging',
        description: 'End-to-end encrypted chat with PQC algorithms',
        icon: '💬',
        status: 'ready',
        category: 'non-wasm',
        url: 'secure-messaging/secure-messaging.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'SHA3-256']
      },
      {
        id: 'blockchain-wallet',
        title: 'Blockchain Wallet',
        description: 'PQC-secured cryptocurrency wallet',
        icon: '🔐',
        status: 'ready',
        category: 'non-wasm',
        url: 'blockchain-wallet/blockchain-wallet.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'Dilithium-65', 'HQC-192']
      },
      {
        id: 'document-signing',
        title: 'Document Signing',
        description: 'Digital document signing & verification',
        icon: '📝',
        status: 'ready',
        category: 'non-wasm',
        url: 'document-signing/document-signing.html',
        algorithms: ['Falcon-512', 'Dilithium-65', 'SPHINCS+', 'SHA3-256']
      },
      {
        id: 'iot-security',
        title: 'IoT Security',
        description: 'IoT device security gateway',
        icon: '🏠',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'iot-security/iot-security.html',
        algorithms: ['Kyber-512', 'Falcon-512', 'SPHINCS+']
      },
      {
        id: 'financial-security',
        title: 'Financial Security',
        description: 'Financial transaction security system',
        icon: '💰',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'financial-security/financial-security.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'Dilithium-65']
      },
      {
        id: 'supply-chain-security',
        title: 'Supply Chain Security',
        description: 'Supply chain integrity verification',
        icon: '📦',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'supply-chain-security/supply-chain-security.html',
        algorithms: ['Falcon-512', 'SPHINCS+', 'SHA3-256']
      },
      {
        id: 'healthcare-data-protection',
        title: 'Healthcare Data Protection',
        description: 'Medical data privacy & security',
        icon: '🏥',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'healthcare-data-protection/healthcare-data-protection.html',
        algorithms: ['Kyber-1024', 'Falcon-1024', 'Dilithium-87']
      },
      {
        id: 'government-communications',
        title: 'Government Communications',
        description: 'Secure government messaging',
        icon: '🏛️',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'government-communications/government-communications.html',
        algorithms: ['Kyber-1024', 'Falcon-1024', 'SPHINCS+']
      },
      {
        id: 'smart-contract-security',
        title: 'Smart Contract Security',
        description: 'Blockchain smart contract security',
        icon: '⚡',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'smart-contract-security/smart-contract-security.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'Dilithium-65']
      },
      {
        id: 'digital-identity',
        title: 'Digital Identity',
        description: 'Digital identity management',
        icon: '🆔',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'digital-identity/digital-identity.html',
        algorithms: ['Falcon-512', 'SPHINCS+', 'SHA3-256']
      },
      {
        id: 'quantum-resistant-vpn',
        title: 'Quantum-Resistant VPN',
        description: 'Post-quantum VPN implementation',
        icon: '🔒',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'quantum-resistant-vpn/quantum-resistant-vpn.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'HQC-192']
      },
      {
        id: 'post-quantum-database',
        title: 'Post-Quantum Database',
        description: 'Database encryption & security',
        icon: '🗄️',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'post-quantum-database/post-quantum-database.html',
        algorithms: ['Kyber-512', 'Falcon-512', 'SHA3-256']
      },
      {
        id: 'ml-model-protection',
        title: 'ML Model Protection',
        description: 'Machine learning model security',
        icon: '🤖',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'ml-model-protection/ml-model-protection.html',
        algorithms: ['Kyber-768', 'Falcon-512', 'Dilithium-65']
      },
      {
        id: 'secure-voting-system',
        title: 'Secure Voting System',
        description: 'Quantum-resistant voting system',
        icon: '🗳️',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'secure-voting-system/secure-voting-system.html',
        algorithms: ['Falcon-512', 'SPHINCS+', 'SHA3-256']
      },
      {
        id: 'post-quantum-blockchain',
        title: 'Post-Quantum Blockchain',
        description: 'Advanced blockchain security',
        icon: '⛓️',
        status: 'coming-soon',
        category: 'non-wasm',
        url: 'post-quantum-blockchain/post-quantum-blockchain.html',
        algorithms: ['Kyber-1024', 'Falcon-1024', 'Dilithium-87']
      },
      // WASM Demos (6 total)
      {
        id: 'quantum-key-distribution',
        title: 'Quantum Key Distribution',
        description: 'QKD protocol implementation',
        icon: '🔑',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'quantum-key-distribution/quantum-key-distribution.html',
        algorithms: ['QKD Protocols', 'PQC Integration']
      },
      {
        id: 'quantum-resistant-iot-wasm',
        title: 'Quantum-Resistant IoT (WASM)',
        description: 'IoT network security with WASM',
        icon: '🌐',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'quantum-resistant-iot/quantum-resistant-iot.html',
        algorithms: ['WASM Optimized', 'PQC Algorithms']
      },
      {
        id: 'post-quantum-cloud-storage-wasm',
        title: 'Post-Quantum Cloud Storage',
        description: 'Cloud storage security with WASM',
        icon: '☁️',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'post-quantum-cloud-storage/post-quantum-cloud-storage.html',
        algorithms: ['WASM Performance', 'PQC Security']
      },
      {
        id: 'advanced-messaging',
        title: 'Advanced Messaging (WASM)',
        description: 'Enhanced messaging with WASM',
        icon: '📨',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'advanced-messaging/advanced-messaging.html',
        algorithms: ['WASM Optimized', 'Real-time PQC']
      },
      {
        id: 'real-time-crypto',
        title: 'Real-time Crypto (WASM)',
        description: 'Real-time cryptographic operations',
        icon: '⚡',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'real-time-crypto/real-time-crypto.html',
        algorithms: ['WASM Performance', 'Live PQC']
      },
      {
        id: 'interactive-learning',
        title: 'Interactive Learning (WASM)',
        description: 'Educational PQC demonstrations',
        icon: '🎓',
        status: 'wasm-coming-soon',
        category: 'wasm',
        url: 'interactive-learning/interactive-learning.html',
        algorithms: ['Educational', 'Interactive PQC']
      }
    ];

    this.currentFilter = 'all';
    this.init();
  }

  init() {
    this.setupEventListeners();
    this.renderDemoCards();
    this.startFloatingParticles();
    this.updateStats();
  }

  setupEventListeners() {
    // Filter buttons
    document.querySelectorAll('.filter-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        this.setActiveFilter(e.target.dataset.filter);
      });
    });

    // Search functionality
    const searchInput = document.getElementById('searchInput');
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        this.filterDemos(e.target.value);
      });
    }
  }

  setActiveFilter(filter) {
    this.currentFilter = filter;

    // Update active button state
    document.querySelectorAll('.filter-btn').forEach(btn => {
      btn.classList.remove('active');
    });
    document.querySelector(`[data-filter="${filter}"]`).classList.add('active');

    // Filter demos
    this.filterDemos();
  }

  filterDemos(searchTerm = '') {
    const demoCards = document.querySelectorAll('.demo-card');

    demoCards.forEach(card => {
      const demoId = card.dataset.demoId;
      const demo = this.demos.find(d => d.id === demoId);

      let shouldShow = true;

      // Apply category filter
      if (this.currentFilter !== 'all') {
        if (this.currentFilter === 'non-wasm' && demo.category !== 'non-wasm') {
          shouldShow = false;
        } else if (this.currentFilter === 'wasm' && demo.category !== 'wasm') {
          shouldShow = false;
        }
      }

      // Apply search filter
      if (searchTerm && shouldShow) {
        const searchLower = searchTerm.toLowerCase();
        const matchesSearch = demo.title.toLowerCase().includes(searchLower) ||
          demo.description.toLowerCase().includes(searchLower) ||
          demo.algorithms.some(algo => algo.toLowerCase().includes(searchLower));
        shouldShow = matchesSearch;
      }

      // Show/hide card
      card.style.display = shouldShow ? 'block' : 'none';
    });

    // Update stats
    this.updateStats();
  }

  renderDemoCards() {
    const container = document.querySelector('.demo-grid');
    if (!container) return;

    container.innerHTML = this.demos.map(demo => this.createDemoCard(demo)).join('');

    // Add click event listeners to demo cards
    document.querySelectorAll('.demo-card').forEach(card => {
      card.addEventListener('click', (e) => {
        const demoId = card.dataset.demoId;
        this.openDemo(demoId);
      });
    });
  }

  createDemoCard(demo) {
    const statusClass = this.getStatusClass(demo.status);
    const statusText = this.getStatusText(demo.status);

    return `
            <div class="demo-card ${statusClass}" data-demo-id="${demo.id}">
                <div class="demo-icon">${demo.icon}</div>
                <h3>${demo.title}</h3>
                <p>${demo.description}</p>
                <div class="demo-algorithms">
                    <strong>PQC Algorithms:</strong>
                    <div class="algorithm-tags">
                        ${demo.algorithms.map(algo => `<span class="algorithm-tag">${algo}</span>`).join('')}
                    </div>
                </div>
                <div class="demo-status">
                    <span class="status-badge ${statusClass}">${statusText}</span>
                </div>
                <div class="demo-actions">
                    ${demo.status === 'ready' ?
        '<button class="launch-btn">Launch Demo</button>' :
        '<button class="coming-soon-btn" disabled>Coming Soon</button>'
      }
                </div>
            </div>
        `;
  }

  getStatusClass(status) {
    switch (status) {
      case 'ready': return 'status-ready';
      case 'coming-soon': return 'status-coming-soon';
      case 'wasm-coming-soon': return 'status-wasm-coming-soon';
      default: return 'status-coming-soon';
    }
  }

  getStatusText(status) {
    switch (status) {
      case 'ready': return 'Ready';
      case 'coming-soon': return 'Coming Soon';
      case 'wasm-coming-soon': return 'WASM Coming Soon';
      default: return 'Coming Soon';
    }
  }

  openDemo(demoId) {
    const demo = this.demos.find(d => d.id === demoId);
    if (!demo) return;

    if (demo.status === 'ready') {
      window.location.href = demo.url;
    } else {
      this.showComingSoonModal(demo);
    }
  }

  showComingSoonModal(demo) {
    const modal = document.createElement('div');
    modal.className = 'coming-soon-modal';
    modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>${demo.title} - Coming Soon!</h2>
                    <button class="close-btn" onclick="this.closest('.coming-soon-modal').remove()">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="demo-preview">
                        <div class="demo-icon-large">${demo.icon}</div>
                        <h3>${demo.title}</h3>
                        <p>${demo.description}</p>
                    </div>
                    <div class="demo-details">
                        <h4>🔒 PQC Security Features</h4>
                        <ul>
                            ${demo.algorithms.map(algo => `<li>${algo}</li>`).join('')}
                        </ul>
                        <h4>📅 Expected Timeline</h4>
                        <p>${demo.category === 'wasm' ? 'Q3 2024' : 'Q2 2024'}</p>
                        <h4>🚀 What to Expect</h4>
                        <p>This demo will showcase real cryptographic operations with performance metrics,
                        security analysis, and interactive features suitable for investor presentations and research grants.</p>
                    </div>
                </div>
                <div class="modal-footer">
                    <button onclick="this.closest('.coming-soon-modal').remove()">Close</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();
  }

  addModalStyles() {
    if (!document.getElementById('modal-styles')) {
      const style = document.createElement('style');
      style.id = 'modal-styles';
      style.textContent = `
                .coming-soon-modal {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: rgba(0, 0, 0, 0.5);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 10000;
                }
                .modal-content {
                    background: white;
                    border-radius: 20px;
                    max-width: 600px;
                    width: 90%;
                    max-height: 80vh;
                    overflow-y: auto;
                    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                }
                .modal-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 20px 30px;
                    border-bottom: 2px solid #f0f0f0;
                }
                .modal-header h2 {
                    margin: 0;
                    color: #333;
                }
                .close-btn {
                    background: none;
                    border: none;
                    font-size: 24px;
                    cursor: pointer;
                    color: #666;
                }
                .modal-body {
                    padding: 30px;
                }
                .demo-preview {
                    text-align: center;
                    margin-bottom: 30px;
                }
                .demo-icon-large {
                    font-size: 4em;
                    margin-bottom: 15px;
                }
                .demo-details h4 {
                    color: #333;
                    margin: 20px 0 10px 0;
                }
                .demo-details ul {
                    margin: 0;
                    padding-left: 20px;
                }
                .demo-details li {
                    margin-bottom: 5px;
                    color: #666;
                }
                .modal-footer {
                    padding: 20px 30px;
                    border-top: 2px solid #f0f0f0;
                    text-align: right;
                }
                .modal-footer button {
                    background: #007bff;
                    color: white;
                    border: none;
                    padding: 10px 20px;
                    border-radius: 8px;
                    cursor: pointer;
                    font-size: 16px;
                }
            `;
      document.head.appendChild(style);
    }
  }

  startFloatingParticles() {
    const particlesContainer = document.querySelector('.floating-particles');
    if (!particlesContainer) return;

    // Create floating particles
    for (let i = 0; i < 20; i++) {
      this.createParticle(particlesContainer);
    }
  }

  createParticle(container) {
    const particle = document.createElement('div');
    particle.className = 'particle';
    particle.style.left = Math.random() * 100 + '%';
    particle.style.animationDuration = (Math.random() * 20 + 10) + 's';
    particle.style.animationDelay = Math.random() * 5 + 's';

    container.appendChild(particle);
  }

  updateStats() {
    const readyCount = this.demos.filter(d => d.status === 'ready').length;
    const comingSoonCount = this.demos.filter(d => d.status === 'coming-soon').length;
    const wasmCount = this.demos.filter(d => d.status === 'wasm-coming-soon').length;
    const totalCount = this.demos.length;

    // Update stats display
    const statsElements = {
      'total-demos': totalCount,
      'ready-demos': readyCount,
      'coming-soon': comingSoonCount,
      'wasm-demos': wasmCount
    };

    Object.entries(statsElements).forEach(([id, count]) => {
      const element = document.getElementById(id);
      if (element) {
        element.textContent = count;
      }
    });
  }
}

// Initialize portal when page loads
document.addEventListener('DOMContentLoaded', () => {
  window.demoPortal = new DemoPortal();
});

// Global functions for demo navigation
function openDemo(demoId) {
  if (window.demoPortal) {
    window.demoPortal.openDemo(demoId);
  }
}

function showDemoInfo(demoId) {
  const demo = window.demoPortal?.demos.find(d => d.id === demoId);
  if (demo) {
    window.demoPortal.showComingSoonModal(demo);
  }
}


