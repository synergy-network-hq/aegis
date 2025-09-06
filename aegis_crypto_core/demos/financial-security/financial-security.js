// Financial Security Demo - Aegis PQC Demo
// Quantum-Resistant Banking Suite with Post-Quantum Cryptography

class FinancialSecurityDemo {
  constructor() {
    this.currentAccount = 'checking';
    this.accounts = {
      checking: {
        balance: 12450.00,
        type: 'Checking',
        number: '****1234',
        available: 12450.00
      },
      savings: {
        balance: 45230.00,
        type: 'Savings',
        number: '****5678',
        available: 45230.00
      },
      investment: {
        balance: 78900.00,
        type: 'Investment',
        number: '****9012',
        available: 78900.00
      }
    };
    this.transactions = [];
    this.pqcStatus = {
      mlkem: 'active',
      mldsa: 'active',
      slhdsa: 'active',
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
    // Account selection
    document.querySelectorAll('.account-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectAccount(e.currentTarget.dataset.account);
      });
    });

    // Action cards
    document.getElementById('transferBtn').addEventListener('click', () => this.showTransferModal());
    document.getElementById('payBillBtn').addEventListener('click', () => this.payBill());
    document.getElementById('investBtn').addEventListener('click', () => this.manageInvestments());
    document.getElementById('loanBtn').addEventListener('click', () => this.applyForLoan());

    // Transfer form
    document.getElementById('transferForm').addEventListener('submit', (e) => {
      e.preventDefault();
      this.processTransfer();
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

  selectAccount(accountType) {
    // Update active state
    document.querySelectorAll('.account-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-account="${accountType}"]`).classList.add('active');

    this.currentAccount = accountType;
    this.updateAccountDisplay();
  }

  updateAccountDisplay() {
    const account = this.accounts[this.currentAccount];
    document.getElementById('accountTitle').textContent = `${account.type} Account`;
    document.getElementById('accountNumber').textContent = `Account: ${account.number}`;
    document.getElementById('accountBalance').textContent = `$${account.balance.toLocaleString('en-US', { minimumFractionDigits: 2 })}`;
    document.getElementById('accountType').textContent = 'Available Balance';
  }

  loadTransactionHistory() {
    this.transactions = [
      {
        id: 'txn_001',
        type: 'credit',
        amount: 2500.00,
        description: 'Direct Deposit - Salary',
        date: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000), // 2 days ago
        status: 'completed',
        account: 'checking'
      },
      {
        id: 'txn_002',
        type: 'debit',
        amount: -850.00,
        description: 'Mortgage Payment',
        date: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000), // 5 days ago
        status: 'completed',
        account: 'checking'
      },
      {
        id: 'txn_003',
        type: 'debit',
        amount: -120.50,
        description: 'Grocery Store Purchase',
        date: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000), // 1 week ago
        status: 'completed',
        account: 'checking'
      },
      {
        id: 'txn_004',
        type: 'credit',
        amount: 5000.00,
        description: 'Investment Transfer',
        date: new Date(Date.now() - 10 * 24 * 60 * 60 * 1000), // 10 days ago
        status: 'completed',
        account: 'savings'
      },
      {
        id: 'txn_005',
        type: 'debit',
        amount: -200.00,
        description: 'Utility Bill Payment',
        date: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000), // 2 weeks ago
        status: 'completed',
        account: 'checking'
      }
    ];

    this.renderTransactions();
  }

  renderTransactions() {
    const transactionsList = document.getElementById('transactionsList');
    if (!transactionsList) return;

    const accountTransactions = this.transactions.filter(txn => txn.account === this.currentAccount);

    transactionsList.innerHTML = accountTransactions.map(txn => `
      <div class="transaction-item ${txn.type}">
        <div>
          <div style="font-weight: 600;">${txn.description}</div>
          <div style="font-size: 0.9em; color: #666;">${txn.date.toLocaleDateString()}</div>
        </div>
        <div style="text-align: right;">
          <div style="color: ${txn.type === 'credit' ? '#28a745' : '#dc3545'}; font-weight: 600;">
            ${txn.type === 'credit' ? '+' : ''}$${txn.amount.toLocaleString('en-US', { minimumFractionDigits: 2 })}
          </div>
          <div style="font-size: 0.9em; color: #666; text-transform: capitalize;">${txn.status}</div>
        </div>
      </div>
    `).join('');
  }

  showTransferModal() {
    console.log('💸 Opening transfer modal...');
    document.getElementById('transferModal').style.display = 'block';
  }

  closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
  }

  async processTransfer() {
    const toAccount = document.getElementById('toAccount').value;
    const amount = parseFloat(document.getElementById('amount').value);
    const memo = document.getElementById('memo').value;

    console.log('💸 Processing secure transfer...');

    const transferSteps = [
      '🔐 Generating PQC session keys...',
      '✍️ Signing transaction with ML-DSA...',
      '🔒 Encrypting transfer data...',
      '🌐 Transmitting via secure channel...',
      '✅ Transfer completed successfully!'
    ];

    for (const step of transferSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 400));
    }

    // Add new transaction
    const newTransaction = {
      id: `txn_${Date.now()}`,
      type: 'debit',
      amount: -amount,
      description: `Transfer to ${toAccount}${memo ? ` - ${memo}` : ''}`,
      date: new Date(),
      status: 'completed',
      account: this.currentAccount
    };

    this.transactions.unshift(newTransaction);
    this.accounts[this.currentAccount].balance -= amount;
    this.accounts[this.currentAccount].available -= amount;

    this.renderTransactions();
    this.updateAccountDisplay();
    this.closeModal('transferModal');

    // Reset form
    document.getElementById('transferForm').reset();

    console.log('✅ Transfer completed successfully!');
  }

  async payBill() {
    console.log('📋 Processing bill payment...');

    const paymentSteps = [
      '🔐 Authenticating with PQC...',
      '✍️ Signing payment with ML-DSA...',
      '🔒 Encrypting payment data...',
      '📋 Scheduling payment...',
      '✅ Bill payment scheduled!'
    ];

    for (const step of paymentSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    // Simulate bill payment
    const billAmount = Math.random() * 200 + 50; // Random amount between $50-$250
    const newTransaction = {
      id: `txn_${Date.now()}`,
      type: 'debit',
      amount: -billAmount,
      description: 'Scheduled Bill Payment',
      date: new Date(),
      status: 'pending',
      account: this.currentAccount
    };

    this.transactions.unshift(newTransaction);
    this.renderTransactions();

    console.log('✅ Bill payment scheduled successfully!');
  }

  async manageInvestments() {
    console.log('📈 Managing investments...');

    const investmentSteps = [
      '🔐 Securing investment data...',
      '✍️ Signing with quantum-resistant keys...',
      '📈 Analyzing portfolio...',
      '🔄 Executing trades...',
      '✅ Investment management complete!'
    ];

    for (const step of investmentSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 350));
    }

    // Simulate investment activity
    const investmentChange = (Math.random() - 0.5) * 1000; // Random change between -$500 and +$500
    this.accounts.investment.balance += investmentChange;

    const newTransaction = {
      id: `txn_${Date.now()}`,
      type: investmentChange > 0 ? 'credit' : 'debit',
      amount: investmentChange,
      description: 'Portfolio Update',
      date: new Date(),
      status: 'completed',
      account: 'investment'
    };

    this.transactions.unshift(newTransaction);
    this.renderTransactions();

    console.log('✅ Investment management completed!');
  }

  async applyForLoan() {
    console.log('🏠 Processing loan application...');

    const loanSteps = [
      '🔐 Securing application data...',
      '✍️ Signing with PQC algorithms...',
      '🔍 Credit verification...',
      '📊 Risk assessment...',
      '✅ Loan application submitted!'
    ];

    for (const step of loanSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 400));
    }

    console.log('✅ Loan application submitted successfully!');
  }

  updatePQCStatus() {
    // Simulate PQC status updates
    setInterval(() => {
      const statusElements = document.querySelectorAll('.status-indicator');
      statusElements.forEach(element => {
        // Randomly update status indicators
        if (Math.random() > 0.95) {
          element.classList.toggle('status-secure');
          element.classList.toggle('status-warning');
        }
      });
    }, 5000);
  }

  startRealTimeUpdates() {
    // Simulate real-time balance updates
    setInterval(() => {
      // Small random fluctuations in investment account
      if (this.currentAccount === 'investment') {
        const fluctuation = (Math.random() - 0.5) * 10;
        this.accounts.investment.balance += fluctuation;
        this.updateAccountDisplay();
      }
    }, 10000);
  }
}

// Global functions for modal management
function closeModal(modalId) {
  document.getElementById(modalId).style.display = 'none';
}
