# Rate Limiting Implementation

## Overview

Rate limiting in cwrdd uses a token bucket algorithm implemented with Redis and integrated with warp middleware. This provides distributed, consistent rate limiting across all application instances.

## Core Principles

- **Fair resource allocation**: Prevent any single user from monopolizing resources
- **Abuse prevention**: Protect against scraping, spam, and DoS attacks
- **Progressive access**: Different limits for anonymous vs authenticated vs trusted users
- **Distributed enforcement**: Consistent limits across all server instances
- **Observable**: Clear feedback to users about limits and remaining capacity

## Token Bucket Algorithm

### Concept

Each user (or IP) has a virtual "bucket" of tokens:
- Bucket has maximum capacity (burst size)
- Tokens regenerate at fixed rate
- Each request consumes one or more tokens
- Request allowed if sufficient tokens available
- Request denied if insufficient tokens

**Advantages:**
- Allows bursts of activity within capacity
- Smooth rate limiting over time
- Simple to understand and implement
- Well-suited for API rate limiting

### Implementation with Redis and Warp

**Redis Data Structure:**

Each bucket stored as Redis hash with fields:
- `tokens`: Current token count (float for precision)
- `last_refill`: Timestamp of last refill (Unix epoch)

**Rate Limit Check Algorithm:**

1. Calculate time elapsed since last refill
2. Calculate tokens to add: `elapsed_seconds * refill_rate`
3. Add tokens to bucket, cap at maximum capacity
4. Check if request cost <= available tokens
5. If yes: Deduct tokens and allow request
6. If no: Deny request with 429 response
7. Update bucket state in Redis

**Atomic Operations:**

Use Redis Lua scripts to ensure atomicity:
- Read bucket state
- Calculate new token count
- Deduct tokens
- Update bucket state

All in single atomic operation to prevent race conditions.

## Rate Limit Tiers

### Anonymous Users

**Identifier:** Source IP address

**Characteristics:**
- Aggressive limits to prevent scraping and abuse
- Encourage account creation
- Allow browsing but not heavy usage

**Typical Limits (indicative):**
- Read public content: Lower capacity, slower refill
- Search queries: Very limited
- Event RSVP: Limited to prevent spam
- Heavy queries: Significantly restricted

**Special Considerations:**
- Shared IPs (NAT, corporate networks) may hit limits faster
- Consider /24 subnet limits for very aggressive blocking
- Allow some burst capacity for normal browsing

### Authenticated Users

**Identifier:** User ID from JWT

**Characteristics:**
- Higher limits than anonymous users
- Allow normal platform usage
- Prevent abuse from compromised accounts

**Typical Limits (indicative):**
- Read requests: Higher capacity and faster refill
- Write requests: Moderate limits
- Search queries: Reasonable limits
- Event creation: Limited to prevent spam
- Message sending: Rate limited to prevent spam

**Account Status Adjustments:**
- New accounts: Reduced limits for first 7 days
- Verified accounts: Standard limits
- Spam-flagged accounts: Significantly reduced limits
- Suspended accounts: All requests denied

### Trusted Users (Future)

**Identifier:** User ID with trust flag

**Requirements:**
- Payment on file (paid subscription or API access)
- Account age > 90 days
- Low spam report ratio (<0.1%)
- Manual verification for highest tier

**Characteristics:**
- Significantly higher limits
- API access
- Bulk operations permitted
- Real-time monitoring

**Trust Score Adjustments:**
- Spam reports reduce limits
- Successful API usage increases limits
- Violations lead to demotion to standard tier
- Review process for limit restoration

## Redis Integration

### Key Structure

**Key Naming Convention:**
```
rate_limit:{tier}:{identifier}:{bucket_type}
```

Examples:
- `rate_limit:anon:192.168.1.100:read`
- `rate_limit:auth:user_12345:write`
- `rate_limit:trusted:user_67890:api_read`

**TTL:**
- Set TTL on keys to auto-expire unused buckets
- TTL = maximum refill time + buffer
- Reduces memory usage for inactive users/IPs

### Redis Operations with Lua Scripts

**Atomic Check and Consume:**

Using Lua script ensures all operations are atomic:
- Fetch current bucket state
- Calculate token refill
- Check if sufficient tokens
- Deduct tokens if allowed
- Update bucket state
- Return result

**Benefits:**
- No race conditions
- Reduced network round-trips
- Consistent across all application instances
- Better performance than separate commands

### Redis Cluster Considerations

**Consistent Hashing:**
- Rate limit keys must map to same Redis node
- User ID or IP used in key ensures consistency
- No cross-node coordination needed

**High Availability:**
- Redis Sentinel or Redis Cluster for failover
- Brief window of inconsistency during failover acceptable
- Graceful degradation if Redis unavailable

**Fallback Strategy:**
- If Redis unavailable: Allow request with logging (fail open)
- Or: Deny request temporarily (fail closed)
- Health checks to detect Redis issues quickly
- Alert on Redis unavailability

## Warp Middleware Integration

### Middleware Structure

Rate limit middleware integrates with warp request handling:

**Responsibilities:**
1. Extract identifier (IP or user ID from JWT)
2. Determine bucket type based on request path/method
3. Determine tier (anonymous, authenticated, trusted)
4. Check rate limit via Redis
5. Allow or deny request
6. Add rate limit headers to response

### Middleware Application

**Selective Application:**
- Apply to all routes by default
- Exempt health check endpoints
- Different limits for different route groups
- Admin endpoints may have different limits

**Error Responses:**

On rate limit exceeded (HTTP 429):
```json
{
    "error": "rate_limit_exceeded",
    "message": "Too many requests. Please try again later.",
    "retry_after": 3600,
    "limit": 100,
    "remaining": 0,
    "reset": 1640995200
}
```

### Request Cost Calculation

**Variable Costs:**
- Simple read: 1 token
- Complex query: 5 tokens
- Write operation: 1 token
- Heavy computation: 10 tokens
- Bulk operation: 50 tokens

**Determination:**
- Endpoint annotation
- Request size/complexity
- Historical performance data
- Administrator configuration

## Response Headers

### Standard Headers

**On Every Response:**
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 742
X-RateLimit-Reset: 1640995200
```

**On Rate Limit Exceeded (429):**
```
Retry-After: 3600
```

**Optional Headers:**
```
X-RateLimit-Bucket: read
X-RateLimit-Tier: authenticated
```

### Header Meanings

- `X-RateLimit-Limit`: Maximum requests in current window
- `X-RateLimit-Remaining`: Requests remaining before limit
- `X-RateLimit-Reset`: Unix timestamp when limit resets/refills
- `Retry-After`: Seconds to wait before retrying (on 429)

## Monitoring and Observability

### Metrics to Track

**Rate Limit Metrics:**
- Requests allowed per tier/bucket
- Requests denied per tier/bucket
- Average tokens consumed per request
- Bucket utilization (how close to limits)
- 429 response rate

**Performance Metrics:**
- Rate limit check latency (Redis)
- Redis connection pool usage
- Lua script execution time
- Middleware overhead

**Security Metrics:**
- IPs hitting limits frequently
- Accounts hitting limits
- Potential abuse patterns
- Anonymous vs authenticated request ratios

### Alerting

**Alert Conditions:**
- High 429 rate (legitimate users hitting limits?)
- Single IP/user hitting limits repeatedly (abuse?)
- Redis unavailability
- Rate limit check latency spikes

### Dashboards

**Grafana Dashboards:**
- Rate limit status by tier
- Top IPs/users by requests
- 429 response trends
- Redis performance metrics
- See: [ADR-003 OpenTelemetry](../../adr/003-opentelemetry-observability.md)

## User Communication

### When Limits Are Hit

**Anonymous Users:**
- Clear message explaining limits
- Suggestion to create account for higher limits
- Link to registration
- Retry-After information

**Authenticated Users:**
- Explain they've hit rate limit
- Suggest waiting or spreading requests over time
- Contact support if limits too restrictive
- Information about trusted user tier (future)

**Error Messages:**
```
"You've made too many requests. Please wait 15 minutes and try again. 
Create an account for higher limits."
```

```
"Rate limit exceeded. Your limit resets in 42 minutes. 
If you need higher limits, consider our API access plan."
```

## Configuration

### Configurable Parameters

**Per Environment:**
- Enable/disable rate limiting (dev might disable)
- Rate limit tiers and values
- Redis connection details
- Fallback behavior (fail open/closed)

**Per Tier:**
- Bucket capacities
- Refill rates
- Request costs
- Special bucket types

**Dynamic Adjustment:**
- Admin can adjust limits without deployment
- A/B testing different limits
- Temporary limit increases for specific users
- Emergency limit reductions under attack

## Testing Strategy

### Unit Tests

- Token bucket math
- Redis key generation
- Tier determination logic
- Request cost calculation

### Integration Tests

- Middleware integration with warp
- Redis operations with Lua scripts
- Concurrent requests
- Bucket refill behavior
- Token consumption accuracy

### Load Tests

- Performance under high request volume
- Redis throughput
- Middleware overhead
- Race condition handling
- Burst handling

## Security Considerations

### Protection Against Evasion

**IP Rotation:**
- Monitor for patterns of IP switching
- Subnet-level blocking for aggressive attacks
- Require authentication for higher access

**Account Creation Spam:**
- Rate limit account creation per IP
- Email verification required
- New account limits reduced

**Distributed Attacks:**
- Monitor aggregate request rates
- Dynamic limit adjustment under attack
- Automatic scaling of infrastructure

### Privacy

**IP Address Storage:**
- IPs used for rate limiting only
- Not associated with user accounts (for anonymous)
- Logs retained for limited time
- Comply with GDPR and privacy regulations

## Integration with Other Features

### Authentication
- Different limits for anonymous vs authenticated
- JWT provides user ID for rate limiting
- See: [Authentication and Authorization](authentication-and-authorization.md)

### Trust & Safety
- Spam reports reduce rate limits
- Account restrictions enforced via rate limits
- See: [Trust and Safety](../trust-and-safety/)

### Observability
- Rate limit metrics in Grafana
- OpenTelemetry integration
- See: [ADR-003 OpenTelemetry](../../adr/003-opentelemetry-observability.md)

## Open Questions

1. What are the exact numeric values for each rate limit tier?
2. Should we implement per-endpoint rate limits or global per-user?
3. How do we handle legitimate users behind shared IPs (corporate NAT)?
4. Should we offer temporary limit increases on request?
5. What's the right balance between strict limits and user frustration?
6. Should we implement progressive enforcement (warnings before hard blocks)?
7. How do we prevent gaming the system (creating multiple accounts)?
8. Should we implement CAPTCHA as alternative to rate limiting?
9. What's the appropriate Redis memory allocation for rate limiting?
10. Should we implement rate limit bypass for emergency situations?
