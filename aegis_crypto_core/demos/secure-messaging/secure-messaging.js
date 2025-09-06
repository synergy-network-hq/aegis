// Secure Messaging Demo - Aegis PQC Demo
// Post-Quantum Cryptography for End-to-End Encrypted Communication

class SecureMessagingDemo {
    constructor() {
        this.currentUser = null;
        this.selectedUser = null;
        this.messages = [];
        this.keyPairs = new Map();
        this.initializeDemo();
    }

    async initializeDemo() {
        console.log('🔐 Initializing Secure Messaging Demo...');

        // Initialize UI
        this.setupEventListeners();
        this.generateUsers();
        this.updateUI();

        // Simulate PQC key generation
        await this.generatePQCKeys();

        console.log('✅ Secure Messaging Demo initialized successfully!');
    }

    setupEventListeners() {
        // User selection
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('user-item')) {
                const userId = e.target.dataset.userId;
                this.selectUser(userId);
            }
        });

        // Send message
        const sendBtn = document.getElementById('sendBtn');
        const messageInput = document.getElementById('messageInput');

        if (sendBtn) {
            sendBtn.addEventListener('click', () => this.sendMessage());
        }

        if (messageInput) {
            messageInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this.sendMessage();
                }
            });
        }

        // Key management
        const generateKeysBtn = document.getElementById('generateKeysBtn');
        if (generateKeysBtn) {
            generateKeysBtn.addEventListener('click', () => this.generatePQCKeys());
        }
    }

    generateUsers() {
        const users = [
            { id: 'alice', name: 'Alice Johnson', status: 'online', avatar: '👩‍💼' },
            { id: 'bob', name: 'Bob Smith', status: 'online', avatar: '👨‍💻' },
            { id: 'charlie', name: 'Charlie Brown', status: 'away', avatar: '👨‍🔬' },
            { id: 'diana', name: 'Diana Prince', status: 'online', avatar: '👩‍🚀' }
        ];

        this.users = users;
        this.currentUser = users[0]; // Alice is current user
        this.renderUserList();
    }

    renderUserList() {
        const userList = document.getElementById('userList');
        if (!userList) return;

        userList.innerHTML = this.users.map(user => `
            <div class="user-item" data-user-id="${user.id}">
                <div>${user.avatar} ${user.name}</div>
                <div style="font-size: 0.9em; opacity: 0.8;">${user.status}</div>
            </div>
        `).join('');
    }

    selectUser(userId) {
        this.selectedUser = this.users.find(u => u.id === userId);

        // Update UI
        document.querySelectorAll('.user-item').forEach(item => {
            item.classList.remove('active');
        });
        document.querySelector(`[data-user-id="${userId}"]`).classList.add('active');

        // Load conversation
        this.loadConversation(userId);
        this.updateUI();
    }

    loadConversation(userId) {
        // Simulate loading conversation history
        const conversationId = this.getConversationId(this.currentUser.id, userId);
        this.messages = this.getConversationHistory(conversationId);
        this.renderMessages();
    }

    getConversationId(user1, user2) {
        return [user1, user2].sort().join('-');
    }

    getConversationHistory(conversationId) {
        // Simulate conversation history
        const histories = {
            'alice-bob': [
                { id: 1, sender: 'alice', content: 'Hey Bob! Ready for our secure meeting?', timestamp: new Date(Date.now() - 3600000), encrypted: true },
                { id: 2, sender: 'bob', content: 'Absolutely! The PQC encryption is working perfectly.', timestamp: new Date(Date.now() - 3500000), encrypted: true },
                { id: 3, sender: 'alice', content: 'Great! Let\'s discuss the new security protocols.', timestamp: new Date(Date.now() - 3400000), encrypted: true }
            ],
            'alice-charlie': [
                { id: 1, sender: 'charlie', content: 'Alice, I need to share some sensitive research data.', timestamp: new Date(Date.now() - 7200000), encrypted: true },
                { id: 2, sender: 'alice', content: 'Perfect timing! Our PQC system is ready for that.', timestamp: new Date(Date.now() - 7100000), encrypted: true }
            ],
            'alice-diana': [
                { id: 1, sender: 'diana', content: 'Mission briefing ready. Quantum-resistant protocols activated.', timestamp: new Date(Date.now() - 1800000), encrypted: true },
                { id: 2, sender: 'alice', content: 'Roger that! All systems secure.', timestamp: new Date(Date.now() - 1700000), encrypted: true }
            ]
        };

        return histories[conversationId] || [];
    }

    renderMessages() {
        const messagesContainer = document.getElementById('messagesContainer');
        if (!messagesContainer) return;

        messagesContainer.innerHTML = this.messages.map(msg => {
            const isCurrentUser = msg.sender === this.currentUser.id;
            const sender = this.users.find(u => u.id === msg.sender);

            return `
                <div class="message ${isCurrentUser ? 'sent' : 'received'}">
                    <div class="message-header">
                        <span>${sender?.avatar || '👤'} ${sender?.name || msg.sender}</span>
                        <span>${this.formatTime(msg.timestamp)}</span>
                    </div>
                    <div class="message-content">
                        ${msg.content}
                    </div>
                </div>
            `;
        }).join('');

        // Scroll to bottom
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }

    async sendMessage() {
        const messageInput = document.getElementById('messageInput');
        const content = messageInput?.value?.trim();

        if (!content || !this.selectedUser) return;

        // Create message object
        const message = {
            id: Date.now(),
            sender: this.currentUser.id,
            content: content,
            timestamp: new Date(),
            encrypted: true
        };

        // Add to messages
        this.messages.push(message);

        // Clear input
        messageInput.value = '';

        // Render messages
        this.renderMessages();

        // Simulate PQC encryption
        await this.simulatePQCEncryption(message);

        // Update UI
        this.updateUI();

        console.log(`📤 Message sent to ${this.selectedUser.name}: ${content}`);
    }

    async simulatePQCEncryption(message) {
        // Simulate PQC encryption process
        const encryptionSteps = [
            '🔐 Generating ML-KEM key pair...',
            '🔑 Performing key encapsulation...',
            '📝 Encrypting message with AES-256...',
            '✍️ Signing with ML-DSA...',
            '✅ Message encrypted and signed!'
        ];

        for (const step of encryptionSteps) {
            console.log(step);
            await new Promise(resolve => setTimeout(resolve, 200));
        }
    }

    async generatePQCKeys() {
        console.log('🔑 Generating Post-Quantum Cryptographic Keys...');

        const keyGenerationSteps = [
            '🎲 Generating random seed...',
            '🔐 Creating ML-KEM-768 key pair...',
            '✍️ Generating ML-DSA-65 signature key...',
            '🛡️ Creating FN-DSA-512 backup key...',
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
                    <span class="status-icon">🔐</span>
                    <span>ML-KEM-768: Active</span>
                </div>
                <div class="status-item">
                    <span class="status-icon">✍️</span>
                    <span>ML-DSA-65: Active</span>
                </div>
                <div class="status-item">
                    <span class="status-icon">🛡️</span>
                    <span>FN-DSA-512: Active</span>
                </div>
            `;
        }
    }

    updateUI() {
        // Update header
        const headerTitle = document.getElementById('headerTitle');
        if (headerTitle && this.selectedUser) {
            headerTitle.textContent = `Chat with ${this.selectedUser.name}`;
        }

        // Update message input placeholder
        const messageInput = document.getElementById('messageInput');
        if (messageInput) {
            messageInput.placeholder = this.selectedUser ?
                `Type a message to ${this.selectedUser.name}...` :
                'Select a user to start chatting...';
            messageInput.disabled = !this.selectedUser;
        }

        // Update send button
        const sendBtn = document.getElementById('sendBtn');
        if (sendBtn) {
            sendBtn.disabled = !this.selectedUser;
        }
    }

    formatTime(timestamp) {
        return timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    // Demo-specific methods
    showPQCInfo() {
        const info = `
🔐 Post-Quantum Cryptography in Secure Messaging:

• ML-KEM-768: Key encapsulation for secure key exchange
• ML-DSA-65: Digital signatures for message authentication
• FN-DSA-512: Additional signature layer for enhanced security
• SHA3-256: Message hashing and integrity verification

All communications are protected against quantum computer attacks!
        `;
        alert(info);
    }

    showSecurityMetrics() {
        const metrics = `
📊 Security Metrics:

• Encryption Strength: 256-bit quantum-resistant
• Key Size: 1,184 bytes (ML-KEM-768)
• Signature Size: 2,701 bytes (ML-DSA-65)
• Hash Algorithm: SHA3-256
• Forward Secrecy: Enabled
• Perfect Forward Secrecy: Enabled

Your messages are protected by the latest NIST standards!
        `;
        alert(metrics);
    }
}

// Initialize demo when page loads
document.addEventListener('DOMContentLoaded', () => {
    window.secureMessagingDemo = new SecureMessagingDemo();

    // Add global functions for demo controls
    window.showPQCInfo = () => window.secureMessagingDemo.showPQCInfo();
    window.showSecurityMetrics = () => window.secureMessagingDemo.showSecurityMetrics();
});

console.log('🚀 Secure Messaging Demo loaded successfully!');
