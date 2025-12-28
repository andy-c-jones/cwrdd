# Authentication and Authorization

## Overview

cwrdd provides secure authentication and authorization mechanisms that balance accessibility with security. Anonymous users can access public content with rate limits, while authenticated users gain full platform access.

## Core Principles

- **Secure by default**: Strong encryption, secure sessions, protection against common attacks
- **Progressive access**: Anonymous users can browse, authenticated users can participate
- **Rate limiting**: Prevent abuse while allowing legitimate use
- **Future-proof**: Architecture supports multiple authentication methods
- **Privacy-preserving**: Minimal data collection, secure storage

## Requirements

### User Types and Access

#### Anonymous Users

**Access Permitted:**
- Read public channels (view messages, history)
- View public forums (threads, posts)
- View public events
- View organization profiles
- View public user profiles
- RSVP to public events (with email, optional name/phone)

**Access Denied:**
- Post messages or create content
- Create events
- Join organizations
- Direct messaging
- Access private content
- Account management features

**Rate Limiting:**
- Aggressive rate limits to prevent scraping and abuse
- Lower throughput for API requests
- Stricter limits on search and discovery features
- Per-IP tracking
- See: [Rate Limiting](rate-limiting.md)

#### Authenticated Users

**Access Granted:**
- All anonymous user permissions
- Create and edit content
- Join organizations and participate
- Create events
- Direct messaging (end-to-end encrypted)
- Profile management
- Access private content (based on permissions)

**Rate Limiting:**
- Higher rate limits than anonymous users
- Account-based tracking
- Trust-based adjustments
- See: [Rate Limiting](rate-limiting.md)

#### Trusted Users (Future Consideration)

**Potential Paid Tier:**
- Even higher rate limits for legitimate high-volume use cases
- API access for integrations
- Bulk operations
- Must maintain trust score (spam reports reduce limits)
- Similar to email reputation systems

**Trust Building:**
- Account age
- Activity history
- Spam report ratio
- Successful interactions
- Community contributions

### Account Creation and Registration

#### Current Implementation

**Username:**
- Must be a valid email address
- Serves as login identifier
- Unique across platform
- Verified via email confirmation

**Password:**
- Minimum complexity requirements
- Hashed using modern, secure algorithm (Argon2id recommended)
- Never stored in plaintext
- Password reset via email verification

**Required Information:**
- Email address (username)
- Password
- Date of birth (age verification - see Trust & Safety)
- Display name
- Terms of Service acceptance

**Optional Information:**
- Phone number (account recovery)
- Profile details

**Email Verification:**
- Verification email sent on registration
- Account limited until email verified
- Resend verification option
- Verification link expiration

#### Future Enhancements

**Passkey Support:**
- WebAuthn/FIDO2 implementation
- Passwordless authentication
- Biometric support (fingerprint, face recognition)
- Security key support (YubiKey, etc.)
- More secure than passwords
- Better user experience

**Identity Provider Integration:**
- OAuth/OIDC support for trusted providers
- Potential providers: GitHub, Google, Apple, etc.
- Account linking (connect multiple auth methods)
- Maintain email as primary identifier

### Authentication Flow

#### Sign Up

1. User provides email, password, date of birth, display name
2. Server validates input:
   - Email format valid and not already registered
   - Password meets complexity requirements
   - Age verification passes (16+)
   - Display name acceptable
3. Server hashes password securely (Argon2id)
4. Account created with "unverified" status
5. Verification email sent
6. User can log in but with limited access until verified
7. User clicks verification link in email
8. Account marked as verified
9. Full access granted

#### Sign In

1. User provides email and password
2. Server looks up user by email
3. Server verifies password hash matches
4. Server checks account status (active, suspended, etc.)
5. Server generates JWT access token and refresh token
6. Tokens returned to client
7. Client stores tokens securely
8. Access token included in subsequent requests

#### Sign Out

1. User clicks sign out
2. Client sends sign out request with tokens
3. Server invalidates refresh token (blacklist or remove from Redis)
4. Client deletes tokens from storage
5. User redirected to public view

#### Session Management

**JWT Access Tokens:**
- Short-lived: 15 minutes
- Contains user ID, roles, permissions
- Signed with server secret (HS256 or RS256)
- Stateless (no server-side session storage for access tokens)
- Refreshed automatically before expiration

**Refresh Tokens:**
- Longer-lived: 7 days (or configurable)
- Stored securely on client (httpOnly cookie preferred)
- Used to obtain new access token
- Stored server-side in Redis with user association
- Can be revoked (sign out, security event)
- Rotated on use (issue new refresh token with each refresh)

**Session Activity:**
- Session extends with each interaction
- 15-minute sliding window from last activity
- Inactivity timeout logs user out
- User explicitly signs out ends session immediately

**Security Measures:**
- Tokens signed and verified
- HTTPS required (tokens never sent over HTTP)
- HttpOnly cookies for refresh tokens (prevents XSS access)
- Secure flag on cookies
- SameSite attribute for CSRF protection
- Token expiration enforced
- Refresh token rotation prevents replay attacks

### Password Security

#### Storage

**Hashing Algorithm:**
- Argon2id (recommended - winner of Password Hashing Competition)
- Or bcrypt/scrypt if Argon2 not available
- Salted hashes (unique salt per password)
- Configurable work factor (to adjust with hardware improvements)

**Never:**
- Store passwords in plaintext
- Log passwords
- Transmit passwords except during registration/login over HTTPS
- Share passwords with third parties

#### Password Requirements

**Complexity:**
- Minimum 12 characters (recommended)
- Must contain mix of character types (or high entropy)
- No common passwords (check against common password list)
- No personal information (username, display name)

**User Experience:**
- Show password strength indicator
- Allow paste (don't block password managers)
- Show/hide password toggle
- Clear error messages on failure

#### Password Reset

1. User requests password reset
2. Verification code sent to email
3. Code valid for limited time (e.g., 15 minutes)
4. User enters code and new password
5. Password hash updated
6. All existing sessions invalidated (clear refresh tokens from Redis)
7. User must sign in with new password

**Security:**
- Rate limit password reset requests
- Don't reveal if email exists (always show "check your email")
- Invalidate code after use
- Require email verification

### Authorization

#### Permission Model

**Resource-Based Permissions:**
- Users have permissions based on context
- Organization membership grants permissions
- Role within organization determines capabilities
- Events have owner permissions

**Permission Checks:**
- Every request validates permissions
- Middleware enforces authentication requirements
- Resource-specific authorization checks
- Fail securely (deny by default)

**Permission Contexts:**
- Public resources: No authentication required (read-only)
- User resources: Authenticated and owns resource
- Organization resources: Member with appropriate role
- Private resources: Explicitly granted access

## Anonymous Event RSVP

**Use Case:**
- Allow event discovery and attendance without account requirement
- Organizers need contact information for attendees
- Balance accessibility with spam prevention

**RSVP Process:**

1. Anonymous user views public event
2. Clicks RSVP button
3. Prompted for:
   - Email address (required)
   - Name (optional but encouraged)
   - Phone number (optional)
4. Server validates input:
   - Email format valid
   - Rate limit not exceeded (per email and per IP)
   - Event not at capacity
5. RSVP recorded with email/name/phone
6. Confirmation email sent
7. Attendee added to private attendee list (organizer only)

**Privacy:**
- Email/name/phone not publicly visible
- Only event organizer can see attendee contact info
- Attendees not shown publicly (unless they have accounts and RSVP'd)
- Data used only for event communication

**Rate Limiting:**
- Per email: Limit RSVPs per time period
- Per IP: Limit total RSVPs per time period
- Prevent spam RSVP attacks
- See: [Rate Limiting](rate-limiting.md)

**Attendee Management:**
- Organizer can export attendee list
- Send mass emails to attendees
- Remove spam RSVPs
- Mark attendance at event

## Security Considerations

### Attack Prevention

**Brute Force Protection:**
- Rate limit login attempts
- Account lockout after failed attempts
- CAPTCHA after suspicious activity
- Progressive delays on repeated failures

**Credential Stuffing:**
- Monitor for unusual login patterns
- Alert users of suspicious activity
- Support 2FA (future enhancement)
- Password breach detection (future)

**Session Hijacking:**
- Short-lived access tokens
- Secure token storage
- HTTPS only
- Bind session to IP/user agent (optional, with considerations)

**CSRF Protection:**
- SameSite cookies
- CSRF tokens for sensitive operations
- Verify origin headers

**XSS Protection:**
- HttpOnly cookies (tokens not accessible to JavaScript)
- Content Security Policy
- Input sanitization
- Output encoding

### Data Protection

**In Transit:**
- TLS 1.3 required
- Strong cipher suites
- HSTS enabled
- Certificate pinning (for mobile apps)

**At Rest:**
- Password hashes (Argon2id)
- Email addresses stored securely
- Personal information encrypted
- Database encryption

**Logging:**
- Never log passwords or tokens
- Log authentication events (success, failure, logout)
- Log rate limit violations
- Audit trail for security events

## Redis Integration

### Session Storage

**Refresh Tokens in Redis:**
- Key: `session:refresh:{token_id}`
- Value: User ID, creation time, last used time
- TTL: Refresh token lifetime (7 days)
- Automatic expiration

**Session Invalidation:**
- Sign out: Delete refresh token from Redis
- Password change: Delete all user's refresh tokens
- Account suspension: Delete all user's refresh tokens

**Benefits:**
- Centralized session management
- Fast lookups
- Automatic expiration
- Easy revocation

## User Experience

### Sign In/Sign Up UI

**Streamlined Flow:**
- Combined sign in/sign up page
- Clear distinction between actions
- "Sign in with email" or "Create account"
- Social sign-in options (future)
- Passkey options (future)

**Helpful Features:**
- Remember me (extends refresh token)
- Password reset link prominent
- Email verification resend
- Clear error messages
- Password strength indicator

### Session Management

**User Controls:**
- View active sessions
- Sign out of all sessions
- Session history (future)
- Suspicious activity alerts

**Automatic Behavior:**
- Session extends with activity
- Seamless token refresh
- Graceful session expiration (redirect to sign in)
- Preserve intended destination after login

## Integration with Other Features

### Trust & Safety
- Age verification during registration
- Spam report tracking affects rate limits
- Account restrictions enforced
- See: [Trust and Safety](../trust-and-safety/)

### User Profiles
- Authentication required for profile editing
- Public profiles visible to anonymous users
- Private profiles require authentication
- See: [User Profiles](../users/profiles.md)

### Organizations
- Organization membership requires authentication
- Role-based permissions enforced
- See: [Organizations](../organizations/organizations.md)

### Rate Limiting
- Different limits for anonymous vs authenticated users
- Trust score affects limits
- See: [Rate Limiting](rate-limiting.md)

## Non-Functional Requirements

### Performance
- Authentication check <10ms
- Token generation/validation <10ms
- Session refresh seamless (no user-visible delay)
- Redis lookup <5ms

### Scalability
- Stateless authentication (JWT access tokens)
- Distributed session management (Redis cluster)
- Horizontal scaling supported
- Session storage scales with Redis

### Reliability
- Authentication failures fail securely (deny access)
- Graceful degradation if Redis unavailable
- Session recovery on server restart
- Password hash verification resilient to timing attacks

## Open Questions

1. Should we implement device tracking for suspicious login detection?
2. What's the right balance for refresh token lifetime?
3. Should we support multiple simultaneous sessions per user?
4. How do we handle account recovery if user loses access to email?
5. What 2FA methods should we prioritize (TOTP, SMS, passkey)?
6. Should trusted user tier be invitation-only or open to applications?
7. How do we prevent anonymous RSVP spam at scale?
8. Should we implement "remember this device" functionality?
