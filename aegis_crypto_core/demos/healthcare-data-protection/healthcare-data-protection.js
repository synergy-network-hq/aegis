// Healthcare Data Protection Demo - Aegis PQC Demo
// HIPAA-Compliant Health Vault with Post-Quantum Cryptography

class HealthcareDataProtectionDemo {
  constructor() {
    this.currentPatient = 'john_doe';
    this.patients = {
      john_doe: {
        name: 'John Doe',
        dob: '1985-03-15',
        id: 'P123456',
        lastAccess: new Date()
      },
      jane_smith: {
        name: 'Jane Smith',
        dob: '1990-07-22',
        id: 'P789012',
        lastAccess: new Date()
      },
      bob_wilson: {
        name: 'Bob Wilson',
        dob: '1978-11-08',
        id: 'P345678',
        lastAccess: new Date()
      }
    };
    this.medicalRecords = [];
    this.pqcStatus = {
      mlkem: 'active',
      mldsa: 'active',
      slhdsa: 'active',
      hipaa: 'compliant'
    };

    this.init();
  }

  init() {
    this.setupEventListeners();
    this.loadMedicalRecords();
    this.updatePQCStatus();
    this.startRealTimeUpdates();
  }

  setupEventListeners() {
    // Patient selection
    document.querySelectorAll('.patient-item').forEach(item => {
      item.addEventListener('click', (e) => {
        this.selectPatient(e.currentTarget.dataset.patient);
      });
    });

    // Action cards
    document.getElementById('addRecordBtn').addEventListener('click', () => this.showAddRecordModal());
    document.getElementById('prescribeBtn').addEventListener('click', () => this.issuePrescription());
    document.getElementById('scheduleBtn').addEventListener('click', () => this.scheduleAppointment());
    document.getElementById('emergencyBtn').addEventListener('click', () => this.emergencyAccess());

    // Add record form
    document.getElementById('addRecordForm').addEventListener('submit', (e) => {
      e.preventDefault();
      this.addMedicalRecord();
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

  selectPatient(patientId) {
    // Update active state
    document.querySelectorAll('.patient-item').forEach(item => {
      item.classList.remove('active');
    });
    document.querySelector(`[data-patient="${patientId}"]`).classList.add('active');

    this.currentPatient = patientId;
    this.updatePatientDisplay();
  }

  updatePatientDisplay() {
    const patient = this.patients[this.currentPatient];
    document.getElementById('patientName').textContent = patient.name;
    document.getElementById('patientInfo').textContent = `DOB: ${patient.dob} | Patient ID: ${patient.id}`;

    // Update last access time
    const now = new Date();
    const timeDiff = Math.floor((now - patient.lastAccess) / 1000 / 60);
    document.querySelector('.healthcare-header div:last-child div:last-child').textContent =
      `Last Access: ${timeDiff} min ago`;

    patient.lastAccess = now;
    this.renderMedicalRecords();
  }

  loadMedicalRecords() {
    this.medicalRecords = [
      {
        id: 'rec_001',
        type: 'consultation',
        priority: 'routine',
        notes: 'Annual physical examination. Patient reports feeling well. Blood pressure normal, weight stable.',
        doctor: 'Dr. Sarah Johnson',
        date: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000), // 2 days ago
        patient: 'john_doe'
      },
      {
        id: 'rec_002',
        type: 'lab_result',
        priority: 'routine',
        notes: 'Complete blood count results within normal range. Cholesterol levels improved.',
        doctor: 'Dr. Sarah Johnson',
        date: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000), // 5 days ago
        patient: 'john_doe'
      },
      {
        id: 'rec_003',
        type: 'prescription',
        priority: 'routine',
        notes: 'Prescribed vitamin D supplement for bone health. Dosage: 1000 IU daily.',
        doctor: 'Dr. Sarah Johnson',
        date: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000), // 1 week ago
        patient: 'john_doe'
      },
      {
        id: 'rec_004',
        type: 'diagnosis',
        priority: 'routine',
        notes: 'Diagnosed with seasonal allergies. Recommended over-the-counter antihistamines.',
        doctor: 'Dr. Michael Chen',
        date: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000), // 2 weeks ago
        patient: 'john_doe'
      },
      {
        id: 'rec_005',
        type: 'treatment',
        priority: 'urgent',
        notes: 'Emergency treatment for minor injury. Stitches applied, wound cleaned and dressed.',
        doctor: 'Dr. Emily Rodriguez',
        date: new Date(Date.now() - 21 * 24 * 60 * 60 * 1000), // 3 weeks ago
        patient: 'john_doe'
      }
    ];

    this.renderMedicalRecords();
  }

  renderMedicalRecords() {
    const recordsList = document.getElementById('recordsList');
    if (!recordsList) return;

    const patientRecords = this.medicalRecords.filter(record => record.patient === this.currentPatient);

    recordsList.innerHTML = patientRecords.map(record => `
      <div class="record-item ${record.priority}">
        <div class="record-header">
          <div class="record-type">${this.formatRecordType(record.type)}</div>
          <div class="record-priority ${record.priority}">${record.priority.toUpperCase()}</div>
        </div>
        <div class="record-details">
          <div class="record-doctor">Dr. ${record.doctor}</div>
          <div class="record-dates">
            <span>Date: ${record.date.toLocaleDateString()}</span>
          </div>
          <div style="margin-top: 10px; font-style: italic;">${record.notes}</div>
        </div>
        <div class="record-actions">
          <button class="btn btn-sm btn-primary" onclick="window.healthcareDemo.viewRecord('${record.id}')">
            View Details
          </button>
          <button class="btn btn-sm btn-success" onclick="window.healthcareDemo.shareRecord('${record.id}')">
            Share Securely
          </button>
        </div>
      </div>
    `).join('');
  }

  formatRecordType(type) {
    const types = {
      'consultation': 'Consultation',
      'lab_result': 'Lab Result',
      'prescription': 'Prescription',
      'diagnosis': 'Diagnosis',
      'treatment': 'Treatment'
    };
    return types[type] || type;
  }

  showAddRecordModal() {
    console.log('📋 Opening add record modal...');
    document.getElementById('addRecordModal').style.display = 'block';
  }

  closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
  }

  async addMedicalRecord() {
    const recordType = document.getElementById('recordType').value;
    const recordPriority = document.getElementById('recordPriority').value;
    const recordNotes = document.getElementById('recordNotes').value;

    console.log('📋 Adding secure medical record...');

    const addSteps = [
      '🔐 Encrypting record with PQC...',
      '✍️ Signing with ML-DSA...',
      '🔒 Securing HIPAA compliance...',
      '📋 Storing in secure vault...',
      '✅ Medical record added successfully!'
    ];

    for (const step of addSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 400));
    }

    // Add new record
    const newRecord = {
      id: `rec_${Date.now()}`,
      type: recordType,
      priority: recordPriority,
      notes: recordNotes,
      doctor: 'Dr. Sarah Johnson',
      date: new Date(),
      patient: this.currentPatient
    };

    this.medicalRecords.unshift(newRecord);
    this.renderMedicalRecords();
    this.closeModal('addRecordModal');

    // Reset form
    document.getElementById('addRecordForm').reset();

    console.log('✅ Medical record added successfully!');
  }

  async issuePrescription() {
    console.log('💊 Issuing secure prescription...');

    const prescriptionSteps = [
      '🔐 Verifying doctor credentials...',
      '✍️ Signing prescription with PQC...',
      '🔒 Encrypting medication data...',
      '💊 Transmitting to pharmacy...',
      '✅ Prescription issued successfully!'
    ];

    for (const step of prescriptionSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 350));
    }

    // Add prescription record
    const prescription = {
      id: `rec_${Date.now()}`,
      type: 'prescription',
      priority: 'routine',
      notes: 'Prescription issued for routine medication. Patient advised to follow dosage instructions.',
      doctor: 'Dr. Sarah Johnson',
      date: new Date(),
      patient: this.currentPatient
    };

    this.medicalRecords.unshift(prescription);
    this.renderMedicalRecords();

    console.log('✅ Prescription issued successfully!');
  }

  async scheduleAppointment() {
    console.log('📅 Scheduling secure appointment...');

    const scheduleSteps = [
      '🔐 Checking availability...',
      '✍️ Signing appointment with PQC...',
      '🔒 Encrypting appointment data...',
      '📅 Adding to calendar...',
      '✅ Appointment scheduled successfully!'
    ];

    for (const step of scheduleSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    console.log('✅ Appointment scheduled successfully!');
  }

  async emergencyAccess() {
    console.log('🚨 Emergency access granted...');

    const emergencySteps = [
      '🚨 Emergency access protocol activated...',
      '🔐 Bypassing normal security for emergency...',
      '✍️ Logging emergency access...',
      '🔒 Maintaining audit trail...',
      '✅ Emergency access granted!'
    ];

    for (const step of emergencySteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 200));
    }

    // Add emergency access record
    const emergencyRecord = {
      id: `rec_${Date.now()}`,
      type: 'treatment',
      priority: 'emergency',
      notes: 'Emergency access granted. Patient requires immediate medical attention.',
      doctor: 'Emergency Team',
      date: new Date(),
      patient: this.currentPatient
    };

    this.medicalRecords.unshift(emergencyRecord);
    this.renderMedicalRecords();

    console.log('✅ Emergency access granted successfully!');
  }

  async viewRecord(recordId) {
    const record = this.medicalRecords.find(r => r.id === recordId);
    if (!record) return;

    console.log(`🔍 Viewing record: ${record.type}`);

    const viewSteps = [
      '🔐 Decrypting record with PQC...',
      '✍️ Verifying ML-DSA signature...',
      '🔒 Confirming HIPAA compliance...',
      '📋 Displaying secure record...',
      '✅ Record accessed successfully!'
    ];

    for (const step of viewSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 250));
    }

    // Show record details in alert (in real app, would be a proper modal)
    alert(`Record Details:\n\nType: ${this.formatRecordType(record.type)}\nPriority: ${record.priority}\nDoctor: ${record.doctor}\nDate: ${record.date.toLocaleDateString()}\n\nNotes:\n${record.notes}`);
  }

  async shareRecord(recordId) {
    const record = this.medicalRecords.find(r => r.id === recordId);
    if (!record) return;

    console.log(`🔗 Sharing record securely: ${record.type}`);

    const shareSteps = [
      '🔐 Encrypting record for sharing...',
      '✍️ Signing with PQC algorithms...',
      '🔒 Creating secure share link...',
      '📋 Transmitting securely...',
      '✅ Record shared successfully!'
    ];

    for (const step of shareSteps) {
      console.log(step);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    console.log('✅ Record shared successfully!');
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
      // Update last access times
      Object.values(this.patients).forEach(patient => {
        if (Math.random() > 0.98) {
          patient.lastAccess = new Date();
        }
      });
    }, 10000);
  }
}

// Global functions for modal management
function closeModal(modalId) {
  document.getElementById(modalId).style.display = 'none';
}
