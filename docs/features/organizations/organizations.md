# Organizations

## Overview

Organizations in cwrdd represent groups, communities, businesses, or institutions. They provide structured spaces for collective communication and event management while maintaining clear accountability through individual ownership and defined roles.

## Core Principles

- **Individual accountability**: Organizations are managed by individuals with defined roles
- **Structured hierarchy**: Clear permission levels for different responsibilities
- **Flexible visibility**: Organizations can have public and private spaces
- **Event empowerment**: Organizations facilitate community gatherings and meetings
- **Transparency**: Organization structure and membership can be public

## Requirements

### Organization Creation

**Who Can Create:**
- Any individual user in good standing
- Minimum account age requirements (e.g., 14 days, prevents spam accounts)
- Subject to rate limits (e.g., 5 organizations per user)

**Creation Process:**
1. User initiates organization creation
2. Provide required information:
   - Organization name (unique)
   - Description
   - Category/type (community, business, nonprofit, government, etc.)
   - Public visibility setting
3. Optionally provide:
   - Logo/avatar
   - Website
   - Location
   - Social links
4. Creator becomes first Admin

### Organization Roles

Organizations have a hierarchical permission structure:

#### Organization Admin

**Permissions:**
- Full organization management
- Add/remove admins, moderators, writers, members
- Edit organization details (name, description, logo)
- Create and manage channels and forums
- Create events **as the organization** (no personal attribution)
- Delete organization
- Manage billing and subscriptions (for paid features)
- View analytics and insights

**Attribution:**
- Posts and events show as from the organization itself
- "Organization Name" (not "John Smith from Organization")
- Admin identity not revealed in public posts
- Admin action logs maintained for accountability (internal only)

#### Moderator

**Permissions:**
- Manage content in channels and forums
- Remove inappropriate posts/comments
- Ban/mute members from channels/forums
- Create channels and forums (with restrictions)
- Create events on behalf of organization (with personal attribution)
- View member list
- Cannot modify organization details or roles

**Attribution:**
- Posts show "John Smith on behalf of Organization Name"
- Events created show moderator as creator
- Clear indication they represent the organization

#### Writer

**Permissions:**
- Post in organization channels and forums (beyond member privileges)
- Create events on behalf of organization (with personal attribution)
- Pin/highlight posts (in allowed channels)
- Create announcements
- No moderation powers
- No management powers

**Attribution:**
- Posts show "John Smith on behalf of Organization Name"
- Events show writer as creator
- Clear indication of organizational affiliation

#### Member

**Permissions:**
- View organization profile
- Displayed as organization member (if member list is public)
- Participate in public channels/forums (according to channel rules)
- Receive organization updates (if opted in)
- Leave organization at any time

**Attribution:**
- Normal user posts (their individual identity)
- Can mention their membership in posts
- Shown on organization member list

**Display Options:**
- Members can choose whether membership appears on their public profile
- Members can opt in/out of public member list

### Organization Profile

**Public Information:**
- Organization name
- Logo/avatar
- Description
- Category/type
- Location (if applicable)
- Website and social links
- Member count
- Public channels and forums
- Upcoming public events
- Creation date

**Private Information (Admins only):**
- Full member list with roles
- Private channels and forums
- Analytics and metrics
- Billing information
- Moderation logs

### Organization Discovery

**Public Organizations:**
- Searchable by name, category, location
- Featured/recommended organizations
- Trending organizations
- Organization directory by category

**Private Organizations:**
- Not searchable
- Only visible to members
- Invitation-only membership

## Organization Channels and Forums

See: [Channels and Forums](channels-and-forums.md)

Organizations can create structured communication spaces:

### Chat Channels

**Public Channels:**
- Real-time chat
- Open to anyone or organization members
- Searchable conversation history
- Free for basic usage

**Private Channels:**
- Organization members only
- Invitation-based access
- Encrypted conversations
- Free for limited usage, paid for advanced features

### Forums

**Public Forums:**
- Threaded discussions
- Open to anyone or organization members
- Organized by boards/topics
- **Paid feature** - requires subscription

**Private Forums:**
- Organization members only
- Threaded discussions
- **Free tier**: Limited boards and threads
- **Paid tier**: Unlimited boards and threads

## Organization Events

See: [Event Creation](../events/event-creation.md)

Organizations facilitate in-person gatherings:

**Event Creation:**
- Admins: Create events as the organization (no personal attribution)
- Moderators/Writers: Create events on behalf of organization (with attribution)
- Events promoted in channels, forums, and organization profile

**Event Types:**
- Public events (open to everyone)
- Member-only events
- Private events (invitation-only)

## Technical Requirements

### Data Model

```rust
struct Organization {
    id: OrganizationId,
    name: String,               // Unique
    slug: String,               // URL-friendly, unique
    description: String,
    category: OrganizationType,
    logo_url: Option<String>,
    
    visibility: Visibility,     // Public or Private
    
    created_by: UserId,
    created_at: DateTime,
    updated_at: DateTime,
    
    // Subscription
    subscription_tier: SubscriptionTier,
    subscription_status: SubscriptionStatus,
}

enum OrganizationType {
    Community,
    Business,
    Nonprofit,
    Government,
    Educational,
    Political,
    Other,
}

enum Visibility {
    Public,     // Searchable, anyone can view
    Private,    // Not searchable, members only
}

struct OrganizationMember {
    organization_id: OrganizationId,
    user_id: UserId,
    role: OrganizationRole,
    joined_at: DateTime,
    
    // Display preferences
    show_on_profile: bool,      // Show membership on user profile
    show_in_member_list: bool,  // Show in org's public member list
}

enum OrganizationRole {
    Admin,
    Moderator,
    Writer,
    Member,
}
```

### Permission Checks

```rust
fn can_create_channel(org: &Organization, user: &User, role: OrganizationRole) -> bool {
    match role {
        OrganizationRole::Admin => true,
        OrganizationRole::Moderator => true,
        OrganizationRole::Writer => false,
        OrganizationRole::Member => false,
    }
}

fn can_post_as_organization(role: OrganizationRole) -> bool {
    matches!(role, OrganizationRole::Admin)
}

fn can_post_on_behalf(role: OrganizationRole) -> bool {
    matches!(role, OrganizationRole::Admin | OrganizationRole::Moderator | OrganizationRole::Writer)
}
```

### Attribution Display

When rendering organization posts/events:

```rust
struct PostAttribution {
    organization: Organization,
    author: Option<User>,       // None for Admin posts (as organization)
    attribution_type: AttributionType,
}

enum AttributionType {
    AsOrganization,             // Admin posting as org
    OnBehalf { user: User },    // Moderator/Writer on behalf
}

fn render_attribution(attr: &PostAttribution) -> String {
    match &attr.attribution_type {
        AttributionType::AsOrganization => 
            format!("{}", attr.organization.name),
        AttributionType::OnBehalf { user } => 
            format!("{} on behalf of {}", user.display_name, attr.organization.name),
    }
}
```

## User Experience

### Organization Page

**Public View:**
- Organization header (logo, name, description)
- Member count and notable members
- Tabs:
  - About: Full description, location, links
  - Channels: List of public channels
  - Forums: List of public forums (if subscribed)
  - Events: Upcoming and past events
  - Members: Public member list (if enabled)

**Member View:**
- All public content plus:
  - Private channels (if member)
  - Private forums (if member)
  - Member management (if admin/moderator)
  - Settings (if admin)

### Organization Management

**Admin Dashboard:**
- Organization settings
- Member management (add, remove, change roles)
- Channel and forum management
- Event management
- Analytics and insights
- Billing and subscription
- Moderation tools

## Integration with Other Features

### User Profiles
- Organizations shown on user profiles (if permitted)
- Organizational role badges
- See: [User Profiles](../users/profiles.md)

### Events
- Organizations create and promote events
- Event attribution (as organization or on behalf)
- See: [Events](../events/event-creation.md)

### Pricing
- Public forum creation requires subscription
- Private forum capacity limited on free tier
- See: [Pricing](../pricing/subscription-model.md)

## Non-Functional Requirements

### Performance
- Organization profile loads in <200ms
- CDN caching for public organization data
- Efficient member role lookups

### Scalability
- Support millions of organizations
- Efficient member list pagination
- Database indexes on organization slug, name

## Open Questions

1. Should we allow organization transfers (change owner/creator)?
2. How many admins should an organization have (unlimited or limited)?
3. Should there be a verification process for certain organization types (government, business)?
4. What analytics should organizations have access to?
5. Should organizations be able to "endorse" or "partner with" each other?
