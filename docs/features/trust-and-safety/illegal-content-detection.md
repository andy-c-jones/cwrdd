# Illegal Content Detection and Response

## Overview

cwrdd implements automated systems to detect, remove, and respond to illegal content, particularly child sexual abuse material (CSAM) and other harmful illegal content. These systems balance user privacy with legal obligations and moral responsibility to prevent harm.

## Core Principles

- **Zero tolerance for illegal content** - Illegal content will be removed and offenders punished
- **Proactive detection** - Use technology to detect illegal content before users report it
- **Privacy preservation** - Respect encryption while still protecting users from harm
- **Legal compliance** - Meet reporting obligations in all jurisdictions
- **User protection** - Empower users to report harmful content they receive

## Requirements

### Automated Detection Systems

#### PhotoDNA and Hash Matching

- Integrate Microsoft PhotoDNA or similar hash-matching technology
- Maintain database of known CSAM image hashes from NCMEC and other sources
- Scan all images uploaded to public channels
- Scan profile pictures and public images
- Match against known illegal content databases
- Automated removal of confirmed matches
- Automated reporting to appropriate authorities

#### Machine Learning Detection

- Train ML models to detect potential CSAM and other illegal content
- Models analyze visual content, not encrypted text
- Focus on public uploads where content can be analyzed
- Flag suspicious content for human review
- Continuously update models with new patterns
- Use industry-standard models and databases (NCMEC, ICMEC, etc.)

#### Metadata Analysis

For encrypted private messages, analyze metadata:
- File size and type patterns
- Sharing patterns and frequency
- Number of reports per account
- Behavioral patterns associated with illegal content sharing

### Public Content Scanning

All content in public channels is subject to scanning:
- Images and videos scanned with PhotoDNA and ML models
- Known illegal content automatically removed
- Suspicious content flagged for human review
- Text content analyzed for indicators of illegal activity
- URLs checked against known illegal content domains

### Private Message Limitations

**Encrypted private messages cannot be scanned for content**, but:
- Files attached to messages can have hashes computed client-side and checked against databases
- Users can report messages they receive
- Accounts with multiple reports are flagged and restricted
- Behavioral patterns can indicate illegal activity

### User Reporting System

#### Reporting Mechanism

Users must be able to:
- Report any content as illegal or harmful
- Report from public channels or private messages
- Select reason for report (CSAM, harassment, terrorism, etc.)
- Provide optional additional context
- Submit reports anonymously (reporter identity protected)

#### Report Handling for Public Content

When illegal content is reported in public:
1. Content immediately hidden pending review
2. Trust & safety team notified
3. Review completed within 24 hours (critical cases within 1 hour)
4. If confirmed illegal: permanent removal + account action + legal report
5. If false report: content restored + reporter notified

#### Report Handling for Private Encrypted Messages

When encrypted private message is reported:
1. **Message removed from reporter's inbox immediately**
2. Reporter shown confirmation and support resources
3. Report logged against sender's account
4. Sender's account flagged if multiple reports received
5. Cannot access message content, but record kept of:
   - Sender account ID
   - Recipient account ID
   - Timestamp
   - Report reason
   - Any metadata available

This creates a **blind pattern detection system**:
- We don't know what was sent
- But we know Account X has multiple reports of harmful content
- Patterns emerge showing likely bad actors
- Accounts with pattern of reports are restricted/banned

### Account Actions and Enforcement

#### For Confirmed Illegal Content

When illegal content is confirmed:

**First offense (depending on severity):**
- Content removed
- Account temporarily suspended (7-30 days)
- Formal warning issued
- Law enforcement notification (for serious offenses like CSAM)

**Second offense or severe first offense:**
- Content removed
- Account permanently restricted from uploading images/videos
- Cannot create public channels
- Law enforcement notification
- Added to internal watchlist

**Third offense or extremely severe content:**
- Account permanently banned
- All content removed
- Law enforcement notification
- IP/device ban where appropriate
- Added to shared industry databases where legal

#### Graduated Restrictions

For accounts flagged but not confirmed:
1. **Multiple reports** (3+) - Investigation triggered, temporary upload restrictions
2. **Pattern of reports** (5+) - Cannot upload media, restricted posting
3. **Continued reports** (10+) - Account suspension pending investigation
4. **Confirmed after investigation** - Permanent ban

### Blind Pattern Detection for Encrypted Messages

Since we cannot decrypt private messages:

**Account reputation scoring based on:**
- Number of illegal content reports received
- Number of accounts that have blocked the user
- Frequency of reports relative to message volume
- Time pattern of reports (clusters indicate problematic behavior)
- Cross-referencing with public content violations

**Actions based on reputation score:**
- Low reputation: Warnings and education
- Moderate reputation: Restrictions on new contact outreach
- Low reputation: Cannot message non-contacts
- Very low reputation: Account suspension pending appeal
- Repeated very low: Permanent ban

**Transparency to account holder:**
- Users informed their account has received multiple reports
- Not told specific content or reporters (privacy protection)
- Given opportunity to appeal or explain
- Clear path to redemption if behavior improves

### Legal Reporting Obligations

#### Mandatory Reporting

When CSAM or other illegal content is detected, cwrdd must report to appropriate authorities based on jurisdiction.

#### United Kingdom Reporting

**Primary Authority:**
- **National Crime Agency (NCA) - Child Exploitation and Online Protection Command (CEOP)**
- Reports submitted via: https://www.ceop.police.uk/ceop-reporting/
- For industry reporting: Use NCMEC CyberTipline (UK law enforcement has access to NCMEC reports)
- Alternative: Internet Watch Foundation (IWF) for CSAM reporting

**Reporting Requirements:**
- Report CSAM within 24 hours of detection
- Preserve evidence for law enforcement investigation
- Cooperate with NCA and local police forces
- Comply with Online Safety Act 2023 requirements
- Designate UK-based contact for law enforcement liaison

**Other Illegal Content:**
- Terrorist content: Report to Counter Terrorism Internet Referral Unit (CTIRU)
- Online fraud: Report to Action Fraud (UK's national reporting center)
- Other serious crimes: Report to local police or via Crime Stoppers

#### European Union Reporting

**Primary Authorities by Member State:**

**Ireland** (if cwrdd establishes EU operations here):
- An Garda Síochána - Online Child Exploitation Unit
- Hotline.ie for CSAM reporting
- Reports also submitted to NCMEC (international coordination)

**General EU Requirements:**
- Each member state has national hotlines (coordinated by INHOPE network)
- Report to local authorities in member state where company is established
- Comply with Digital Services Act (DSA) requirements:
  - Designate points of contact in EU
  - Report illegal content to relevant national authorities
  - Cooperate with Digital Services Coordinators
- Use NCMEC CyberTipline for international coordination (EU authorities have access)

**Key EU Hotlines:**
- **INHOPE Network**: International association of internet hotlines (covers all EU countries)
- Reports can be submitted via national INHOPE members
- Cross-border cooperation for international cases

**DSA Obligations:**
- Very Large Online Platforms (VLOPs): Enhanced reporting and risk assessment requirements
- Designate Digital Services Coordinator point of contact
- Annual transparency reports on content moderation
- Notice and action mechanisms for illegal content

#### United States Reporting

**Primary Authority:**
- **National Center for Missing & Exploited Children (NCMEC) CyberTipline**
- Reports submitted via: https://www.cybertipline.org or API integration
- Required under 18 U.S.C. § 2258A for Electronic Service Providers

**Reporting Requirements:**
- Report CSAM within 24 hours of detection
- Preserve evidence for 90+ days (up to 180 days if requested by law enforcement)
- Include all required elements per 18 U.S.C. § 2258A

#### International Coordination

For cross-border cases:
- **Interpol**: International cooperation for serious crimes
- **Europol**: EU-wide coordination for serious organized crime
- **NCMEC**: Serves as international hub, shares reports with international partners
- **Virtual Global Taskforce (VGT)**: International law enforcement partnership

#### Report Contents

Reports to authorities include:
- Account information (username, registration details, email, phone number)
- IP addresses and access logs
- Content hashes and metadata
- Timestamps and access patterns
- Geographic location data (if available)
- Device information
- Any other information required by law or reasonably available

Reports do NOT include:
- Content of encrypted private messages (we don't have access)
- Information about other users unless directly relevant to the report
- Unrelated account activity

#### Reporting Timeline and Process

1. **Immediate** (0-1 hour):
   - Content flagged by automated systems
   - Content removed from public visibility
   - Evidence preserved in secure storage

2. **Urgent Review** (1-4 hours):
   - Human review confirms illegal content
   - Severity assessment
   - Jurisdiction determination

3. **Reporting** (within 24 hours):
   - Report filed with appropriate authority/authorities
   - Account action taken
   - Documentation completed

4. **Follow-up**:
   - Preserve evidence for required retention period
   - Cooperate with law enforcement investigation
   - Provide additional information if requested
   - Track report outcome for transparency reporting

#### Jurisdiction-Specific Requirements

- Maintain knowledge of reporting requirements per jurisdiction
- Implement jurisdiction-specific reporting workflows
- Track and document all reports filed
- Regular training for trust & safety team on legal obligations
- Maintain relationships with law enforcement liaisons in key markets
- Monitor changes to reporting requirements and laws

## Technical Requirements

### System Architecture

- Content scanning service independent from messaging core
- Real-time scanning for uploaded content
- Queue-based review system for flagged content
- Secure storage for evidence (time-limited, encrypted, access-controlled)
- Integration with reporting APIs (NCMEC, etc.)

### PhotoDNA Integration

- API integration with Microsoft PhotoDNA Cloud Service
- On-device hash computation for privacy-preserving scanning
- Regular updates to hash databases
- Fast hash comparison algorithms
- Secure storage of hash databases

### Machine Learning Models

- Use industry-standard models (e.g., those provided by tech coalitions)
- Regular model updates and retraining
- Explainable AI for review process
- A/B testing of model improvements
- Model performance monitoring

### Evidence Preservation

When illegal content is detected:
- Content preserved in secure, encrypted storage
- Access restricted to authorized trust & safety personnel
- Retention period per legal requirements (typically 90+ days)
- Audit logs of all access
- Secure deletion after retention period

### Privacy Protections

- Scanning only where legally and technically possible (public content, hashes)
- Minimal data collection for private content
- Encryption of all evidence and reports
- Access controls and audit logging
- Regular privacy impact assessments

## Human Review Process

### Trust & Safety Team

- Trained reviewers for flagged content
- Psychological support for reviewers (exposure to traumatic content)
- Clear escalation paths for ambiguous cases
- Documentation of decision-making process
- Regular training updates

### Review Workflow

1. Automated system flags content
2. Content enters review queue with priority level
3. Trained reviewer assesses content
4. Decision: remove, allow, escalate
5. If removed: account action + legal reporting if required
6. Documentation of decision and rationale
7. User notification (where appropriate)

### Review Standards

- Clear guidelines for what constitutes illegal content
- Jurisdiction-specific considerations
- Context evaluation (artistic, educational, documentary exceptions)
- Consistent decision-making across reviewers
- Regular calibration sessions

## Metrics and Monitoring

Track and monitor:
- Volume of content scanned daily
- Number of detections by system (PhotoDNA, ML, reports)
- False positive rate
- Time to review flagged content
- Number of accounts actioned
- Reports filed with authorities
- Appeal outcomes
- User reports of missed content

## Transparency

Publish in transparency reports:
- Number of illegal content detections
- Breakdown by type (CSAM, terrorism, other)
- Number of accounts restricted/banned
- Number of reports to authorities
- Average review times
- Appeal outcomes

Do NOT publish:
- Specific detection techniques (avoid evasion)
- Details that could identify victims
- Information that could compromise investigations

## User Support

For users who report illegal content:
- Immediate confirmation of report receipt
- Resources for support (counseling, hotlines)
- Follow-up on action taken (within privacy constraints)
- Clear explanation of process
- Protection of reporter identity

For users affected by illegal content:
- Remove content immediately from their view
- Support resources and counseling information
- Option to speak with trust & safety team
- Account deletion assistance if desired
- Clear communication about their safety

## Non-Functional Requirements

### Performance

- Content scanning must not significantly delay uploads
- Hash matching should complete in <100ms
- ML inference should complete in <500ms
- Review queue should have SLA (critical: 1 hour, high: 24 hours, medium: 72 hours)

### Accuracy

- PhotoDNA false positive rate near zero
- ML model false positive rate <5%
- Human review accuracy >99%
- Regular audits of detection accuracy

### Security

- Evidence storage encrypted at rest and in transit
- Access to detection systems restricted
- Audit logs of all system access
- Regular security assessments
- Compliance with data protection regulations

## Industry Collaboration

- Participate in Tech Coalition and similar initiatives
- Share hash databases (with appropriate legal agreements)
- Contribute to industry-wide detection improvements
- Collaborate on best practices
- Support research on detection technologies (privacy-preserving)

## Future Considerations

- Advanced AI for more sophisticated detection
- Cross-platform coordination (industry-wide)
- Better support for victims and survivors
- Improved reviewer tools and support
- Integration with emerging detection technologies
- Privacy-preserving scanning techniques (homomorphic encryption, etc.)

## Open Questions

1. How do we balance proactive scanning with privacy in gray areas?
2. What is the right threshold for account restrictions based on blind reports?
3. Should we implement client-side scanning for private content (controversial)?
4. How do we handle content that is illegal in some jurisdictions but not others?
5. What appeal process is appropriate for illegal content bans?
6. How do we support law enforcement while protecting user privacy?
