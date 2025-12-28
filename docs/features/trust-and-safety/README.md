# Trust and Safety

## Overview

The Trust and Safety functional area encompasses all systems, policies, and processes designed to keep cwrdd users safe while respecting privacy and enabling democratic participation. This includes age verification, jurisdictional compliance, content moderation, spam prevention, and illegal content detection.

## Core Principles

cwrdd's approach to trust and safety balances competing priorities:

- **User Safety First** - Protecting users from harm is paramount
- **Privacy Preservation** - Safety measures must respect end-to-end encryption and user privacy
- **Legal Compliance** - Meet legal obligations in all jurisdictions where we operate
- **Transparency** - Be open about how safety systems work and their limitations
- **User Empowerment** - Give users control over their safety through blocking, reporting, and privacy settings
- **Proportionate Response** - Apply graduated enforcement that matches the severity of violations

## Key Challenges

The trust and safety work at cwrdd is uniquely challenging because:

1. **End-to-end encryption prevents content scanning** - Unlike traditional platforms, we cannot scan private messages for harmful content, requiring innovative approaches like blind pattern detection
2. **Jurisdictional variability** - Laws vary significantly across regions, requiring flexible and configurable systems
3. **Privacy vs. safety tradeoffs** - Every safety measure must be evaluated against privacy implications
4. **Sophisticated bad actors** - Spammers and malicious actors continually evolve their tactics

## Feature Requirements

### [Age Verification](age-verification.md)

Ensures users are at least 16 years old before creating accounts, complying with legal requirements and protecting young people.

**Key Requirements:**
- Minimum age of 16 for all users
- Server-side age verification
- Secure storage of birth date information
- Clear messaging for users who don't meet age requirements

### [Jurisdictional Compliance](jurisdictional-compliance.md)

Defines cwrdd's policy on encryption and jurisdictional launch requirements, including the commitment to never provide backdoors to encrypted communications.

**Key Requirements:**
- End-to-end encryption for all private communications
- Will not launch in jurisdictions requiring encryption backdoors
- Comprehensive legal review before launching in new regions
- Clear withdrawal process if jurisdiction laws become incompatible

### [Spam Detection and Prevention](spam-detection-and-prevention.md)

Multi-layered systems to detect and prevent spam while respecting privacy in encrypted channels.

**Key Requirements:**
- Machine learning and rule-based spam detection
- User reporting and blocking capabilities
- Automated restrictions for flagged spam accounts
- Graduated enforcement with appeals process
- Metadata analysis for encrypted messages (not content)

### [Illegal Content Detection](illegal-content-detection.md)

Systems to detect, remove, and report illegal content, particularly CSAM, while respecting encryption constraints.

**Key Requirements:**
- PhotoDNA and hash-matching for known illegal content
- Machine learning for suspicious content detection
- User reporting system with immediate removal from reporter's view
- Blind pattern detection for encrypted content (based on reports)
- Mandatory reporting to authorities (NCMEC, etc.)
- Graduated account restrictions and bans

## Technical Architecture Considerations

### Privacy-Preserving Approaches

Trust and safety systems must work within the constraint that **private encrypted messages cannot be scanned**:

- **Metadata analysis** - Analyze message patterns, frequency, recipients without accessing content
- **Blind pattern detection** - Track reports against accounts to identify bad actors without knowing content
- **Hash-matching on device** - Client-side hash computation for images before encryption
- **Public content scanning** - Scan public channels where encryption is not used
- **User reports** - Rely on recipients to report harmful content they receive

### Modular Design

Trust and safety systems should be:
- **Independent services** - Separate from core messaging infrastructure
- **Event-driven** - React to user actions, reports, and detections
- **Auditable** - All actions logged for review and compliance
- **Configurable** - Support jurisdiction-specific rules and thresholds
- **Scalable** - Handle growing user base and content volume

### Data Considerations

- **Separation of concerns** - Trust and safety data separate from message data
- **Access controls** - Strict limits on who can access what data
- **Retention policies** - Store only what's needed, delete when no longer required
- **Encryption** - Evidence and reports stored securely
- **Audit logs** - Track all access and actions

## Human Review Requirements

Automated systems cannot handle all scenarios - human review is essential:

- **Trust & Safety Team** - Trained reviewers for flagged content and appeals
- **Escalation paths** - Clear processes for ambiguous or complex cases
- **Reviewer support** - Psychological support for exposure to traumatic content
- **Quality assurance** - Regular calibration and accuracy checks
- **Documentation** - Clear guidelines and decision-making frameworks

## Metrics and Transparency

### Key Metrics

- Spam detection accuracy (false positive/negative rates)
- Content moderation response times
- Appeal resolution times and outcomes
- Reports filed with authorities
- Account restrictions and bans
- User satisfaction with safety measures

### Transparency Reporting

Publish regular reports including:
- Volume of spam and illegal content detections
- Government requests and responses
- Content removal statistics by jurisdiction
- Appeal outcomes
- System accuracy and improvements

## Compliance Framework

### Regular Reviews

- Legal compliance audits per jurisdiction
- Privacy impact assessments for new safety features
- Security assessments of safety systems
- Review of detection accuracy and bias
- Evaluation of jurisdictional law changes

### Documentation

- Decision rationale for all policy choices
- Training materials for trust & safety team
- User-facing safety documentation
- Transparency reports
- Incident response procedures

## Future Considerations

As cwrdd grows and technology evolves:

- **Advanced AI** - More sophisticated detection while preserving privacy
- **Industry collaboration** - Share intelligence with other platforms (privacy-preserving)
- **Client-side scanning** - Controversial but may be required in some jurisdictions
- **Decentralized moderation** - Community-driven safety with platform oversight
- **Privacy-enhancing technologies** - Homomorphic encryption, secure multi-party computation

## Related Documentation

- [Core Principles](../../README.md#core-principles) - Overall project philosophy
- [Privacy & Security](../privacy-and-security/) - Privacy and security features (to be created)
- [User Management](../user-management/) - Account management features (to be created)

## Open Questions

Cross-cutting questions across all trust and safety features:

1. How do we balance automated enforcement with human judgment?
2. What is the right threshold for restricting accounts based on blind reports?
3. How do we ensure fairness across cultures, languages, and regions?
4. What role should users play in moderation (beyond reporting)?
5. How do we scale human review as the platform grows?
6. Should we implement client-side scanning for certain content types?
7. How do we handle coordinated inauthentic behavior and state actors?
