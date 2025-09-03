// Blockchain Wallet Demo - Aegis PQC Demo
// This demo showcases PQC-secured cryptocurrency wallet operations

class BlockchainWallet {
  constructor() {
    this.currentWallet = 'btc';
    this.wallets = {
      btc: { balance: 0.025, currency: 'BTC', address: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh' },
      eth: { balance: 2.5, currency: 'ETH', address: '0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6' },
      aegis: { balance: 1000, currency: 'AEG', address: 'aeg1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh' }
    };
    this.transactions = [];
    this.pqcStatus = {
      kyber: 'active',
      falcon: 'active',
      dilithium: 'active',
      hqc: 'active'
    };

    this.init();
  }

  init() {
    this.setupEventListeners();
    this.loadTransactionHistory();
    this.updatePQCStatus();
    this.startRealTimeUpdates();
  }

  setupEventListeners() {
    // Wallet selection
    document.querySelectorAll('.wallet-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectWallet(e.currentTarget.dataset.wallet);
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

  selectWallet(walletType) {
    // Update active state
    document.querySelectorAll('.wallet-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-wallet="${walletType}"]`).classList.add('active');

    this.currentWallet = walletType;
    this.updateWalletDisplay();
  }

  updateWalletDisplay() {
    const wallet = this.wallets[this.currentWallet];
    const header = document.querySelector('.wallet-header');

    header.innerHTML = `
            <div>
                <h1>${this.getCurrencyName(wallet.currency)} Wallet</h1>
                <p style="color: #666; margin: 0;">Address: ${wallet.address}</p>
            </div>
            <div style="text-align: right;">
                <div style="font-size: 2em; font-weight: bold; color: #28a745;">${wallet.balance} ${wallet.currency}</div>
                <div style="color: #666;">≈ $${this.getUSDValue(wallet.balance, wallet.currency)} USD</div>
            </div>
        `;

    this.updateTransactionHistory();
  }

  getCurrencyName(currency) {
    const names = {
      'BTC': 'Bitcoin',
      'ETH': 'Ethereum',
      'AEG': 'Aegis Token'
    };
    return names[currency] || currency;
  }

  getUSDValue(amount, currency) {
    const rates = {
      'BTC': 50000,
      'ETH': 3000,
      'AEG': 0.50
    };
    return Math.round(amount * rates[currency]);
  }

  handleAction(action) {
    switch (action) {
      case '📤 send':
        this.showSendModal();
        break;
      case '📥 receive':
        this.showReceiveModal();
        break;
      case '🔄 swap':
        this.showSwapModal();
        break;
      case '🎯 stake':
        this.showStakeModal();
        break;
    }
  }

  showSendModal() {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Send ${this.wallets[this.currentWallet].currency}</h2>
                <div class="form-group">
                    <label>To Address:</label>
                    <input type="text" id="sendAddress" placeholder="Enter recipient address">
                </div>
                <div class="form-group">
                    <label>Amount:</label>
                    <input type="number" id="sendAmount" step="0.000001" max="${this.wallets[this.currentWallet].balance}">
                </div>
                <div class="form-group">
                    <label>Fee:</label>
                    <select id="sendFee">
                        <option value="low">Low (0.00001 ${this.wallets[this.currentWallet].currency})</option>
                        <option value="medium" selected>Medium (0.00005 ${this.wallets[this.currentWallet].currency})</option>
                        <option value="high">High (0.0001 ${this.wallets[this.currentWallet].currency})</option>
                    </select>
                </div>
                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Cancel</button>
                    <button onclick="walletDemo.sendTransaction()" class="primary">Send</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();
  }

  showReceiveModal() {
    const wallet = this.wallets[this.currentWallet];
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Receive ${wallet.currency}</h2>
                <div class="qr-code">
                    <div style="width: 200px; height: 200px; background: #f0f0f0; display: flex; align-items: center; justify-content: center; margin: 20px auto; border-radius: 10px;">
                        QR Code for ${wallet.address}
                    </div>
                </div>
                <div class="address-display">
                    <label>Your Address:</label>
                    <div class="address-box">
                        <code>${wallet.address}</code>
                        <button onclick="navigator.clipboard.writeText('${wallet.address}')">Copy</button>
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

  showSwapModal() {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Swap Tokens</h2>
                <div class="form-group">
                    <label>From:</label>
                    <select id="swapFrom">
                        <option value="btc">Bitcoin (BTC)</option>
                        <option value="eth">Ethereum (ETH)</option>
                        <option value="aegis">Aegis Token (AEG)</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>To:</label>
                    <select id="swapTo">
                        <option value="eth">Ethereum (ETH)</option>
                        <option value="btc" selected>Bitcoin (BTC)</option>
                        <option value="aegis">Aegis Token (AEG)</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>Amount:</label>
                    <input type="number" id="swapAmount" step="0.000001">
                </div>
                <div class="swap-preview">
                    <div>Rate: 1 BTC = 16.67 ETH</div>
                    <div>You'll receive: <span id="swapReceive">0 ETH</span></div>
                </div>
                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Cancel</button>
                    <button onclick="walletDemo.executeSwap()" class="primary">Swap</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();

    // Update swap preview
    document.getElementById('swapAmount').addEventListener('input', (e) => {
      this.updateSwapPreview();
    });
  }

  showStakeModal() {
    const modal = document.createElement('modal');
    modal.className = 'modal';
    modal.innerHTML = `
            <div class="modal-content">
                <h2>Stake Tokens</h2>
                <div class="stake-info">
                    <div class="stake-rate">
                        <h3>Current APY: 12.5%</h3>
                        <p>Stake your tokens to earn passive income</p>
                    </div>
                </div>
                <div class="form-group">
                    <label>Amount to Stake:</label>
                    <input type="number" id="stakeAmount" step="0.000001" max="${this.wallets[this.currentWallet].balance}">
                </div>
                <div class="stake-preview">
                    <div>Daily Reward: <span id="dailyReward">0</span> ${this.wallets[this.currentWallet].currency}</div>
                    <div>Monthly Reward: <span id="monthlyReward">0</span> ${this.wallets[this.currentWallet].currency}</div>
                </div>
                <div class="modal-actions">
                    <button onclick="this.closest('.modal').remove()">Cancel</button>
                    <button onclick="walletDemo.stakeTokens()" class="primary">Stake</button>
                </div>
            </div>
        `;

    document.body.appendChild(modal);
    this.addModalStyles();

    // Update stake preview
    document.getElementById('stakeAmount').addEventListener('input', (e) => {
      this.updateStakePreview();
    });
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
                    max-width: 500px;
                    width: 90%;
                    max-height: 80vh;
                    overflow-y: auto;
                }
                .form-group {
                    margin-bottom: 20px;
                }
                .form-group label {
                    display: block;
                    margin-bottom: 5px;
                    font-weight: 600;
                }
                .form-group input,
                .form-group select {
                    width: 100%;
                    padding: 10px;
                    border: 2px solid #e0e0e0;
                    border-radius: 8px;
                    font-size: 16px;
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
                .address-box {
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    background: #f8f9fa;
                    padding: 15px;
                    border-radius: 8px;
                    margin-top: 10px;
                }
                .address-box code {
                    flex: 1;
                    word-break: break-all;
                }
                .address-box button {
                    padding: 8px 16px;
                    background: #007bff;
                    color: white;
                    border: none;
                    border-radius: 6px;
                    cursor: pointer;
                }
            `;
      document.head.appendChild(style);
    }
  }

  sendTransaction() {
    const address = document.getElementById('sendAddress').value;
    const amount = parseFloat(document.getElementById('sendAmount').value);
    const fee = document.getElementById('sendFee').value;

    if (!address || !amount || amount <= 0) {
      alert('Please enter valid address and amount');
      return;
    }

    if (amount > this.wallets[this.currentWallet].balance) {
      alert('Insufficient balance');
      return;
    }

    // Simulate transaction
    this.processTransaction('sent', amount, address);
    document.querySelector('.modal').remove();

    // Show success message
    this.showNotification('Transaction sent successfully!', 'success');
  }

  executeSwap() {
    const from = document.getElementById('swapFrom').value;
    const to = document.getElementById('swapTo').value;
    const amount = parseFloat(document.getElementById('swapAmount').value);

    if (!amount || amount <= 0) {
      alert('Please enter valid amount');
      return;
    }

    // Simulate swap
    this.showNotification('Swap executed successfully!', 'success');
    document.querySelector('.modal').remove();
  }

  stakeTokens() {
    const amount = parseFloat(document.getElementById('stakeAmount').value);

    if (!amount || amount <= 0) {
      alert('Please enter valid amount');
      return;
    }

    if (amount > this.wallets[this.currentWallet].balance) {
      alert('Insufficient balance');
      return;
    }

    // Simulate staking
    this.showNotification('Tokens staked successfully!', 'success');
    document.querySelector('.modal').remove();
  }

  updateSwapPreview() {
    const amount = parseFloat(document.getElementById('swapAmount').value) || 0;
    const from = document.getElementById('swapFrom').value;
    const to = document.getElementById('swapTo').value;

    // Simple conversion rates
    const rates = {
      'btc': { 'eth': 16.67, 'aegis': 2000 },
      'eth': { 'btc': 0.06, 'aegis': 120 },
      'aegis': { 'btc': 0.0005, 'eth': 0.0083 }
    };

    const receiveAmount = amount * (rates[from]?.[to] || 0);
    document.getElementById('swapReceive').textContent = `${receiveAmount.toFixed(6)} ${this.getCurrencyName(to)}`;
  }

  updateStakePreview() {
    const amount = parseFloat(document.getElementById('stakeAmount').value) || 0;
    const apy = 0.125; // 12.5%

    const dailyReward = (amount * apy) / 365;
    const monthlyReward = dailyReward * 30;

    document.getElementById('dailyReward').textContent = dailyReward.toFixed(6);
    document.getElementById('monthlyReward').textContent = monthlyReward.toFixed(6);
  }

  processTransaction(type, amount, address) {
    const transaction = {
      id: Date.now(),
      type: type,
      amount: amount,
      address: address,
      timestamp: new Date(),
      status: 'confirmed'
    };

    this.transactions.unshift(transaction);
    this.updateTransactionHistory();

    // Update balance
    if (type === 'sent') {
      this.wallets[this.currentWallet].balance -= amount;
    } else if (type === 'received') {
      this.wallets[this.currentWallet].balance += amount;
    }

    this.updateWalletDisplay();
  }

  loadTransactionHistory() {
    // Sample transactions
    this.transactions = [
      {
        id: 1,
        type: 'received',
        amount: 0.025,
        address: 'Exchange',
        timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
        status: 'confirmed'
      },
      {
        id: 2,
        type: 'sent',
        amount: 0.005,
        address: 'Merchant',
        timestamp: new Date(Date.now() - 24 * 60 * 60 * 1000), // 1 day ago
        status: 'confirmed'
      }
    ];

    this.updateTransactionHistory();
  }

  updateTransactionHistory() {
    const container = document.querySelector('.transaction-history');
    const wallet = this.wallets[this.currentWallet];

    // Filter transactions for current wallet
    const walletTransactions = this.transactions.filter(t =>
      t.currency === wallet.currency || !t.currency
    );

    const historyHTML = walletTransactions.map(tx => `
            <div class="transaction-item ${tx.type}">
                <div>
                    <div style="font-weight: 600;">${tx.type === 'sent' ? 'Sent to' : 'Received from'} ${tx.address}</div>
                    <div style="font-size: 0.9em; color: #666;">${this.formatTimestamp(tx.timestamp)}</div>
                </div>
                <div style="text-align: right;">
                    <div style="color: ${tx.type === 'sent' ? '#dc3545' : '#28a745'}; font-weight: 600;">
                        ${tx.type === 'sent' ? '-' : '+'}${tx.amount} ${wallet.currency}
                    </div>
                    <div style="font-size: 0.9em; color: #666;">${tx.status}</div>
                </div>
            </div>
        `).join('');

    container.innerHTML = `
            <h3>Transaction History</h3>
            ${historyHTML}
        `;
  }

  formatTimestamp(timestamp) {
    const now = new Date();
    const diff = now - timestamp;

    if (diff < 60 * 1000) return 'Just now';
    if (diff < 60 * 60 * 1000) return `${Math.floor(diff / (60 * 1000))} minutes ago`;
    if (diff < 24 * 60 * 60 * 1000) return `${Math.floor(diff / (60 * 60 * 1000))} hours ago`;
    return `${Math.floor(diff / (24 * 60 * 60 * 1000))} days ago`;
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
    }, 10000); // Update every 10 seconds
  }

  updatePQCStatusDisplay() {
    const statusContainer = document.querySelector('.pqc-status');
    if (!statusContainer) return;

    const statusHTML = Object.entries(this.pqcStatus).map(([algo, status]) => {
      const displayName = {
        'kyber': 'Kyber-768',
        'falcon': 'Falcon-512',
        'dilithium': 'Dilithium-65',
        'hqc': 'HQC-192'
      }[algo];

      return `<div><span class="status-indicator status-${status}"></span>${displayName}: ${status}</div>`;
    }).join('');

    statusContainer.innerHTML = `
            <h4>🔒 PQC Security Status</h4>
            ${statusHTML}
        `;
  }

  startRealTimeUpdates() {
    // Simulate real-time price updates
    setInterval(() => {
      // Random price fluctuations
      Object.keys(this.wallets).forEach(currency => {
        const fluctuation = (Math.random() - 0.5) * 0.02; // ±1% change
        this.wallets[currency].balance *= (1 + fluctuation);
      });

      this.updateWalletDisplay();
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

// Initialize wallet demo
let walletDemo;

// Navigation function
function goBack() {
  window.location.href = '../portal/index.html';
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
  walletDemo = new BlockchainWallet();
});
