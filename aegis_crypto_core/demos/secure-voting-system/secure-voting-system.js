// Secure Voting System Demo - Aegis PQC Demo
// Tamper-Proof Voting Platform with Post-Quantum Cryptography

class SecureVotingSystemDemo {
  constructor() {
    this.currentElection = 'presidential';
    this.elections = {
      presidential: {
        name: 'Presidential Election',
        description: '2024 General Election - Vote for President',
        status: 'open',
        candidates: [
          {
            id: 'candidate_1',
            name: 'Sarah Johnson',
            party: 'democrat',
            platform: 'Progressive policies for climate change and healthcare reform'
          },
          {
            id: 'candidate_2',
            name: 'Michael Chen',
            party: 'republican',
            platform: 'Economic growth and traditional values'
          },
          {
            id: 'candidate_3',
            name: 'Alex Rodriguez',
            party: 'independent',
            platform: 'Bipartisan solutions and fiscal responsibility'
          }
        ]
      },
      senate: {
        name: 'Senate Election',
        description: 'District 12 Senate Race',
        status: 'open',
        candidates: [
          {
            id: 'candidate_4',
            name: 'Dr. Emily Davis',
            party: 'democrat',
            platform: 'Education reform and healthcare access'
          },
          {
            id: 'candidate_5',
            name: 'Robert Wilson',
            party: 'republican',
            platform: 'Business development and tax reform'
          }
        ]
      },
      mayor: {
        name: 'Mayoral Election',
        description: 'City of Springfield Mayor',
        status: 'open',
        candidates: [
          {
            id: 'candidate_6',
            name: 'Lisa Thompson',
            party: 'democrat',
            platform: 'Urban development and public safety'
          },
          {
            id: 'candidate_7',
            name: 'James Brown',
            party: 'republican',
            platform: 'Infrastructure and economic growth'
          }
        ]
      }
    };
    this.selectedCandidate = null;
    this.voterVerified = false;
    this.voteCast = false;
    this.pqcStatus = {
      mldsa: 'active',
      slhdsa: 'active',
      sha3: 'active',
      zk: 'active'
    };

    this.init();
  }

  init() {
    this.setupEventListeners();
    this.renderCandidates();
    this.updatePQCStatus();
    this.startRealTimeUpdates();
  }

  setupEventListeners() {
    // Election selection
    document.querySelectorAll('.election-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectElection(e.currentTarget.dataset.election);
      });
    });

    // Action cards
    document.getElementById('verifyVoterBtn').addEventListener('click', () => this.showVerifyVoterModal());
    document.getElementById('castVoteBtn').addEventListener('click', () => this.castVote());
    document.getElementById('verifyVoteBtn').addEventListener('click', () => this.verifyVote());
    document.getElementById('viewResultsBtn').addEventListener('click', () => this.viewResults());

    // Submit vote button
    document.getElementById('submitVoteBtn').addEventListener('click', () => this.submitVote());

    // Voter verification form
    document.getElementById('verifyVoterForm').addEventListener('submit', (e) => {
      e.preventDefault();
      this.verifyVoter();
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

  selectElection(electionId) {
    // Update active state
    document.querySelectorAll('.election-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-election="${electionId}"]`).classList.add('active');

    this.currentElection = electionId;
    this.updateElectionDisplay();
    this.renderCandidates();
  }

  updateElectionDisplay() {
    const election = this.elections[this.currentElection];
    document.getElementById('electionName').textContent = election.name;
    document.getElementById('electionInfo').textContent = election.description;
  }

  renderCandidates() {
    const candidatesList = document.getElementById('candidatesList');
    if (!candidatesList) return;

    const election = this.elections[this.currentElection];

    candidatesList.innerHTML = election.candidates.map(candidate => `
      <div class="candidate-item" data-candidate="${candidate.id}">
        <div class="candidate-header">
          <div class="candidate-name">${candidate.name}</div>
          <div class="candidate-party ${candidate.party}">${candidate.party.toUpperCase()}</div>
        </div>
        <div class="candidate-details">
          <div class="candidate-platform">${candidate.platform}</div>
        </div>
      </div>
    `).join('');

    // Add click handlers for candidate selection
    document.querySelectorAll('.candidate-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectCandidate(e.currentTarget.dataset.candidate);
      });
    });
  }

  selectCandidate(candidateId) {
    // Remove previous selection
    document.querySelectorAll('.candidate-item').forEach(item => {
      item.classList.remove('selected');
    });

    // Add selection to clicked candidate
    document.querySelector(`[data-candidate="${candidateId}"]`).classList.add('selected');

    this.selectedCandidate = candidateId;

    // Enable submit button if voter is verified
    const submitBtn = document.getElementById('submitVoteBtn');
    if (this.voterVerified && !this.voteCast) {
      submitBtn.disabled = false;
    }
  }

  showVerifyVoterModal() {
    console.log('🆔 Opening voter verification modal...');
    document.getElementById('verifyVoterModal').style.display = 'block';
  }

  closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
  }

  async verifyVoter() {
    const voterId = document.getElementById('voterId').value;
    const voterSsn = document.getElementById('voterSsn').value;

    console.log('🆔 Verifying voter identity...');

    const verifySteps = [
      '🔐 Encrypting voter data with PQC...',
      '✍️ Signing verification with ML-DSA...',
      '🔍 Checking voter registration...',
      '🛡️ Validating identity...',
      '✅ Voter verified successfully!'
    ];

    for (const step of verifySteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 400));
    }

    this.voterVerified = true;
    this.closeModal('verifyVoterModal');

    // Enable submit button if candidate is selected
    const submitBtn = document.getElementById('submitVoteBtn');
    if (this.selectedCandidate && !this.voteCast) {
      submitBtn.disabled = false;
    }

    // Reset form
    document.getElementById('verifyVoterForm').reset();

    console.log('✅ Voter verified successfully!');
  }

  async castVote() {
    if (!this.voterVerified) {
      alert('Please verify your voter identity first.');
      return;
    }

    if (!this.selectedCandidate) {
      alert('Please select a candidate first.');
      return;
    }

    console.log('🗳️ Casting vote...');
    this.submitVote();
  }

  async submitVote() {
    if (!this.voterVerified || !this.selectedCandidate || this.voteCast) {
      return;
    }

    console.log('🗳️ Submitting secure vote...');

    const voteSteps = [
      '🔐 Encrypting vote with PQC...',
      '✍️ Signing with ML-DSA-65...',
      '🔒 Creating zero-knowledge proof...',
      '🛡️ Transmitting to secure blockchain...',
      '✅ Vote cast successfully!'
    ];

    for (const step of voteSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    this.voteCast = true;
    document.getElementById('submitVoteBtn').disabled = true;
    document.getElementById('submitVoteBtn').textContent = 'Vote Cast Successfully';

    // Disable candidate selection
    document.querySelectorAll('.candidate-item').forEach(item => {
      item.style.pointerEvents = 'none';
      item.style.opacity = '0.6';
    });

    console.log('✅ Vote cast successfully!');
  }

  async verifyVote() {
    if (!this.voteCast) {
      alert('No vote has been cast yet.');
      return;
    }

    console.log('🔍 Verifying vote...');

    const verifySteps = [
      '🔐 Decrypting vote with PQC...',
      '✍️ Verifying ML-DSA signature...',
      '🔍 Checking blockchain record...',
      '🛡️ Confirming vote integrity...',
      '✅ Vote verified successfully!'
    ];

    for (const step of verifySteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    const election = this.elections[this.currentElection];
    const candidate = election.candidates.find(c => c.id === this.selectedCandidate);

    alert(`Vote Verification Successful!\n\nElection: ${election.name}\nCandidate: ${candidate.name}\nParty: ${candidate.party.toUpperCase()}\n\nYour vote has been securely recorded and verified.`);

    console.log('✅ Vote verified successfully!');
  }

  async viewResults() {
    console.log('📊 Viewing election results...');

    const resultsSteps = [
      '🔐 Decrypting results with PQC...',
      '✍️ Verifying result signatures...',
      '🔍 Aggregating vote counts...',
      '📊 Generating secure report...',
      '✅ Results displayed!'
    ];

    for (const step of resultsSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 350));
    }

    const election = this.elections[this.currentElection];
    let resultsText = `Election Results: ${election.name}\n\n`;

    election.candidates.forEach((candidate, index) => {
      const votes = Math.floor(Math.random() * 1000) + 500; // Random vote count
      resultsText += `${index + 1}. ${candidate.name} (${candidate.party.toUpperCase()}): ${votes} votes\n`;
    });

    resultsText += '\nTotal votes cast: ' + election.candidates.reduce((sum, candidate, index) => {
      return sum + Math.floor(Math.random() * 1000) + 500;
    }, 0);

    alert(resultsText);

    console.log('✅ Results displayed successfully!');
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
    // Simulate real-time updates
    setInterval(() => {
      // Update voting status
      if (Math.random() > 0.99) {
        const statusElement = document.querySelector('.voting-header div:last-child div:last-child');
        if (statusElement) {
          const statuses = ['Voting Status: Open', 'Voting Status: Active', 'Voting Status: Secure'];
          statusElement.textContent = statuses[Math.floor(Math.random() * statuses.length)];
        }
      }
    }, 10000);
  }
}

// Global functions for modal management
function closeModal(modalId) {
  document.getElementById(modalId).style.display = 'none';
}
