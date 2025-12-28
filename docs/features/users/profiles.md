# User Profiles and Identity

## Overview

cwrdd users are individuals with distinct identities. Each user can control what information they share publicly versus privately, supporting both open participation and privacy preservation.

## Core Principles

- **Individual identity**: All accounts represent individual people, not anonymous entities
- **Privacy control**: Users decide what to share publicly vs. privately
- **Flexibility**: Same profile for both contexts, or separate profiles
- **Authenticity**: Public profiles support accountability in public discourse
- **Contact privacy**: Different information shown to saved contacts

## Requirements

### User Account Types

**Individual Users Only:**
- Every account represents a real individual person
- No anonymous accounts
- No shared accounts
- No bot accounts (except official cwrdd bots)
- Organizations are separate entities managed by individuals

### Profile Structure

Each user has two profile contexts:

#### 1. Public Profile

**Visibility:**
- Shown to anyone viewing the user in public channels, groups, or forums
- Visible when user posts in public spaces
- Searchable (with user permission)
- Displayed on organization member lists

**Information:**
- Display name (required)
- Profile picture (optional)
- Bio/description (optional, max 500 characters)
- Location (optional, city/region level)
- Website/social links (optional)
- Organization memberships (user can choose which to display)
- Public activity indicators (posts, events created)
- Join date

**Privacy Controls:**
- User controls what fields are populated
- User can hide profile from search
- User can hide organization memberships
- Profile is always visible in context (e.g., their public posts)

#### 2. Private Profile (Contact-Only)

**Visibility:**
- Shown only to users who have saved this user as a contact
- Replaces or supplements public profile for contacts
- Not visible in public contexts

**Information:**
- All public profile fields (if shared)
- Additional contact information:
  - Email address (optional)
  - Phone number (optional)
  - Extended bio (optional)
  - More detailed location (optional)
  - Personal website/social links
  - Interests and topics
- Contact-only information is never shown publicly

**Privacy Controls:**
- User decides which additional fields to show contacts
- Can show completely different information than public profile
- Can use different display name for contacts (e.g., real name vs. pseudonym)

### Profile Configuration Options

**Option 1: Unified Profile**
- User maintains single profile
- Same information shown to everyone (public and contacts)
- Simpler to manage
- Maximum openness

**Option 2: Enhanced Contact Profile**
- Public profile shows basic information
- Contact profile adds additional information
- Balance of privacy and openness

**Option 3: Separate Profiles**
- Completely different public vs. private profiles
- Public profile may use pseudonym
- Private profile reveals real identity to contacts
- Maximum privacy control

### Display Name Requirements

**Public Display Name:**
- Required for all users
- Can be real name or pseudonym
- Must be unique (or suffixed with discriminator)
- Subject to content policies (no offensive names)
- Can be changed (with rate limiting to prevent abuse)

**Private Display Name (for contacts):**
- Optional separate name for contacts
- Often user's real name if public is pseudonym
- Shown only to contacts
- Can be same as public name

### Profile Pictures

**Public Profile Picture:**
- Optional but encouraged
- Maximum size: 5MB
- Supported formats: JPEG, PNG, WebP
- Subject to content moderation (no illegal/offensive content)
- Shown in public contexts

**Private Profile Picture (for contacts):**
- Optional separate image for contacts
- Same technical requirements
- Can be different from public profile picture
- Shown only to contacts

## Technical Requirements

### Data Model

```rust
struct User {
    id: UserId,
    username: String,           // Unique, immutable after creation
    email: String,              // Private, for account access
    phone: Option<String>,      // Private, for account recovery
    
    public_profile: PublicProfile,
    private_profile: PrivateProfile,
    
    created_at: DateTime,
    updated_at: DateTime,
}

struct PublicProfile {
    display_name: String,
    profile_picture_url: Option<String>,
    bio: Option<String>,        // Max 500 chars
    location: Option<String>,   // City/region
    website: Option<String>,
    social_links: Vec<SocialLink>,
    
    // Privacy settings
    searchable: bool,           // Default: true
    show_organizations: bool,   // Default: true
    show_join_date: bool,       // Default: true
}

struct PrivateProfile {
    // If None, uses public profile value
    display_name: Option<String>,
    profile_picture_url: Option<String>,
    
    // Contact-only fields
    email_visible: bool,        // Show email to contacts
    phone_visible: bool,        // Show phone to contacts
    extended_bio: Option<String>,
    additional_links: Vec<SocialLink>,
}
```

### Caching Strategy

**CDN Caching:**
- Profile API responses include cache headers (e.g., `Cache-Control: public, max-age=300`)
- CDN caches profile responses at edge locations
- Profile images served via CDN with long cache times
- Cache invalidation on profile updates (CDN purge API)

**Cache Headers by Content Type:**
- Public profile JSON: `max-age=300` (5 minutes)
- Profile images: `max-age=86400` (24 hours), versioned URLs for updates
- Private profile (contacts only): `private, max-age=60` (1 minute, not cached publicly)

### Privacy and Security

**Data Protection:**
- Contact-only information never leaked in public APIs
- Profile images stored securely and served via CDN
- Email and phone never exposed without explicit permission
- Profile data encrypted at rest

**Content Moderation:**
- Profile pictures scanned for illegal content (see Trust & Safety)
- Display names checked against blocked terms
- Bio content moderated for policy violations
- Users can report inappropriate profiles

## Integration with Other Features

### Organizations
- User's organization memberships shown on profile (if permitted)
- Users can represent themselves or organizations
- See: [Organizations](../organizations/organizations.md)

### Events
- Users can create personal events
- Events shown on user profile (if public)
- See: [Events](../events/event-creation.md)

## Non-Functional Requirements

### Performance
- Profile loads in <200ms
- Profile images optimized and cached via CDN
- Database indexes on username, user_id
- Efficient JSON serialization

### Scalability
- CDN handles profile request load
- Database scales with user growth
- Profile images on object storage (S3-compatible)

## Open Questions

1. Should username be changeable or immutable?
2. How do we handle impersonation (someone using a notable person's name)?
3. Should we allow completely blank public profiles (just username)?
4. What's the right rate limit for profile changes?
5. Should we show "last active" timestamp on profiles?
