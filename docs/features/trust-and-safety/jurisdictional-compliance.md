# Jurisdictional Compliance and Encryption Policy

## Overview

cwrdd is committed to protecting user privacy through end-to-end encryption. This commitment means we will only launch in jurisdictions that respect users' rights to private, encrypted communications.

## Core Principle

**cwrdd will never provide a mechanism to decrypt private user data. We will not launch in jurisdictions that have laws requiring backdoors or access to encrypted communications.**

## Policy

### Encryption Commitment

- All private communications use end-to-end encryption
- cwrdd does not hold keys to decrypt private messages
- No employee, administrator, or system process can access the content of private encrypted messages
- This is a non-negotiable technical and ethical stance

### Jurisdictional Requirements

Before launching in any jurisdiction, we must verify:

1. **No mandatory decryption laws** - The jurisdiction does not require service providers to decrypt user data or provide backdoor access to encrypted communications
2. **No key escrow requirements** - The jurisdiction does not require encryption keys to be held in escrow or made available to authorities
3. **No mandatory data localization that conflicts with encryption** - If data must be stored locally, it must be possible to do so while maintaining end-to-end encryption
4. **Legal protection for encrypted services** - The jurisdiction provides legal protection for services that cannot access user content

### Jurisdiction Launch Checklist

Prior to launching in a new jurisdiction:

- [ ] Legal review of encryption-related laws
- [ ] Legal review of surveillance and lawful access laws
- [ ] Legal review of data protection and privacy laws
- [ ] Legal review of age-related requirements
- [ ] Legal review of content moderation requirements
- [ ] Assessment of political and regulatory stability
- [ ] Documented decision with rationale
- [ ] Risk assessment and mitigation plan

### Compliance Obligations

In each jurisdiction where we operate, we must:

- Comply with age verification requirements (see age-verification.md)
- Comply with transparency reporting requirements
- Comply with content removal orders for public content
- Comply with lawful account suspension or termination orders
- Comply with data protection and privacy regulations
- Maintain appropriate business licenses and registrations

### What We Cannot Comply With

We cannot and will not:

- Decrypt private end-to-end encrypted messages
- Provide backdoor access to encrypted communications
- Install surveillance capabilities that compromise encryption
- Weaken encryption algorithms or key lengths at government request
- Provide real-time access to encrypted communications
- Store private encryption keys where they can be accessed by cwrdd or third parties

## Technical Requirements

### Encryption Implementation

- Private messages must use industry-standard end-to-end encryption
- Encryption implementation must be auditable and transparent
- Keys must be generated and stored only on user devices
- Key exchange protocols must be secure and verified

### Jurisdiction Detection

- System must detect user jurisdiction (by IP address, phone number, or explicit selection)
- Registration must be blocked if user is in an unsupported jurisdiction
- Clear messaging must explain why service is unavailable in that jurisdiction
- Users attempting to circumvent restrictions (e.g., via VPN) should be detected where possible and informed of Terms of Service implications

### Public Content Compliance

While private content cannot be decrypted, public content is subject to local laws:

- Public posts and channels must comply with local content regulations
- Content moderation systems must support jurisdiction-specific rules
- Removal orders for public content must be processed according to local law
- Transparency reports should detail content removal requests by jurisdiction

## Withdrawal from Jurisdictions

If a jurisdiction implements laws incompatible with our encryption commitment:

1. **Assessment Period** - Legal team assesses the new law and its implications
2. **Decision Timeline** - Executive decision within 30 days of law taking effect
3. **User Notification** - Minimum 60 days notice to users in affected jurisdiction
4. **Data Export** - Provide tools for users to export their public data
5. **Service Termination** - Block new registrations immediately, cease service by deadline
6. **Public Statement** - Issue public statement explaining withdrawal decision

## Transparency

cwrdd commits to:

- Publishing regular transparency reports detailing government requests
- Documenting jurisdictions where service is not available and why
- Challenging unjust or overly broad government requests
- Supporting legal and policy efforts to protect encryption
- Contributing to policy discussions about encryption and privacy

## Regional Considerations

### United Kingdom (Primary Market)

**Legal Framework:**
- **Online Safety Act 2023**: Comprehensive framework for online safety
  - Duty of care to users
  - Risk assessments for illegal content and child safety
  - Content moderation requirements
  - Transparency reporting obligations
  - OFCOM as regulator with enforcement powers
- **Investigatory Powers Act 2016 (IPA)**: Grants broad surveillance powers
  - Technical Capability Notices (TCNs) could require assistance
  - Current interpretation: Cannot require breaking encryption if technically impossible
  - Monitor for changes that would mandate backdoors
- **Data Protection Act 2018**: UK implementation of GDPR
- **Protection of Children Act 1978 & Criminal Justice Act 1988**: CSAM laws

**cwrdd's Assessment:**
- **Currently compatible**: Online Safety Act does not explicitly require breaking encryption
- **OFCOM has stated**: Cannot require technically impossible measures
- **Risk area**: Government pressure for "lawful access" to encrypted content
- **Our commitment**: Will comply with all requirements that don't break encryption
- **Red line**: If law changes to require decryption, we evaluate withdrawal

**Operational Requirements:**
- Designate UK-based contact for OFCOM
- Register with OFCOM as regulated service (when thresholds met)
- Conduct annual risk assessments (CSAM, terrorism, fraud)
- Publish transparency reports
- Implement age verification (already doing 16+)
- Report illegal content to NCA/CEOP within 24 hours
- Respond to takedown requests for illegal content in public channels
- Maintain UK-accessible contact methods for users and law enforcement

**Reporting Authorities:**
- CSAM: NCA Child Exploitation and Online Protection Command (CEOP)
- Terrorism: Counter Terrorism Internet Referral Unit (CTIRU)
- Fraud: Action Fraud
- General crime: Local police forces or Crime Stoppers

### European Union (Primary Market)

**Legal Framework:**
- **General Data Protection Regulation (GDPR)**: Strong privacy protections
  - Requires data protection by design and default
  - Right to privacy enshrined in law
  - Fines up to 4% of global turnover for violations
- **ePrivacy Directive**: Privacy in electronic communications
  - Confidentiality of communications required
  - End-to-end encryption protected
- **Digital Services Act (DSA)**: Content moderation and platform responsibilities
  - Risk assessments for Very Large Online Platforms (VLOPs)
  - Transparency reporting
  - Notice and action mechanisms for illegal content
  - User appeals process
  - Designate Digital Services Coordinator point of contact
- **Proposed Regulation on Child Sexual Abuse Material (CSAM Regulation)**: Under debate
  - Could require detection orders for CSAM
  - Controversial provisions around client-side scanning
  - Final form uncertain - monitor closely

**cwrdd's Assessment:**
- **Highly compatible**: EU generally supportive of encryption
- **GDPR alignment**: Our privacy-by-design approach aligns perfectly
- **DSA compliance**: Achievable without breaking encryption
- **Risk area**: Final form of CSAM Regulation unknown
- **Our commitment**: Comply with all requirements unless they mandate breaking encryption

**Operational Requirements:**
- Establish legal entity in EU member state (likely Ireland or Netherlands)
- Designate EU representative for GDPR compliance
- Designate Digital Services Coordinator point of contact
- Appoint Data Protection Officer (DPO)
- Conduct Data Protection Impact Assessments (DPIAs)
- Publish transparency reports (DSA requirements)
- Implement notice and action mechanisms
- Provide user appeals process
- Report illegal content to member state authorities
- Cooperate with Digital Services Coordinators

**If Established in Ireland:**
- Data Protection Commission (DPC) as supervisory authority
- Digital Services Coordinator: Commission for Communications Regulation (ComReg)
- Report CSAM to An Garda Síochána Online Child Exploitation Unit
- Report to Hotline.ie (INHOPE member)
- English-language jurisdiction simplifies operations

**INHOPE Network:**
- Pan-European network of internet hotlines
- Submit reports to national INHOPE member in country of operation
- Cross-border coordination for international CSAM cases
- Coordination with NCMEC for global reach

### United States (Secondary Market)

- No federal law requiring backdoors (currently)
- Section 230 protections for content moderation
- COPPA compliance required for users under 13 (we exceed this with 16+ requirement)
- State-level laws may vary - assess per state if necessary
- **NCMEC CyberTipline**: Mandatory CSAM reporting under 18 U.S.C. § 2258A
- First Amendment protections for encrypted speech

### Other European Countries (Post-Brexit)

**Switzerland:**
- Strong privacy protections
- Not EU member but similar privacy culture
- Revised Federal Act on Data Protection (FADP) similar to GDPR
- No encryption backdoor requirements
- Good infrastructure for hosting/operations

**Norway/Iceland (EEA):**
- GDPR applies via EEA agreement
- Strong privacy protections
- No encryption restrictions

### China

- Encryption backdoor requirements likely incompatible
- Data localization requirements may conflict with security model
- Extensive content controls incompatible with private encrypted messaging
- Cybersecurity Law requires data storage in China with government access
- **Not a target market for initial launch**
- **Assessment**: Incompatible with our encryption commitment

### Jurisdictions to Monitor

Maintain active monitoring of:
- **Brazil**: Data protection laws (LGPD) and encryption debates
- **India**: Encryption traceability proposals and intermediary guidelines
- **Australia**: Assistance and Access Act implications for encryption
- **Russia**: Encryption and VPN regulations (likely incompatible)
- **Canada**: Similar to US/UK but monitor PIPEDA and proposed online safety legislation
- **Any jurisdiction with pending encryption legislation**

### Regional Launch Priority

Based on compatibility and market opportunity:

**Phase 1 (Initial Launch):**
1. United Kingdom - Home market, compatible framework
2. Ireland - EU base, English-language, compatible
3. Other EU countries - GDPR alignment

**Phase 2 (Expansion):**
4. United States - Large market, compatible
5. Canada - Similar to US/UK
6. Switzerland/Norway/Iceland - Compatible privacy frameworks
7. Australia - Monitor Assistance and Access Act carefully

**Not Planned:**
- China - Incompatible encryption requirements
- Russia - Incompatible encryption requirements
- Any jurisdiction with mandatory decryption laws

## Non-Functional Requirements

### Legal

- Maintain relationships with legal counsel in each jurisdiction
- Regular legal compliance audits
- Document all compliance decisions
- Maintain insurance for legal challenges

### Operational

- Ability to restrict service by jurisdiction at short notice
- User communication templates for service withdrawal
- Data export capabilities for affected users
- Business continuity plans for jurisdiction withdrawal

## Open Questions

1. How do we handle users who travel between jurisdictions?
2. What is our policy on users using VPNs to access service from restricted jurisdictions?
3. Should we geofence the application or rely on Terms of Service?
4. How do we balance user access with compliance risk?
5. At what point does a proposed law trigger withdrawal versus wait-and-see?
