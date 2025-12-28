# Age Verification and Account Eligibility

## Overview

To ensure the safety of young people and comply with legal requirements across jurisdictions, cwrdd enforces a minimum age requirement of 16 years for all users.

## Core Principle

**Users under 16 years of age are not permitted to create or maintain accounts on cwrdd.**

## Requirements

### Age Verification

- Users must declare their date of birth during account registration
- The system must calculate and verify that the user is at least 16 years of age
- Age verification must happen before account creation is completed
- Users who provide a date of birth indicating they are under 16 must be prevented from creating an account

### User Communication

- Clear messaging must be displayed to users under 16 explaining why they cannot create an account
- The messaging should be respectful and age-appropriate
- The system should not store any data about users who fail age verification

### Compliance Considerations

- Age requirements may vary by jurisdiction (some require 13+, others 16+, some 18+)
- cwrdd uses the most conservative approach (16+) to minimize compliance complexity
- As we expand to new jurisdictions, we may need to implement jurisdiction-specific age requirements
- The age verification system must support configuration of different age requirements per jurisdiction if needed in the future

#### United Kingdom Requirements

**Legal Framework:**
- **Online Safety Act 2023**: Requires age verification/assurance for services accessed by children
- **Age Appropriate Design Code (Children's Code)**: ICO guidance on protecting children online
- **GDPR/Data Protection Act 2018**: Age of digital consent is 13 in UK

**cwrdd's Approach:**
- Minimum age 16 exceeds UK requirements (which allow 13+ with protections)
- Self-declaration of age during registration
- Server-side validation required (no client-side bypass)
- Clear messaging explaining age requirement
- No collection of data from users who fail age check

**Future Requirements:**
- Online Safety Act may require enhanced age verification in future
- OFCOM may issue technical standards for age assurance
- May need third-party age verification services if requirements change
- Monitor regulatory guidance from OFCOM and ICO

#### European Union Requirements

**Legal Framework:**
- **GDPR**: Age of digital consent varies by member state (13-16 years)
  - Most common: 13-14 years
  - Some countries: 15-16 years (e.g., Austria, Germany)
- **Digital Services Act**: Age verification requirements for some content
- **Proposed CSAM Regulation**: May include age verification requirements

**cwrdd's Approach:**
- Minimum age 16 meets or exceeds all EU member state requirements
- Single age threshold (16) simplifies EU-wide compliance
- GDPR-compliant data collection and storage
- Parental consent NOT required (all users must be 16+)

**Member State Variations:**
| Country | GDPR Digital Consent Age | cwrdd Minimum (16) |
|---------|-------------------------|-------------------|
| Austria | 14 | ✓ Exceeds |
| Belgium | 13 | ✓ Exceeds |
| France | 15 | ✓ Exceeds |
| Germany | 16 | ✓ Meets |
| Ireland | 16 | ✓ Meets |
| Italy | 14 | ✓ Exceeds |
| Netherlands | 16 | ✓ Meets |
| Spain | 14 | ✓ Exceeds |

**Future Considerations:**
- Some member states may introduce stricter age verification
- DSA may evolve to require enhanced age assurance
- Monitor national implementations of EU directives

#### Other Jurisdictions

**United States:**
- **COPPA**: Requires parental consent for under 13
- cwrdd approach (16+) exceeds COPPA requirements
- Some states proposing age verification laws - monitor closely

**General Approach:**
- Default minimum age: 16 years
- Jurisdiction-specific overrides if legally required
- Configuration system supports higher age requirements (e.g., 18+ for certain features)
- Legal review required before launching in new jurisdiction

### Technical Requirements

- Date of birth must be validated at the API level
- Age calculation must account for time zones correctly
- The system should prevent users from creating accounts with birth dates in the future
- The system should reject obviously invalid birth dates (e.g., over 120 years ago)

### Privacy Considerations

- Date of birth is sensitive personal information and must be stored securely
- Date of birth should not be publicly visible
- Access to date of birth data should be restricted to authorized personnel only
- Users should be able to update their date of birth if they made a mistake, but changes should be logged and monitored for abuse

## Non-Functional Requirements

### Performance

- Age verification should not add noticeable latency to the registration process
- Calculation should be done efficiently without complex date libraries if possible

### Security

- Date of birth data must be stored in compliance with data protection regulations
- Age verification logic must be server-side only (never trust client-side calculations)
- Failed age verification attempts should not reveal system internals

### Auditability

- All age verification failures should be logged (without storing personal data of rejected users)
- Logs should track patterns that might indicate automated account creation attempts

## Future Considerations

- Integration with third-party age verification services may be required in some jurisdictions
- Some jurisdictions may require parental consent for users between certain ages (e.g., 13-16)
- We may need to implement re-verification processes if age requirements change in a jurisdiction
- Consider how to handle users who become underage due to changes in jurisdiction requirements

## Open Questions

1. Should we allow users to register if they will turn 16 within a certain timeframe (e.g., 30 days)?
2. How should we handle existing accounts if a jurisdiction's age requirement increases?
3. What documentation should we require if users claim their birthdate was entered incorrectly and they are actually old enough?
