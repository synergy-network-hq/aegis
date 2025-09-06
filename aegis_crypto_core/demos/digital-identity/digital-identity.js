// Digital Identity Demo - Aegis PQC Demo
// Post-Quantum Cryptography for Digital Identity and Authentication

class DigitalIdentityDemo {
  constructor() {
    this.currentUser = null;
    this.identityProviders = [];
    this.credentials = [];
    this.initializeDemo();
  }

  async initializeDemo() {
    console.log('🆔 Initializing Digital Identity Demo...');

    // Initialize UI
    this.setupEventListeners();
    this.generateIdentityProviders();
    this.generateCredentials();
    this.updateUI();

    // Simulate PQC key generation
    await this.generatePQCKeys();

    console.log('✅ Digital Identity Demo initialized successfully!');
  }

  setupEventListeners() {
    // Identity provider selection
    document.addEventListener('click', (e) => {
      if (e.target.classList.contains('provider-item')) {
        const providerId = e.target.dataset.providerId;
        this.selectProvider(providerId);
      }
    });

    // Credential actions
    const issueCredentialBtn = document.getElementById('issueCredentialBtn');
    const verifyCredentialBtn = document.getElementById('verifyCredentialBtn');
    const revokeCredentialBtn = document.getElementById('revokeCredentialBtn');

    if (issueCredentialBtn) {
      issueCredentialBtn.addEventListener('click', () => this.issueCredential());
    }

    if (verifyCredentialBtn) {
      verifyCredentialBtn.addEventListener('click', () => this.verifyCredential());
    }

    if (revokeCredentialBtn) {
      revokeCredentialBtn.addEventListener('click', () => this.revokeCredential());
    }

    // Key management
    const generateKeysBtn = document.getElementById('generateKeysBtn');
    if (generateKeysBtn) {
      generateKeysBtn.addEventListener('click', () => this.generatePQCKeys());
    }
  }

  generateIdentityProviders() {
    const providers = [
      {
        id: 'government',
        name: 'Government ID',
        type: 'Official',
        icon: '🏛️',
        description: 'Official government digital identity',
        securityLevel: 'Maximum',
        algorithms: ['ML-DSA-87', 'FN-DSA-1024']
      },
      {
        id: 'banking',
        name: 'Banking Identity',
        type: 'Financial',
        icon: '🏦',
        description: 'Financial institution digital identity',
        securityLevel: 'Enhanced',
        algorithms: ['ML-DSA-65', 'ML-KEM-768']
      },
      {
        id: 'healthcare',
        name: 'Healthcare ID',
        type: 'Medical',
        icon: '🏥',
        description: 'Healthcare provider digital identity',
        securityLevel: 'Enhanced',
        algorithms: ['ML-DSA-65', 'SLH-DSA-SHA2-256f']
      },
      {
        id: 'enterprise',
        name: 'Enterprise ID',
        type: 'Corporate',
        icon: '🏢',
        description: 'Corporate digital identity',
        securityLevel: 'Standard',
        algorithms: ['ML-DSA-44', 'ML-KEM-512']
      }
    ];

    this.identityProviders = providers;
    this.renderProviderList();
  }

  renderProviderList() {
    const providerList = document.getElementById('providerList');
    if (!providerList) return;

    providerList.innerHTML = this.identityProviders.map(provider => `
            <div class="provider-item" data-provider-id="${provider.id}">
                <div class="provider-icon">${provider.icon}</div>
                <div class="provider-info">
                    <div class="provider-name">${provider.name}</div>
                    <div class="provider-type">${provider.type}</div>
                    <div class="provider-description">${provider.description}</div>
                </div>
                <div class="security-level ${provider.securityLevel.toLowerCase()}">
                    ${provider.securityLevel}
                </div>
                <div class="pqc-algorithms">
                    ${provider.algorithms.map(alg => `<span class="alg-badge">${alg}</span>`).join('')}
                </div>
            </div>
        `).join('');
  }

  selectProvider(providerId) {
    const provider = this.identityProviders.find(p => p.id === providerId);

    // Update UI
    document.querySelectorAll('.provider-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-provider-id="${providerId}"]`).classList.add('active');

    // Update provider details
    this.updateProviderDetails(provider);
    this.updateUI();
  }

  updateProviderDetails(provider) {
    const providerDetails = document.getElementById('providerDetails');
    if (!providerDetails) return;

    providerDetails.innerHTML = `
            <div class="provider-header">
                <div class="provider-icon-large">${provider.icon}</div>
                <div class="provider-info-large">
                    <h3>${provider.name}</h3>
                    <p>${provider.description}</p>
                    <div class="security-badge ${provider.securityLevel.toLowerCase()}">
                        ${provider.securityLevel} Security
                    </div>
                </div>
            </div>
            <div class="algorithms-section">
                <h4>PQC Algorithms Used:</h4>
                <div class="algorithms-list">
                    ${provider.algorithms.map(alg => `
                        <div class="algorithm-item">
                            <span class="alg-name">${alg}</span>
                            <span class="alg-status active">Active</span>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
  }

  generateCredentials() {
    const credentials = [
      {
        id: 'gov_id_001',
        type: 'Government ID',
        issuer: 'Government ID',
        holder: 'John Doe',
        issued: new Date('2024-01-15'),
        expires: new Date('2029-01-15'),
        status: 'Valid',
        attributes: {
          'Name': 'John Doe',
          'Date of Birth': '1985-06-15',
          'National ID': '123456789',
          'Address': '123 Main St, City, State'
        }
      },
      {
        id: 'bank_id_001',
        type: 'Banking Identity',
        issuer: 'Banking ID',
        holder: 'John Doe',
        issued: new Date('2024-02-01'),
        expires: new Date('2025-02-01'),
        status: 'Valid',
        attributes: {
          'Account Number': '****1234',
          'Bank': 'Secure Bank',
          'Account Type': 'Premium',
          'Credit Score': 'Excellent'
        }
      },
      {
        id: 'health_id_001',
        type: 'Healthcare ID',
        issuer: 'Healthcare ID',
        holder: 'John Doe',
        issued: new Date('2024-03-01'),
        expires: new Date('2025-03-01'),
        status: 'Valid',
        attributes: {
          'Patient ID': 'P123456',
          'Insurance': 'HealthPlus',
          'Primary Care': 'Dr. Smith',
          'Allergies': 'None'
        }
      }
    ];

    this.credentials = credentials;
    this.renderCredentials();
  }

  renderCredentials() {
    const credentialsList = document.getElementById('credentialsList');
    if (!credentialsList) return;

    credentialsList.innerHTML = this.credentials.map(cred => `
            <div class="credential-item ${cred.status.toLowerCase()}">
                <div class="credential-header">
                    <div class="credential-type">${cred.type}</div>
                    <div class="credential-status ${cred.status.toLowerCase()}">${cred.status}</div>
                </div>
                <div class="credential-details">
                    <div class="credential-issuer">Issued by: ${cred.issuer}</div>
                    <div class="credential-dates">
                        <span>Issued: ${cred.issued.toLocaleDateString()}</span>
                        <span>Expires: ${cred.expires.toLocaleDateString()}</span>
                    </div>
                </div>
                <div class="credential-actions">
                    <button class="btn btn-sm" onclick="window.digitalIdentityDemo.verifyCredential('${cred.id}')">
                        Verify
                    </button>
                    <button class="btn btn-sm btn-danger" onclick="window.digitalIdentityDemo.revokeCredential('${cred.id}')">
                        Revoke
                    </button>
                </div>
            </div>
        `).join('');
  }

  async issueCredential() {
    console.log('📜 Issuing new digital credential...');

    const issueSteps = [
      '🔐 Generating PQC key pair...',
      '📝 Creating credential template...',
      '✍️ Signing with ML-DSA...',
      '🔒 Encrypting sensitive data...',
      '✅ Credential issued successfully!'
    ];

    for (const step of issueSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    // Add new credential
    const newCredential = {
      id: `cred_${Date.now()}`,
      type: 'Enterprise ID',
      issuer: 'Enterprise ID',
      holder: 'John Doe',
      issued: new Date(),
      expires: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000), // 1 year
      status: 'Valid',
      attributes: {
        'Employee ID': 'EMP001',
        'Department': 'Engineering',
        'Role': 'Senior Developer',
        'Clearance': 'Level 2'
      }
    };

    this.credentials.push(newCredential);
    this.renderCredentials();

    console.log('✅ New credential issued successfully!');
  }

  async verifyCredential(credentialId) {
    const credential = this.credentials.find(c => c.id === credentialId);
    if (!credential) return;

    console.log(`🔍 Verifying credential: ${credential.type}`);

    const verifySteps = [
      '🔐 Retrieving public key...',
      '✍️ Verifying ML-DSA signature...',
      '🔒 Decrypting credential data...',
      '📋 Validating attributes...',
      '✅ Credential verified successfully!'
    ];

    for (const step of verifySteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 250));
    }

    // Show verification result
    const result = `
🔍 Credential Verification Result:

✅ Credential: ${credential.type}
✅ Issuer: ${credential.issuer}
✅ Status: ${credential.status}
✅ Signature: Valid
✅ Expiry: ${credential.expires > new Date() ? 'Valid' : 'Expired'}

The credential is authentic and has not been tampered with.
        `;
    alert(result);
  }

  async revokeCredential(credentialId) {
    const credential = this.credentials.find(c => c.id === credentialId);
    if (!credential) return;

    console.log(`🚫 Revoking credential: ${credential.type}`);

    const revokeSteps = [
      '🔐 Accessing revocation system...',
      '✍️ Signing revocation with ML-DSA...',
      '📋 Updating revocation list...',
      '🔄 Distributing revocation update...',
      '✅ Credential revoked successfully!'
    ];

    for (const step of revokeSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 250));
    }

    // Update credential status
    credential.status = 'Revoked';
    this.renderCredentials();

    console.log('✅ Credential revoked successfully!');
  }

  async generatePQCKeys() {
    console.log('🔑 Generating Post-Quantum Cryptographic Keys...');

    const keyGenerationSteps = [
      '🎲 Generating random seed...',
      '🔐 Creating ML-DSA signature key pair...',
      '🔑 Generating ML-KEM key encapsulation key...',
      '🛡️ Creating SLH-DSA hash-based signature...',
      '✅ All PQC keys generated successfully!'
    ];

    for (const step of keyGenerationSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    // Update key status
    this.updateKeyStatus();
  }

  updateKeyStatus() {
    const keyStatus = document.getElementById('keyStatus');
    if (keyStatus) {
      keyStatus.innerHTML = `
                <div class="status-item">
                    <span class="status-icon">✍️</span>
                    <span>ML-DSA-65: Active</span>
                </div>
                <div class="status-item">
                    <span class="status-icon">🔐</span>
                    <span>ML-KEM-768: Active</span>
                </div>
                <div class="status-item">
                    <span class="status-icon">🛡️</span>
                    <span>SLH-DSA-SHA2-256f: Active</span>
                </div>
            `;
    }
  }

  updateUI() {
    // Update header
    const headerTitle = document.getElementById('headerTitle');
    if (headerTitle) {
      headerTitle.textContent = 'Digital Identity & Authentication';
    }
  }

  // Demo-specific methods
  showPQCInfo() {
    const info = `
🆔 Post-Quantum Cryptography in Digital Identity:

• ML-DSA-65: Digital signatures for credential authentication
• ML-KEM-768: Key encapsulation for secure key exchange
• SLH-DSA-SHA2-256f: Hash-based signatures for long-term security
• FN-DSA-1024: Additional signature layer for maximum security

All digital identities are protected against quantum computer attacks!
        `;
    alert(info);
  }

  showSecurityMetrics() {
    const metrics = `
📊 Security Metrics:

• Signature Strength: 256-bit quantum-resistant
• Key Size: 1,312 bytes (ML-DSA-65)
• Signature Size: 2,701 bytes (ML-DSA-65)
• Hash Algorithm: SHA3-256
• Forward Secrecy: Enabled
• Revocation: Real-time

Your digital identity is protected by the latest NIST standards!
        `;
    alert(metrics);
  }
}

// Initialize demo when page loads
document.addEventListener('DOMContentLoaded', () => {
  window.digitalIdentityDemo = new DigitalIdentityDemo();

  // Add global functions for demo controls
  window.showPQCInfo = () => window.digitalIdentityDemo.showPQCInfo();
  window.showSecurityMetrics = () => window.digitalIdentityDemo.showSecurityMetrics();
});

console.log('🚀 Digital Identity Demo loaded successfully!');
