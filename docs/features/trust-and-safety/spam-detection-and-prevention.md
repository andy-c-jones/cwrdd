# Spam Detection and Prevention

## Overview

cwrdd implements multi-layered spam detection and prevention systems to protect users from unwanted messages, automated abuse, and malicious actors while respecting user privacy in private encrypted communications.

## Core Principles

- **Proactive detection** - Use machine learning and filtering to detect spam before it impacts users
- **User empowerment** - Give users tools to report and block spam
- **Graduated response** - Apply proportionate restrictions based on spam detection confidence
- **Privacy preservation** - Balance spam detection with privacy, especially in encrypted channels

## Requirements

### Detection Systems

#### Machine Learning Detection

- Train ML models to identify spam patterns from:
  - Reported spam content (from public channels and user reports)
  - Account behavior patterns (message frequency, recipient patterns, account age)
  - Network analysis (accounts working in coordination)
  - Content similarity detection (repeated messages to many recipients)
- Models must be continuously updated with new spam patterns
- False positive rate must be monitored and minimized
- Models must be auditable and explainable where possible

#### Rule-Based Filtering

- Implement filters for known spam patterns:
  - Message frequency limits per time period
  - Repeated identical or near-identical messages
  - Suspicious URLs or link patterns
  - Known spam keywords or phrases (in non-encrypted contexts)
  - Account creation velocity from same network
- Rules must be configurable and testable
- Rules must be documented with rationale

#### Behavioral Analysis

Monitor for spam indicators:
- High volume of messages to non-contacts
- High volume of contact requests
- Messages sent immediately after account creation
- Rapid account-switching patterns
- Coordinated behavior across multiple accounts

### User Reporting

#### Report Spam Functionality

Users must be able to:
- Report any message or account as spam
- Report from public channels, private messages, or contact requests
- Optionally provide additional context about the spam
- Block the account simultaneously with reporting

#### Report Processing

- All spam reports are logged and analyzed
- Reports contribute to account reputation scoring
- Multiple reports from different users increase confidence
- Reports trigger review by automated systems
- High-confidence spam triggers immediate action
- Lower-confidence reports queue for human review

### User Blocking

Users must be able to:
- Block any account from contacting them
- View list of blocked accounts
- Unblock accounts if desired
- Block prevents: messages, contact requests, seeing user in public channels
- Blocking is immediate and persistent
- Blocked account is not notified of the block (privacy consideration)

### Automated Restrictions

#### Account Flagging

When spam is detected with high confidence:
- Account is flagged in the spam detection system
- Flag triggers automated restrictions
- Flag is reviewed by trust & safety team
- Flag may be escalated to temporary or permanent ban

#### Restrictions for Flagged Accounts

Accounts flagged as potential spam are restricted:
- **Cannot send messages to non-contacts** - Can only message people who have saved them as a contact
- **Cannot send contact requests** - Must be added by others
- **Rate limits applied** - Reduced message sending limits
- **Public channel restrictions** - May be prevented from posting in public channels
- **Contact request restrictions** - May be prevented from joining new groups

These restrictions:
- Take effect immediately upon flagging
- Remain in place until reviewed by trust & safety team
- Can be appealed by the account holder
- Are lifted if appeal is successful or false positive is determined

#### Graduated Enforcement

1. **First flag** - Temporary restrictions (7 days), warning message
2. **Second flag** - Extended restrictions (30 days), stern warning
3. **Third flag** - Permanent restrictions with review process
4. **Severe violations** - Immediate account suspension pending review

### Appeals Process

Users must be able to:
- Appeal spam flags and restrictions
- Provide context and evidence they are not spammers
- Request human review of automated decisions
- Have appeals reviewed within reasonable timeframe (e.g., 48-72 hours)
- Receive clear explanation of decision

Appeals process must:
- Be clearly documented and accessible
- Be handled by trained trust & safety team
- Consider all evidence fairly
- Document decision rationale
- Allow for escalation if initial appeal is denied

## Technical Requirements

### System Architecture

- Spam detection service independent from messaging core
- Real-time processing for high-priority checks
- Batch processing for analysis and ML training
- Event-driven architecture for spam reports and flags
- Audit logging of all spam-related actions

### Data Collection

For spam detection, collect:
- Message metadata (sender, recipient, timestamp, size) - NOT content of encrypted messages
- Public channel content (can be analyzed)
- User behavior metrics
- Report details and frequency
- Network/IP information
- Device fingerprints (with user consent)

### Privacy Protections

- Encrypted message content is never scanned or analyzed
- Metadata analysis only, respecting privacy principles
- Spam detection data stored separately from message data
- Access to spam detection data restricted to authorized personnel
- Regular audits of data access and usage

### Performance Requirements

- Spam checks must not add noticeable latency to message delivery
- ML inference must complete in <100ms for real-time checks
- Batch analysis should complete within reasonable timeframes
- System must scale to handle high message volumes

## Public vs. Private Content

### Public Channels

Public content can be scanned for spam:
- Content filtering and analysis permitted
- Automated removal of spam posts
- Account restrictions for public channel spam
- Transparency in moderation actions

### Private Encrypted Messages

Privacy must be preserved:
- **Cannot scan encrypted message content**
- **Can analyze metadata** (frequency, patterns, recipients)
- **Rely on user reports** for content-based spam detection
- **Behavioral analysis** only
- Restrictions based on pattern detection, not content inspection

## Metrics and Monitoring

Track and monitor:
- Spam reports per day/week/month
- Flagged accounts and confidence levels
- False positive rate (appeals successfully upheld)
- True positive rate (confirmed spam)
- User blocking frequency
- Restricted account recidivism rates
- Time to review appeals
- User satisfaction with spam protection

## Transparency

- Publish spam statistics in transparency reports
- Document spam detection methodologies (without revealing evasion techniques)
- Clearly communicate to users how spam detection works
- Explain what data is used for spam detection
- Be transparent about encrypted vs. non-encrypted content handling

### UK and EU Transparency Requirements

**United Kingdom (Online Safety Act 2023):**
- Annual transparency reports required by OFCOM
- Include: volume of complaints, action taken, response times
- Publish user-facing terms of service and community guidelines
- Make complaints and appeals process accessible
- Report spam/fraud statistics if requested by OFCOM

**European Union (Digital Services Act):**
- Biannual transparency reports required
- Include: notices received, action taken, automated moderation use
- Average time to handle notices
- Information about use of automated tools and accuracy
- Available in language(s) of member states where service offered
- Aggregated data on complaints and outcomes
- Submit to Digital Services Coordinator and European Commission

**Content of Transparency Reports:**
- Total spam reports received (by type: messages, accounts, public posts)
- Automated spam detections (high confidence vs flagged for review)
- Account restrictions applied (by severity: temporary, permanent)
- Appeals received and outcomes (upheld, overturned, pending)
- False positive rate (where measurable)
- Response time metrics (time to review, time to act)
- Geographic breakdown by jurisdiction (where applicable)

### Coordination with Authorities

**United Kingdom:**
- **OFCOM**: Regulatory oversight, risk assessments, compliance
- **Trading Standards**: Consumer protection, fraud prevention
- **Action Fraud**: Reporting fraud and scam content
- Cooperate with investigations into spam/fraud operations

**European Union:**
- **Digital Services Coordinator**: Primary regulatory contact per member state
- **Consumer protection authorities**: Fraud and scam coordination
- **Europol**: Serious organized crime including large-scale fraud
- Participate in coordinated enforcement actions

**Industry Collaboration:**
- **Messaging Anti-Abuse Working Group (MAAWG)**: Share spam intelligence
- **Global Anti-Scam Alliance**: Coordinate on fraud prevention
- Share spam patterns with other platforms (privacy-preserving)

## Non-Functional Requirements

### Accuracy

- Target false positive rate <1%
- Target true positive rate >95%
- Continuous improvement through feedback loops

### Fairness

- Avoid bias in spam detection (language, culture, region)
- Ensure legitimate bulk messaging use cases are not flagged
- Consider context (business accounts, community notifications, etc.)

### Scalability

- System must scale to millions of users
- ML models must handle increasing data volumes
- Review processes must scale with user base (consider AI assistance)

## Future Considerations

- Integration with third-party spam databases
- Community-driven spam reporting (trusted reporters)
- Reputation systems for accounts and domains
- Advanced AI for adaptive spam detection
- Cross-platform spam intelligence sharing (privacy-preserving)

## Open Questions

1. How do we handle false positives for legitimate high-volume senders (businesses, organizations)?
2. Should we implement a "verified account" system to bypass some spam checks?
3. What appeals process is appropriate for different levels of restrictions?
4. How do we prevent abuse of the spam reporting system itself?
5. Should temporary phone number services be blocked from registration?
