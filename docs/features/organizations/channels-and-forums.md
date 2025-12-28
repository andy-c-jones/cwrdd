# Channels and Forums

## Overview

Organizations can create structured communication spaces for their communities. Channels provide real-time chat, while forums enable threaded, asynchronous discussions. Both can be public or private, with different pricing tiers.

## Core Principles

- **Structured communication**: Different formats for different needs (real-time vs. threaded)
- **Access control**: Public vs. private spaces with clear permissions
- **Scalability**: Support small groups to large communities
- **Monetization**: Advanced features require subscriptions to sustain platform
- **Moderation**: Clear tools for maintaining healthy discussions

## Requirements

### Chat Channels

Real-time messaging spaces for organization discussions.

#### Public Channels

**Characteristics:**
- Real-time chat interface
- Open to specified audience (anyone, members, etc.)
- Searchable message history
- Can be listed in organization profile
- Visible in public organization directory

**Access Levels:**
- **Open to all**: Anyone can view and participate
- **Members only**: Must be organization member to participate
- **Read-only**: Anyone can view, only members/writers can post

**Features:**
- Real-time messaging
- Message reactions
- File/image sharing
- Message pinning (moderators/admins)
- Thread replies
- Member list
- Search history

**Pricing:**
- **Free**: Unlimited public channels
- Message history retention limits on free tier
- Paid tiers can extend history retention

#### Private Channels

**Characteristics:**
- End-to-end encrypted group chat
- Invitation-only access
- Not listed publicly
- Only visible to channel members

**Access Control:**
- Admins/moderators can create private channels
- Invite specific users (don't need to be org members)
- Remove users from channel
- Delete channel

**Features:**
- End-to-end encryption for messages
- File/image sharing (encrypted)
- Video/audio calls (future)
- Message reactions
- Smaller member capacity than public channels

**Pricing:**
- **Free tier**: Limited number of private channels per organization
- **Paid tier**: Increased or unlimited private channels
- **Free tier**: Limited members per private channel
- **Paid tier**: Increased member capacity per private channel

### Forums

Threaded discussion boards for organized, asynchronous conversations.

#### Public Forums

**Characteristics:**
- Threaded discussions organized by boards/topics
- Long-form content support
- Searchable and indexable
- Open to specified audience
- Visible in organization profile

**Structure:**
- **Forum**: Top-level container (one per organization)
- **Boards**: Categories/topics (e.g., "General Discussion", "Announcements", "Help")
- **Threads**: Individual discussion topics
- **Posts**: Messages within threads

**Access Levels:**
- **Open to all**: Anyone can view and post
- **Members only**: Must be organization member to post
- **Read-only**: Anyone can view, only members/writers can post

**Features:**
- Rich text formatting (Markdown)
- Thread voting/popularity
- Thread pinning and locking
- Post editing history
- File/image attachments
- User mentions and notifications
- Thread subscriptions
- Search and filtering

**Pricing:**
- **Paid feature**: Public forums require organization subscription
- Different tiers with varying board limits
- Higher tiers include advanced moderation and analytics

**Rationale for Paid Model:**
- Forums require significant infrastructure (storage, search indexing, moderation tools)
- High-quality forum software is typically paid
- Revenue supports platform sustainability
- Free channels and private forums still available

#### Private Forums

**Characteristics:**
- Threaded discussions for organization members only
- Not publicly searchable
- More intimate discussions
- Same structure as public forums (boards, threads, posts)

**Access Control:**
- Only organization members can access
- Admins/moderators can restrict specific boards to certain roles
- Fine-grained permissions per board

**Features:**
- Same as public forums
- More granular permission controls
- Optional encryption for sensitive discussions

**Pricing:**
- **Free tier**: Limited boards and threads
- **Paid tier**: Increased or unlimited capacity
- More affordable than public forums (less infrastructure burden)

## Channel and Forum Management

### Creation

**Who Can Create:**
- **Public Channels**: Admins and Moderators
- **Private Channels**: Admins and Moderators
- **Public Forums**: Admins only (requires subscription)
- **Private Forums**: Admins only
- **Boards**: Admins and Moderators

### Permissions

**Board-Level Permissions:**
- Read access (everyone, members, writers, moderators, admins)
- Write access (everyone, members, writers, moderators, admins)
- Moderate access (moderators, admins)

**Channel Permissions:**
- Defined by channel visibility and organization role
- Moderators can manage messages and members
- Writers can post with organizational attribution

## Moderation Tools

### Channel Moderation

**Message Actions:**
- Delete message
- Pin message
- Mute user temporarily
- Report message for review

**Channel Actions:**
- Slow mode (rate limiting)
- Lock channel (admins only)
- Archive channel
- Export channel history

### Forum Moderation

**Thread Actions:**
- Pin thread
- Lock thread
- Move thread to different board
- Merge threads
- Delete thread

**Post Actions:**
- Edit post (with indicator)
- Delete post
- Split into new thread
- Report for review

**User Actions:**
- Warn user
- Mute user from forum/board
- Ban user from forum
- View user history in forum

## User Experience

### Channel Interface

**Channel List:**
- Sidebar showing all channels user has access to
- Unread message indicators
- Pin favorite channels to top
- Separate sections: Public, Private, Direct Messages

**Channel View:**
- Real-time message stream
- Message composition at bottom
- Member list sidebar (collapsible)
- Channel info and settings
- Search within channel
- Jump to date

### Forum Interface

**Forum Home:**
- List of boards with descriptions
- Thread count and recent activity per board
- Breadcrumb navigation

**Board View:**
- List of threads with preview
- Thread sorting (recent, popular, unanswered)
- Sticky/pinned threads at top
- Thread creation button
- Search and filters

**Thread View:**
- Original post at top
- Chronological replies
- Rich text rendering
- Quote and reply buttons
- Thread tools (lock, pin, subscribe)
- Breadcrumb navigation

## Integration with Other Features

### Organizations
- Channels and forums belong to organizations
- Organizational roles determine permissions
- See: [Organizations](organizations.md)

### Events
- Events can be promoted in channels and forums
- Event discussion threads in forums
- See: [Events](../events/event-creation.md)

### Pricing
- Forum creation tied to subscription tier
- Channel capacity limits on free tier
- See: [Pricing](../pricing/subscription-model.md)

## Non-Functional Requirements

### Performance
- Real-time message delivery with minimal latency
- Forum threads load quickly
- Efficient pagination for large forums
- CDN caching for forum content

### Scalability
- Support thousands of concurrent users in channels
- Millions of forum posts
- Efficient full-text search
- WebSocket connection pooling for real-time features

### Accessibility
- Keyboard navigation throughout
- Screen reader support
- WCAG 2.1 AA compliance
- High contrast mode support

## Open Questions

1. Should we support nested boards in forums (sub-boards)?
2. What's the right balance between free and paid forum features?
3. Should channel message history be searchable for non-members?
4. How do we handle very large channels (10,000+ members)?
5. Should we support channel/forum federation with other platforms?
6. What moderation tools are most critical for launch vs. can be added later?
