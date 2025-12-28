# Event Creation and Management

## Overview

Events are the core of cwrdd's mission to facilitate real-world human interaction. Both individuals and organizations can create, promote, and manage in-person gatherings and meetings.

## Core Principles

- **Facilitate in-person meetings**: Events are about real-world gathering, not virtual
- **Democratic participation**: Events enable organizing and democratic engagement
- **Individual and collective**: Both individuals and organizations can host events
- **Clear attribution**: Transparency about who is organizing events
- **Accessibility**: Event information should be easy to discover and share

## Requirements

### Who Can Create Events

#### Individual Users

**Any user can create personal events:**
- Events associated with their individual account
- Full control over event details
- Can promote in channels and forums where they have access
- Event shown on their public profile (if they choose)

#### Organizations

**Events on behalf of organizations have different attribution:**

**Admins:**
- Create events **as the organization**
- No personal attribution visible
- Event appears to be directly from the organization
- "Organization Name presents..."

**Moderators and Writers:**
- Create events **on behalf of the organization**
- Personal attribution included
- Event shows individual as organizer
- "John Smith on behalf of Organization Name presents..."
- Clear indication they represent the organization

**Why the distinction:**
- Admins speak with full organizational authority
- Writers/Moderators represent but don't embody the organization
- Transparency about decision-making authority
- Accountability while enabling distributed event creation

### Event Types

#### Public Events

**Characteristics:**
- Visible to anyone
- Listed in public event directories
- Searchable
- Anyone can RSVP (with or without account)
- Can be shared outside the platform

**Use Cases:**
- Community gatherings
- Public meetings
- Political rallies
- Protests and demonstrations
- Open houses
- Public lectures

#### Member-Only Events

**Characteristics:**
- Only organization members can see and RSVP
- Not listed in public directories
- Requires organization membership to access

**Use Cases:**
- Member meetings
- Internal planning sessions
- Member social events
- Committee meetings

#### Private Events

**Characteristics:**
- Invitation-only
- Not searchable or listed
- Only invited users can see event details
- Manual invitation by organizer

**Use Cases:**
- Small gatherings
- Sensitive meetings
- Exclusive events
- Leadership meetings

### Event Information

**Required Fields:**
- Event title
- Date and time (with timezone)
- Location (physical address or description)
- Event type (public, member-only, private)
- Organizer (individual or organization)

**Optional Fields:**
- Description (rich text)
- Cover image
- Additional organizers/co-hosts
- RSVP limit (capacity)
- RSVP deadline
- External registration link
- Tags/categories
- Attached documents (agenda, directions, etc.)
- Contact information
- Accessibility information (wheelchair access, etc.)
- Cost/fee information (if applicable)

### Event Visibility and Discovery

**Public Events:**
- Appear in public event calendar
- Searchable by location, date, category
- Shown on organizer's profile (individual or organization)
- Can be embedded on external websites
- Shareable links

**Event Promotion:**
- Post in organization channels and forums
- Share to personal contacts
- Cross-post to multiple organizations (with permission)
- Featured events (algorithm or admin curated)
- Trending events based on RSVPs

### RSVP and Attendance

**RSVP Options:**
- Going
- Interested/Maybe
- Not Going
- RSVP visibility (public or private)

**Attendance Tracking:**
- Organizers can see RSVP list
- Optional check-in feature at event
- Post-event attendance confirmation

**Capacity Management:**
- Optional RSVP limits
- Waitlist functionality
- Automatic notifications when spots open

**Guest Management:**
- Allow guests (bring a friend)
- Guest approval by organizer
- Guest limits per attendee

### Event Updates and Communication

**Organizer Tools:**
- Send updates to all RSVPs
- Message all attendees
- Edit event details (with notification of changes)
- Cancel event (with automatic notifications)
- Mark event as happened/completed

**Attendee Tools:**
- Receive event reminders
- Notification of changes
- Ask questions (comments on event)
- Share with contacts
- Add to personal calendar (iCal export)

### Recurring Events

**Support for:**
- Weekly, monthly, yearly recurrence
- Custom recurrence patterns
- Single RSVP for all occurrences or per-occurrence
- Edit single occurrence or all future occurrences

**Use Cases:**
- Weekly community meetings
- Monthly socials
- Annual conferences

## Event Promotion in Channels and Forums

### Channel Promotion

**Posting Events:**
- Admins/Moderators/Writers can post organization events
- Individual users can post their own events (where permitted)
- Rich event card display in channel
- Direct RSVP from channel message
- Event updates appear in channel

### Forum Promotion

**Event Threads:**
- Create discussion thread for event
- Event details prominently displayed
- Discussion about event logistics
- Q&A with organizers
- Post-event discussion

**Event Boards:**
- Dedicated board for upcoming events
- Sticky thread for major events
- Event archive board for past events

## User Experience

### Event Creation Flow

1. Click "Create Event" (from profile or organization page)
2. Choose attribution (personal, as organization, on behalf of organization)
3. Fill out event details
4. Choose visibility (public, member-only, private)
5. Add co-organizers if desired
6. Preview event page
7. Publish event
8. Promote event (share, post to channels/forums)

### Event Page

**Header:**
- Event title and cover image
- Date, time, location
- Organizer information with attribution
- RSVP buttons
- Share buttons

**Body:**
- Full description
- Map and directions
- RSVP list (if public)
- Comments/discussion
- Related events
- Organizer contact information

**Sidebar:**
- Event details summary
- Co-organizers
- Tags/categories
- Export to calendar
- Report event

### Event Discovery

**Event Calendar:**
- Calendar view of public events
- Filter by location, date range, category
- Map view showing nearby events
- List view with search and filters

**Organization Events:**
- Tab on organization profile showing all events
- Upcoming and past events
- Subscribe to organization's events

**Personal Events:**
- "My Events" page showing:
  - Events user is attending
  - Events user created
  - Suggested events based on interests

## Integration with Other Features

### Organizations
- Organizations create and promote events
- Organizational roles determine event creation permissions
- Events shown on organization profile
- See: [Organizations](../organizations/organizations.md)

### User Profiles
- Personal events shown on user profile
- Event attendance history (if public)
- See: [User Profiles](../users/profiles.md)

### Channels and Forums
- Events promoted in organization communication spaces
- Discussion threads for events
- See: [Channels and Forums](../organizations/channels-and-forums.md)

## Non-Functional Requirements

### Performance
- Event page loads quickly
- Calendar views render efficiently
- Search results return promptly
- Real-time RSVP updates

### Scalability
- Support millions of events
- Thousands of attendees per event
- Efficient geographic search
- Calendar view optimization

### Accessibility
- Screen reader friendly
- Keyboard navigation
- High contrast support
- Clear date/time formatting

## Open Questions

1. Should events support virtual/hybrid options (online + in-person)?
2. How do we handle event spam (fake events)?
3. Should we allow ticketing/payment integration for paid events?
4. What moderation tools are needed for event comments/discussions?
5. Should users be able to "host" events at their private locations?
6. How do we handle event series (multiple related events)?
7. Should events have their own discussion forums/channels?
