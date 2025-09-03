// Document Signing Demo - Aegis PQC Demo
// This demo showcases PQC-secured digital document signing and verification

class DocumentSigning {
  constructor() {
    this.currentUser = 'alice';
    this.users = {
      alice: { name: 'Alice Johnson', role: 'CEO', status: 'pending' },
      bob: { name: 'Bob Smith', role: 'CTO', status: 'pending' },
      carol: { name: 'Carol Davis', role: 'Legal Counsel', status: 'pending' }
    };
    this.document = {
      id: 'DOC-2024-001',
      title: 'Contract Agreement',
      content: '',
      hash: 'a1b2c3d4e5f6...',
      signatures: [],
      status: 'pending',
      createdAt: new Date(Date.now() - 2 * 60 * 60 * 1000) // 2 hours ago
    };
    this.pqcStatus = {
      falcon: 'active',
      dilithium: 'active',
      sphincs: 'active',
      sha3: 'active'
    };

    this.init();
  }

  init() {
    this.setupEventListeners();
    this.loadDocumentContent();
    this.updatePQCStatus();
    this.startRealTimeUpdates();
  }

  setupEventListeners() {
    // User selection
    document.querySelectorAll('.user-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectUser(e.currentTarget.dataset.user);
      });
    });

    // Action cards
    document.querySelectorAll('.action-card').forEach(card => {
      card.addEventListener('click', (e) => {
        const action = e.currentTarget.querySelector('h3').textContent.toLowerCase();
        this.handleAction(action);
      });
    });
  }

  selectUser(userId) {
    // Update active state
    document.querySelectorAll('.user-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-user="${userId}"]`).classList.add('active');

    this.currentUser = userId;
    this.updateUserDisplay();
  }

  updateUserDisplay() {
    const user = this.users[this.currentUser];
    // Update any user-specific display elements
    console.log(`Selected user: ${user.name} (${user.role})`);
  }

  handleAction(action) {
    switch (action) {
      case '✍️ sign document':
        this.showSignModal();
        break;
      case '🔍 verify signature':
        this.showVerifyModal();
        break;
      case '📤 export':
        this.showExportModal();
        break;
      case '📊 audit trail':
        this.showAuditModal();
        break;
    }
  }

  showSignModal() {
    const user = this.users[this.currentUser];
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Sign Document</h2>
                <div class="signer-info">
                    <h3>${user.name}</h3>
                    <p>Role: ${user.role}</p>
                    <p>Document: ${this.document.title}</p>
                </div>

                <div class="signature-options">
                    <h4>Signature Options</h4>
                    <div class="option-group">
                        <label>
                            <input type="checkbox" id="falconSig" checked>
                            Falcon-512 Signature (Primary)
                        </label>
                        <label>
                            <input type="checkbox" id="dilithiumSig" checked>
                            Dilithium-65 Signature (Secondary)
                        </label>
                        <label>
                            <input type="checkbox" id="sphincsSig">
                            SPHINCS+ Signature (Tertiary)
                        </label>
                    </div>
                </div>

                <div class="signature-preview">
                    <h4>Signature Preview</h4>
                    <div class="preview-box">
                        <div>Document Hash: ${this.document.hash}</div>
                        <div>Signer: ${user.name}</div>
                        <div>Timestamp: ${new Date().toISOString()}</div>
                        <div>PQC Algorithms: <span id="algoPreview">Falcon-512, Dilithium-65</span></div>
                    </div>
                </div>

                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Cancel</button>
                    <button onclick="documentSigningDemo.signDocument()" class="primary">Sign Document</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();

    // Update algorithm preview based on checkboxes
    document.getElementById('falconSig').addEventListener('change', () => this.updateAlgoPreview());
    document.getElementById('dilithiumSig').addEventListener('change', () => this.updateAlgoPreview());
    document.getElementById('sphincsSig').addEventListener('change', () => this.updateAlgoPreview());
  }

  showVerifyModal() {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Verify Signature</h2>
                <div class="verification-input">
                    <label>Document Hash:</label>
                    <input type="text" id="verifyHash" value="${this.document.hash}" placeholder="Enter document hash">
                </div>
                <div class="verification-input">
                    <label>Signature:</label>
                    <textarea id="verifySignature" placeholder="Paste signature data"></textarea>
                </div>
                <div class="verification-input">
                    <label>Public Key:</label>
                    <textarea id="verifyPublicKey" placeholder="Paste public key"></textarea>
                </div>

                <div class="verification-result" id="verificationResult" style="display: none;">
                    <h4>Verification Result</h4>
                    <div class="result-box" id="resultBox"></div>
                </div>

                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Close</button>
                    <button onclick="documentSigningDemo.verifySignature()" class="primary">Verify</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();
  }

  showExportModal() {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Export Document</h2>
                <div class="export-options">
                    <h4>Export Format</h4>
                    <div class="option-group">
                        <label>
                            <input type="radio" name="exportFormat" value="pdf" checked>
                            PDF Document
                        </label>
                        <label>
                            <input type="radio" name="exportFormat" value="txt">
                            Plain Text
                        </label>
                        <label>
                            <input type="radio" name="exportFormat" value="json">
                            JSON with Signatures
                        </label>
                    </div>
                </div>

                <div class="export-preview">
                    <h4>Export Preview</h4>
                    <div class="preview-box">
                        <div>Document: ${this.document.title}</div>
                        <div>Signatures: ${this.document.signatures.length}/3</div>
                        <div>Format: <span id="formatPreview">PDF</span></div>
                        <div>Size: <span id="sizePreview">~45 KB</span></div>
                    </div>
                </div>

                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Cancel</button>
                    <button onclick="documentSigningDemo.exportDocument()" class="primary">Export</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();

    // Update format preview
    document.querySelectorAll('input[name="exportFormat"]').forEach(radio => {
      radio.addEventListener('change', (e) => {
        document.getElementById('formatPreview').textContent = e.target.value.toUpperCase();
        this.updateExportSize(e.target.value);
      });
    });
  }

  showAuditModal() {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Audit Trail</h2>
                <div class="audit-timeline">
                    <div class="timeline-item">
                        <div class="timeline-marker">📝</div>
                        <div class="timeline-content">
                            <h4>Document Created</h4>
                            <p>${this.document.createdAt.toLocaleString()}</p>
                            <p>Created by: System</p>
                        </div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-marker">🔐</div>
                        <div class="timeline-content">
                            <h4>Document Hash Generated</h4>
                            <p>${this.document.createdAt.toLocaleString()}</p>
                            <p>Hash: ${this.document.hash}</p>
                        </div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-marker">📋</div>
                        <div class="timeline-content">
                            <h4>Sent for Signatures</h4>
                            <p>${this.document.createdAt.toLocaleString()}</p>
                            <p>Recipients: Alice Johnson, Bob Smith, Carol Davis</p>
                        </div>
                    </div>
                </div>

                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Close</button>
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
                .modal {
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
                    padding: 30px;
                    border-radius: 15px;
                    max-width: 600px;
                    width: 90%;
                    max-height: 80vh;
                    overflow-y: auto;
                }
                .signer-info, .signature-options, .signature-preview,
                .verification-input, .export-options, .export-preview {
                    margin-bottom: 20px;
                }
                .option-group label {
                    display: block;
                    margin-bottom: 10px;
                    cursor: pointer;
                }
                .option-group input[type="checkbox"],
                .option-group input[type="radio"] {
                    margin-right: 10px;
                }
                .preview-box, .result-box {
                    background: #f8f9fa;
                    padding: 15px;
                    border-radius: 8px;
                    font-family: monospace;
                    font-size: 14px;
                }
                .verification-input label {
                    display: block;
                    margin-bottom: 5px;
                    font-weight: 600;
                }
                .verification-input input,
                .verification-input textarea {
                    width: 100%;
                    padding: 10px;
                    border: 2px solid #e0e0e0;
                    border-radius: 8px;
                    font-size: 14px;
                    margin-bottom: 15px;
                }
                .verification-input textarea {
                    min-height: 80px;
                    resize: vertical;
                }
                .timeline-item {
                    display: flex;
                    margin-bottom: 20px;
                    align-items: flex-start;
                }
                .timeline-marker {
                    font-size: 24px;
                    margin-right: 15px;
                    margin-top: 5px;
                }
                .timeline-content h4 {
                    margin: 0 0 5px 0;
                    color: #333;
                }
                .timeline-content p {
                    margin: 0 0 5px 0;
                    color: #666;
                    font-size: 14px;
                }
                .modal-actions {
                    display: flex;
                    gap: 10px;
                    justify-content: flex-end;
                    margin-top: 20px;
                }
                .modal-actions button {
                    padding: 10px 20px;
                    border: none;
                    border-radius: 8px;
                    cursor: pointer;
                    font-size: 16px;
                }
                .modal-actions button.primary {
                    background: #007bff;
                    color: white;
                }
            `;
      document.head.appendChild(style);
    }
  }

  updateAlgoPreview() {
    const falcon = document.getElementById('falconSig').checked;
    const dilithium = document.getElementById('dilithiumSig').checked;
    const sphincs = document.getElementById('sphincsSig').checked;

    const algorithms = [];
    if (falcon) algorithms.push('Falcon-512');
    if (dilithium) algorithms.push('Dilithium-65');
    if (sphincs) algorithms.push('SPHINCS+');

    document.getElementById('algoPreview').textContent = algorithms.join(', ') || 'None selected';
  }

  updateExportSize(format) {
    const sizes = {
      'pdf': '~45 KB',
      'txt': '~12 KB',
      'json': '~28 KB'
    };
    document.getElementById('sizePreview').textContent = sizes[format] || '~45 KB';
  }

  signDocument() {
    const user = this.users[this.currentUser];
    const falcon = document.getElementById('falconSig').checked;
    const dilithium = document.getElementById('dilithiumSig').checked;
    const sphincs = document.getElementById('sphincsSig').checked;

    if (!falcon && !dilithium && !sphincs) {
      alert('Please select at least one signature algorithm');
      return;
    }

    // Simulate signature process
    this.showNotification('Signing document...', 'info');

    setTimeout(() => {
      const signature = {
        id: Date.now(),
        signer: user.name,
        role: user.role,
        timestamp: new Date(),
        algorithms: [],
        status: 'signed'
      };

      if (falcon) signature.algorithms.push('Falcon-512');
      if (dilithium) signature.algorithms.push('Dilithium-65');
      if (sphincs) signature.algorithms.push('SPHINCS+');

      this.document.signatures.push(signature);
      this.users[this.currentUser].status = 'signed';

      this.updateSignatureHistory();
      this.updateDocumentInfo();

      document.querySelector('.modal').remove();
      this.showNotification('Document signed successfully!', 'success');
    }, 2000);
  }

  verifySignature() {
    const hash = document.getElementById('verifyHash').value;
    const signature = document.getElementById('verifySignature').value;
    const publicKey = document.getElementById('verifyPublicKey').value;

    if (!hash || !signature || !publicKey) {
      alert('Please fill in all fields');
      return;
    }

    // Simulate verification process
    this.showNotification('Verifying signature...', 'info');

    setTimeout(() => {
      const resultBox = document.getElementById('resultBox');
      const verificationResult = document.getElementById('verificationResult');

      // Simulate verification result
      const isValid = Math.random() > 0.3; // 70% success rate for demo

      if (isValid) {
        resultBox.innerHTML = `
                    <div style="color: #28a745; font-weight: bold;">✅ Signature Verified Successfully</div>
                    <div>Document Hash: ${hash}</div>
                    <div>Signature Algorithm: Falcon-512</div>
                    <div>Verification Time: ${new Date().toLocaleString()}</div>
                    <div>Security Level: Post-Quantum Secure</div>
                `;
      } else {
        resultBox.innerHTML = `
                    <div style="color: #dc3545; font-weight: bold;">❌ Signature Verification Failed</div>
                    <div>Document Hash: ${hash}</div>
                    <div>Error: Invalid signature or corrupted data</div>
                    <div>Verification Time: ${new Date().toLocaleString()}</div>
                `;
      }

      verificationResult.style.display = 'block';
    }, 1500);
  }

  exportDocument() {
    const format = document.querySelector('input[name="exportFormat"]:checked').value;

    // Simulate export process
    this.showNotification('Exporting document...', 'info');

    setTimeout(() => {
      const filename = `document-${this.document.id}-${format}.${format === 'json' ? 'json' : format}`;

      // Create download link
      const blob = new Blob([`Exported ${this.document.title} in ${format.toUpperCase()} format`], { type: 'text/plain' });
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      a.click();
      window.URL.revokeObjectURL(url);

      document.querySelector('.modal').remove();
      this.showNotification(`Document exported as ${filename}`, 'success');
    }, 1000);
  }

  loadDocumentContent() {
    // Load document content from textarea
    const textarea = document.getElementById('documentContent');
    this.document.content = textarea.value;

    // Generate document hash
    this.document.hash = this.generateHash(this.document.content);

    // Update display
    this.updateDocumentInfo();
  }

  generateHash(content) {
    // Simple hash generation for demo
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
      const char = content.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash).toString(16).padStart(8, '0') + '...';
  }

  updateDocumentInfo() {
    const totalSignatures = this.document.signatures.length;
    const requiredSignatures = Object.keys(this.users).length;

    document.querySelector('.info-card .value').textContent = `${totalSignatures}/${requiredSignatures}`;

    // Update document status
    if (totalSignatures === requiredSignatures) {
      this.document.status = 'completed';
      document.querySelector('.document-header .value').textContent = 'Status: Completed';
      document.querySelector('.document-header .value').style.color = '#28a745';
    }
  }

  updateSignatureHistory() {
    const container = document.querySelector('.signature-history');

    const historyHTML = Object.entries(this.users).map(([userId, user]) => {
      const signature = this.document.signatures.find(s => s.signer === user.name);
      const status = signature ? 'signed' : 'pending';
      const timestamp = signature ? signature.timestamp.toLocaleString() : '-';
      const algorithms = signature ? signature.algorithms.join(', ') : '-';

      return `
                <div class="signature-item ${status}">
                    <div>
                        <div style="font-weight: 600;">${status === 'signed' ? 'Signed' : 'Pending'}: ${user.name}</div>
                        <div style="font-size: 0.9em; color: #666;">${user.role} - ${algorithms}</div>
                    </div>
                    <div style="text-align: right;">
                        <div style="color: ${status === 'signed' ? '#28a745' : '#ffc107'}; font-weight: 600;">
                            ${status === 'signed' ? 'Signed' : 'Pending'}
                        </div>
                        <div style="font-size: 0.9em; color: #666;">${timestamp}</div>
                    </div>
                </div>
            `;
    }).join('');

    container.innerHTML = `
            <h3>Signature History</h3>
            ${historyHTML}
        `;
  }

  updatePQCStatus() {
    // Simulate real-time PQC status updates
    setInterval(() => {
      const statuses = ['active', 'warning', 'danger'];
      const randomStatus = statuses[Math.floor(Math.random() * statuses.length)];

      // Randomly update one PQC algorithm status
      const algorithms = Object.keys(this.pqcStatus);
      const randomAlgo = algorithms[Math.floor(Math.random() * algorithms.length)];
      this.pqcStatus[randomAlgo] = randomStatus;

      this.updatePQCStatusDisplay();
    }, 15000); // Update every 15 seconds
  }

  updatePQCStatusDisplay() {
    const statusContainer = document.querySelector('.pqc-status');
    if (!statusContainer) return;

    const statusHTML = Object.entries(this.pqcStatus).map(([algo, status]) => {
      const displayName = {
        'falcon': 'Falcon-512',
        'dilithium': 'Dilithium-65',
        'sphincs': 'SPHINCS+',
        'sha3': 'SHA3-256'
      }[algo];

      return `<div><span class="status-indicator status-${status}"></span>${displayName}: ${status}</div>`;
    }).join('');

    statusContainer.innerHTML = `
            <h4>🔒 PQC Security Status</h4>
            ${statusHTML}
        `;
  }

  startRealTimeUpdates() {
    // Simulate real-time updates
    setInterval(() => {
      // Update document hash periodically
      this.document.hash = this.generateHash(this.document.content + Date.now());
      this.updateDocumentInfo();
    }, 30000); // Update every 30 seconds
  }

  showNotification(message, type = 'info') {
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;

    // Add notification styles
    if (!document.getElementById('notification-styles')) {
      const style = document.createElement('style');
      style.id = 'notification-styles';
      style.textContent = `
                .notification {
                    position: fixed;
                    top: 20px;
                    right: 20px;
                    padding: 15px 20px;
                    border-radius: 8px;
                    color: white;
                    font-weight: 600;
                    z-index: 10001;
                    animation: slideIn 0.3s ease;
                }
                .notification-success { background: #28a745; }
                .notification-info { background: #17a2b8; }
                .notification-warning { background: #ffc107; color: #212529; }
                .notification-error { background: #dc3545; }
                @keyframes slideIn {
                    from { transform: translateX(100%); opacity: 0; }
                    to { transform: translateX(0); opacity: 1; }
                }
            `;
      document.head.appendChild(style);
    }

    document.body.appendChild(notification);

    // Remove notification after 3 seconds
    setTimeout(() => {
      notification.remove();
    }, 3000);
  }
}

// Initialize document signing demo
let documentSigningDemo;

// Navigation function
function goBack() {
  window.location.href = '../portal/index.html';
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
  documentSigningDemo = new DocumentSigning();
});
