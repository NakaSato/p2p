# Engineering Department Operations Manual
## P2P Energy Trading System - Single Validator Network

**System:** University P2P Energy Trading Platform
**Network Type:** Private, Single Engineering Department Validator
**Deployment:** Engineering Complex Only
**Authority:** Engineering Department Full Control

---

## Table of Contents
1. [System Overview](#system-overview)
2. [Engineering Department Authority](#engineering-department-authority)
3. [Daily Operations](#daily-operations)
4. [Technical Management](#technical-management)
5. [User Management](#user-management)
6. [Emergency Procedures](#emergency-procedures)
7. [Maintenance & Monitoring](#maintenance--monitoring)
8. [Academic Integration](#academic-integration)
9. [Future Expansion](#future-expansion)

---

## System Overview

### Network Architecture
- **Validator Count**: 1 (Engineering Department controlled)
- **Network Type**: Private Solana blockchain cluster
- **Connectivity**: Air-gapped campus network only
- **Authority Model**: Engineering Department sole authority
- **Consensus**: Single validator (no multi-party consensus required)

### Engineering Complex Coverage
- **Building**: Engineering Complex
- **Smart Meters**: 15 units (ENG_001 through ENG_015)
- **Energy Sources**: 50kW solar array, grid connection, 10kWh battery storage
- **Participants**: 250 (50 faculty, 75 grad students, 125 undergrads)

### Blockchain Programs
| Program | Program ID | Function |
|---------|------------|----------|
| Registry | RegEngDeptEnergyP2P1234567890123456789 | User and device management |
| Energy Token | EnergyTokenEngDept1234567890123456789 | SPL tokens for energy representation |
| Trading | TradingEngDeptP2P1234567890123456789 | Order matching and market clearing |
| Oracle | OracleEngDeptAMI1234567890123456789 | AMI data integration and validation |
| Governance | GovernanceEngDeptPoA1234567890123456789 | Engineering Department authority controls |

---

## Engineering Department Authority

### Full System Authority
The Engineering Department maintains complete authority over all system operations:

#### REC Validation Authority
- **Certificate Issuance**: Engineering Department validates all renewable energy certificates
- **Token Minting**: Automated minting based on Engineering-validated solar generation
- **Energy Quality**: Engineering standards for renewable vs. grid energy classification
- **Sustainability Coordination**: Optional coordination with Sustainability Office for policies

#### Network Administration
- **Validator Management**: Engineering IT manages the sole validator node
- **Program Updates**: Engineering Department approves all smart contract upgrades
- **Network Configuration**: Engineering controls all network parameters and settings
- **Access Control**: Engineering Department manages all user authentication

#### Market Operations
- **Market Clearing**: Automated hourly clearing under Engineering oversight
- **Price Discovery**: Engineering Department sets market operation parameters
- **Emergency Controls**: Engineering can pause/resume market operations
- **Settlement**: Automated settlement with Engineering Department monitoring

### Decision Making Process
1. **Immediate Decisions**: Engineering IT staff (validator operations, user support)
2. **Policy Decisions**: Engineering Department faculty committee
3. **Technical Upgrades**: Engineering IT with department approval
4. **Emergency Actions**: Engineering IT immediate authority, department notification

---

## Daily Operations

### Morning Procedures (8:00 AM)
1. **Validator Status Check**
   ```bash
   # Check validator health
   solana validator-info get --url http://localhost:8899
   solana cluster-version --url http://localhost:8899
   
   # Check account balances
   solana balance --url http://localhost:8899
   ```

2. **Smart Meter Data Verification**
   ```bash
   # Review overnight meter readings
   curl http://localhost:8899/ami-data/summary/last24h
   
   # Check oracle submissions
   solana account OracleEngDeptAMI1234567890123456789 --url http://localhost:8899
   ```

3. **Market Operations Review**
   ```bash
   # Check overnight trading activity
   curl http://localhost:8899/trading/summary/last24h
   
   # Verify automated market clearing
   solana account TradingEngDeptP2P1234567890123456789 --url http://localhost:8899
   ```

### Ongoing Monitoring
- **System Health**: Continuous monitoring via Engineering IT dashboard
- **User Activity**: Real-time trading and energy generation monitoring
- **Network Performance**: Transaction throughput and latency tracking
- **Storage Management**: Ledger size and account storage monitoring

### End-of-Day Procedures (5:00 PM)
1. **Daily Report Generation**
2. **Backup Verification**
3. **Performance Metrics Review**
4. **Next-Day Preparation**

---

## Technical Management

### Validator Operations

#### Starting the Validator
```bash
#!/bin/bash
# Engineering Department Validator Startup
cd /opt/campus-blockchain/validators/engineering
./start-engineering-validator.sh
```

#### Monitoring Commands
```bash
# Check validator status
solana validator-info get

# Monitor validator logs
tail -f /var/log/campus-blockchain/engineering/validator.log

# Check network performance
solana ping --url http://localhost:8899
```

#### Backup Procedures
```bash
# Daily ledger backup
cp -r /opt/campus-blockchain/data/engineering /backup/$(date +%Y%m%d)

# Weekly full system backup
tar -czf /backup/weekly/engineering-system-$(date +%Y%m%d).tar.gz \
  /opt/campus-blockchain/validators/engineering \
  /opt/campus-blockchain/data/engineering
```

### Program Management

#### Deploying Updates
```bash
# Build and deploy program updates
cd /opt/campus-blockchain/programs
anchor build
anchor deploy --program-name [program-name] --provider.cluster http://localhost:8899
```

#### Configuration Updates
```bash
# Update system configuration
solana program invoke --program-id GovernanceEngDeptPoA1234567890123456789 \
  --data [configuration-data] --url http://localhost:8899
```

### Performance Optimization
- **Transaction Throughput**: Monitor and optimize for >100 TPS
- **Storage Efficiency**: Regular cleanup of old account data
- **Network Latency**: Maintain <2 second transaction confirmation
- **Resource Utilization**: Monitor CPU, memory, and storage usage

---

## User Management

### User Registration Process
1. **Engineering Authentication**: Verify Engineering Department affiliation
2. **Account Creation**: Create blockchain account with Engineering Department validation
3. **Meter Assignment**: Link user to specific Engineering Complex smart meters
4. **Training Completion**: Ensure user completes system training module

### Engineering Department Participants

#### Faculty Members (50 users)
- **Office Assignments**: Individual office meter assignments
- **Research Accounts**: Special accounts for research data access
- **Authority Level**: Standard trading with optional research data access

#### Graduate Students (75 users)
- **Lab Assignments**: Shared lab meter access
- **Research Integration**: Integration with thesis/research projects
- **Authority Level**: Standard trading with supervised research access

#### Undergraduate Students (125 users)
- **Classroom Integration**: Course-related energy trading exercises
- **Learning Accounts**: Limited trading for educational purposes
- **Authority Level**: Supervised trading with educational limits

### Account Management
```bash
# Create new user account
spl-token create-account EnergyTokenEngDept1234567890123456789 --owner [user-pubkey]

# Assign meter to user
solana program invoke --program-id RegEngDeptEnergyP2P1234567890123456789 \
  --data [assign-meter-data]

# Check user balance
spl-token balance EnergyTokenEngDept1234567890123456789 --owner [user-pubkey]
```

---

## Emergency Procedures

### System Emergency Response

#### Level 1: Service Disruption
- **Issue**: Minor service interruptions, slow performance
- **Response**: Engineering IT immediate investigation
- **Authority**: Engineering IT staff
- **Timeline**: Resolve within 1 hour

#### Level 2: Security Incident
- **Issue**: Unauthorized access attempts, suspicious transactions
- **Response**: Immediate system pause, security investigation
- **Authority**: Engineering IT with department notification
- **Timeline**: Immediate pause, resolution within 4 hours

#### Level 3: Critical System Failure
- **Issue**: Validator failure, data corruption, network compromise
- **Response**: Complete system shutdown, emergency recovery
- **Authority**: Engineering Department emergency committee
- **Timeline**: Immediate shutdown, recovery within 24 hours

### Emergency Contacts
- **Primary**: Engineering IT Director (555-0101)
- **Secondary**: Engineering Department Chair (555-0102)
- **Backup**: University IT Security (555-0103)

### Emergency Procedures

#### System Pause
```bash
# Pause all market operations
solana program invoke --program-id GovernanceEngDeptPoA1234567890123456789 \
  --data [emergency-pause] --url http://localhost:8899
```

#### Emergency Shutdown
```bash
# Graceful validator shutdown
solana-validator --ledger /opt/campus-blockchain/data/engineering exit
```

#### Recovery Procedures
```bash
# Restore from backup
systemctl stop solana-validator
rm -rf /opt/campus-blockchain/data/engineering/*
tar -xzf /backup/latest/engineering-system-backup.tar.gz -C /
systemctl start solana-validator
```

---

## Maintenance & Monitoring

### Regular Maintenance Schedule

#### Daily (Automated)
- System health checks
- Performance metrics collection
- Automated backups
- Log rotation

#### Weekly (Engineering IT)
- Full system backup
- Performance analysis
- User activity review
- Security log analysis

#### Monthly (Engineering Department)
- System performance review
- User feedback collection
- Financial impact analysis
- Academic integration assessment

#### Quarterly (Engineering Committee)
- Strategic system review
- Expansion planning assessment
- Budget and resource review
- Academic research integration

### Monitoring Dashboard
- **System Status**: Validator health, network performance
- **User Activity**: Active users, transaction volume, energy trading
- **Financial Metrics**: Cost savings, energy prices, transaction fees
- **Academic Integration**: Research usage, course integration, student participation

### Key Performance Indicators
- **Uptime Target**: >99% availability
- **Performance Target**: <2 second transaction latency
- **User Satisfaction**: >80% positive feedback
- **Energy Efficiency**: >10% cost reduction vs. baseline
- **Academic Impact**: Integration in 2+ courses per semester

---

## Academic Integration

### Course Integration

#### Current Courses
1. **EE 485: Blockchain Technology**
   - Real system data for analysis
   - Student projects using trading data
   - Hands-on blockchain development

2. **SENG 490: Sustainable Energy Systems**
   - Energy trading economics
   - Renewable energy certificate analysis
   - Grid integration studies

#### Research Projects
1. **Blockchain Energy Economics** (Dr. Smith)
2. **Smart Grid Integration** (Dr. Johnson)
3. **Renewable Energy Optimization** (Dr. Lee)

#### Student Opportunities
- **Capstone Projects**: System feature development
- **Research Assistantships**: Data analysis and optimization
- **Internships**: System administration and maintenance
- **Publications**: Co-authorship on research papers

### Data Access for Research
```bash
# Export trading data for research
solana account TradingEngDeptP2P1234567890123456789 --output json > trading_data.json

# Export energy generation data
curl http://localhost:8899/oracle/energy-data/export > energy_data.csv

# Export user activity (anonymized)
curl http://localhost:8899/registry/user-activity/anonymized > user_activity.json
```

### Educational Benefits
- **Hands-on Experience**: Real blockchain system operation
- **Research Opportunities**: Live system data for analysis
- **Industry Preparation**: Blockchain and energy sector skills
- **Innovation Platform**: Test new features and algorithms

---

## Future Expansion

### Expansion Options

#### Option A: Engineering-Led Campus Expansion
- **Approach**: Expand Engineering validator to cover additional buildings
- **Control**: Engineering Department maintains sole authority
- **Timeline**: 6-12 months based on pilot success
- **Investment**: Additional smart meters and infrastructure

#### Option B: Multi-Validator Federation
- **Approach**: Add validators from other departments
- **Control**: Engineering Department remains lead authority
- **Partners**: Sustainability Office, Facilities Management
- **Timeline**: 12-18 months with inter-departmental agreements

#### Option C: Research and Development Focus
- **Approach**: Maintain Engineering scope, enhance research capabilities
- **Control**: Engineering Department exclusive control
- **Focus**: Advanced features, AI integration, optimization algorithms
- **Timeline**: Ongoing development and research initiatives

### Decision Criteria
1. **Pilot Success**: >85% user satisfaction, >99% uptime, >10% cost savings
2. **Academic Value**: Integration in multiple courses, active research projects
3. **Administrative Efficiency**: Smooth operations, minimal support burden
4. **Financial Viability**: Positive ROI, sustainable operational costs
5. **University Support**: Administrative backing for expansion

### Implementation Planning
- **Phase 1**: Complete Engineering Department evaluation
- **Phase 2**: Assess expansion demand and feasibility
- **Phase 3**: Develop expansion plan with timeline and budget
- **Phase 4**: Seek university approval and resource allocation
- **Phase 5**: Execute expansion plan with Engineering Department leadership

---

## Contact Information

### Engineering Department Team
- **System Administrator**: Engineering IT Director (eng-it@university.edu)
- **Technical Support**: Engineering IT Staff (eng-support@university.edu)
- **Academic Coordinator**: Engineering Department Chair (eng-chair@university.edu)
- **Research Coordinator**: Blockchain Research Lab (blockchain-lab@university.edu)

### Emergency Procedures
- **24/7 Emergency**: Engineering IT Hotline (555-0999)
- **University IT**: Campus IT Emergency (555-0111)
- **Facilities**: Campus Facilities Emergency (555-0222)

---

**Document Version**: 1.0
**Last Updated**: November 2025
**Next Review**: February 2026
**Authority**: Engineering Department, State University

---

*This manual provides comprehensive guidance for operating the Engineering Department P2P Energy Trading System. The system operates under Engineering Department authority with full control over all aspects of the blockchain network, ensuring efficient operation and seamless academic integration.*
