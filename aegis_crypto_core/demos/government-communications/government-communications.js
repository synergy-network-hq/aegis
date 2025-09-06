// Government Communications Demo - Aegis PQC Demo
// Classified Government Network with Maximum Security Post-Quantum Cryptography

class GovernmentCommunicationsDemo {
  constructor() {
    this.currentDepartment = 'defense';
    this.departments = {
      defense: {
        name: 'Department of Defense',
        classification: 'TOP SECRET',
        clearance: 'TS/SCI',
        lastAccess: new Date()
      },
      state: {
        name: 'Department of State',
        classification: 'SECRET',
        clearance: 'SECRET',
        lastAccess: new Date()
      },
      intelligence: {
        name: 'Intelligence Agency',
        classification: 'TOP SECRET',
        clearance: 'TS/SCI',
        lastAccess: new Date()
      },
      homeland: {
        name: 'Homeland Security',
        classification: 'SECRET',
        clearance: 'SECRET',
        lastAccess: new Date()
      }
    };
    this.classifiedMessages = [];
    this.pqcStatus = {
      mlkem: 'active',
      mldsa: 'active',
      fndsa: 'active',
      slhdsa: 'active'
    };

    this.init();
  }

  init() {
    this.setupEventListeners();
    this.loadClassifiedMessages();
    this.updatePQCStatus();
    this.startRealTimeUpdates();
  }

  setupEventListeners() {
    // Department selection
    document.querySelectorAll('.department-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectDepartment(e.currentTarget.dataset.department);
      });
    });

    // Action cards
    document.getElementById('sendMessageBtn').addEventListener('click', () => this.showSendMessageModal());
    document.getElementById('shareDocumentBtn').addEventListener('click', () => this.shareDocument());
    document.getElementById('emergencyAlertBtn').addEventListener('click', () => this.sendEmergencyAlert());
    document.getElementById('secureCallBtn').addEventListener('click', () => this.initiateSecureCall());

    // Send message form
    document.getElementById('sendMessageForm').addEventListener('submit', (e) => {
      e.preventDefault();
      this.sendClassifiedMessage();
    });

    // Modal close
    document.querySelectorAll('.close').forEach(closeBtn => {
      closeBtn.addEventListener('click', (e) => {
        const modal = e.target.closest('.modal');
        this.closeModal(modal.id);
      });
    });

    // Click outside modal to close
    window.addEventListener('click', (e) => {
      if (e.target.classList.contains('modal')) {
        this.closeModal(e.target.id);
      }
    });
  }

  selectDepartment(departmentId) {
    // Update active state
    document.querySelectorAll('.department-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-department="${departmentId}"]`).classList.add('active');

    this.currentDepartment = departmentId;
    this.updateDepartmentDisplay();
  }

  updateDepartmentDisplay() {
    const department = this.departments[this.currentDepartment];
    document.getElementById('departmentName').textContent = department.name;
    document.getElementById('departmentInfo').textContent = 'Secure Government Communications Network';

    // Update security badge
    const securityBadge = document.querySelector('.security-badge');
    securityBadge.textContent = `🔒 ${department.classification}`;
    securityBadge.className = `security-badge ${department.classification.toLowerCase().replace(' ', '-')}`;

    // Update last access time
    const now = new Date();
    const timeDiff = Math.floor((now - department.lastAccess) / 1000 / 60);
    document.querySelector('.government-header div:last-child div:last-child').textContent =
      `Last Access: ${timeDiff} min ago`;

    department.lastAccess = now;
    this.renderClassifiedMessages();
  }

  loadClassifiedMessages() {
    this.classifiedMessages = [
      {
        id: 'msg_001',
        classification: 'top-secret',
        subject: 'Strategic Defense Initiative Update',
        content: 'Latest intelligence reports indicate potential threats to national security. Immediate action required.',
        sender: 'Gen. Sarah Mitchell',
        recipient: 'Department of Defense',
        date: new Date(Date.now() - 2 * 60 * 60 * 1000),
        department: 'defense'
      },
      {
        id: 'msg_002',
        classification: 'secret',
        subject: 'Diplomatic Relations Assessment',
        content: 'Analysis of current diplomatic situation and recommendations for foreign policy adjustments.',
        sender: 'Amb. Robert Chen',
        recipient: 'Department of State',
        date: new Date(Date.now() - 4 * 60 * 60 * 1000),
        department: 'state'
      }
    ];

    this.renderClassifiedMessages();
  }

  renderClassifiedMessages() {
    const messagesList = document.getElementById('messagesList');
    if (!messagesList) return;

    const departmentMessages = this.classifiedMessages.filter(msg => msg.department === this.currentDepartment);

    messagesList.innerHTML = departmentMessages.map(msg => `
      <div class="message-item ${msg.classification}">
        <div class="message-header">
          <div class="message-classification">${msg.subject}</div>
          <div class="classification-badge ${msg.classification}">${msg.classification.toUpperCase().replace('-', ' ')}</div>
        </div>
        <div class="message-details">
          <div class="message-sender">From: ${msg.sender}</div>
          <div class="message-dates">
            <span>Date: ${msg.date.toLocaleDateString()}</span>
            <span>Time: ${msg.date.toLocaleTimeString()}</span>
          </div>
          <div style="margin-top: 10px; font-style: italic;">${msg.content}</div>
        </div>
        <div class="message-actions">
          <button class="btn btn-sm btn-primary" onclick="window.governmentDemo.viewMessage('${msg.id}')">
            View Details
          </button>
          <button class="btn btn-sm btn-danger" onclick="window.governmentDemo.secureDelete('${msg.id}')">
            Secure Delete
          </button>
        </div>
      </div>
    `).join('');
  }

  showSendMessageModal() {
    console.log('📨 Opening send message modal...');
    document.getElementById('sendMessageModal').style.display = 'block';
  }

  closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
  }

  async sendClassifiedMessage() {
    const recipient = document.getElementById('messageRecipient').value;
    const classification = document.getElementById('messageClassification').value;
    const subject = document.getElementById('messageSubject').value;
    const content = document.getElementById('messageContent').value;

    console.log('📨 Sending classified message...');

    const sendSteps = [
      '🔐 Encrypting with maximum security PQC...',
      '✍️ Signing with ML-DSA-87...',
      '🔒 Applying FN-DSA-1024 signatures...',
      '🛡️ Transmitting via secure channel...',
      '✅ Classified message sent successfully!'
    ];

    for (const step of sendSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    // Add new message
    const newMessage = {
      id: `msg_${Date.now()}`,
      classification: classification,
      subject: subject,
      content: content,
      sender: this.departments[this.currentDepartment].name,
      recipient: this.departments[recipient].name,
      date: new Date(),
      department: this.currentDepartment
    };

    this.classifiedMessages.unshift(newMessage);
    this.renderClassifiedMessages();
    this.closeModal('sendMessageModal');

    // Reset form
    document.getElementById('sendMessageForm').reset();

    console.log('✅ Classified message sent successfully!');
  }

  async shareDocument() {
    console.log('📄 Sharing classified document...');
    console.log('✅ Document shared securely!');
  }

  async sendEmergencyAlert() {
    console.log('🚨 Sending emergency alert...');
    console.log('✅ Emergency alert transmitted successfully!');
  }

  async initiateSecureCall() {
    console.log('📞 Initiating secure call...');
    console.log('✅ Secure call established successfully!');
  }

  async viewMessage(messageId) {
    const message = this.classifiedMessages.find(m => m.id === messageId);
    if (!message) return;

    console.log(`🔍 Viewing classified message: ${message.subject}`);
    alert(`Classified Message: ${message.subject}\n\nFrom: ${message.sender}\nContent: ${message.content}`);
  }

  async secureDelete(messageId) {
    const message = this.classifiedMessages.find(m => m.id === messageId);
    if (!message) return;

    console.log(`🗑️ Securely deleting message: ${message.subject}`);
    this.classifiedMessages = this.classifiedMessages.filter(m => m.id !== messageId);
    this.renderClassifiedMessages();
    console.log('✅ Message securely deleted!');
  }

  updatePQCStatus() {
    // Simulate PQC status updates
    setInterval(() => {
      const statusElements = document.querySelectorAll('.status-indicator');
      statusElements.forEach(element => {
        if (Math.random() > 0.95) {
          element.classList.toggle('status-secure');
          element.classList.toggle('status-warning');
        }
      });
    }, 5000);
  }

  startRealTimeUpdates() {
    // Simulate real-time updates
    setInterval(() => {
      Object.values(this.departments).forEach(department => {
        if (Math.random() > 0.98) {
          department.lastAccess = new Date();
        }
      });
    }, 10000);
  }
}

// Global functions for modal management
function closeModal(modalId) {
  document.getElementById(modalId).style.display = 'none';
}
