# ADR-002: Server-Side Rendering with HTMX for Web Frontend

## Status

**ACCEPTED** - 2025-12-28

## Context

cwrdd needs a web frontend that allows users to interact with the platform. The frontend must:

- **Be performant**: Fast initial page loads and responsive interactions are critical to user experience
- **Be secure**: Minimize attack surface and protect user data
- **Be accessible**: Work for users with varying abilities and network conditions
- **Support progressive enhancement**: Function without JavaScript where possible, enhance with JavaScript where available
- **Be maintainable**: Simple to understand, develop, and evolve over time
- **Be cost-effective**: Lower complexity = fewer resources needed = ability to serve more users

Modern frontend architectures typically fall into two categories:

1. **Client-Side Rendering (CSR)**: Single-Page Applications (SPAs) built with React, Vue, Angular, etc. JavaScript frameworks render the UI in the browser.

2. **Server-Side Rendering (SSR)**: HTML is generated on the server and sent to the browser. JavaScript enhances the experience but isn't required for core functionality.

We must choose an approach that aligns with cwrdd's principles while delivering an excellent user experience.

## Decision

We will implement the web frontend using **server-side rendering (SSR)** with **Askama** templates and **HTMX** for progressive enhancement.

### Architecture

The **web** application will serve an MVC-style frontend:

```
web/
├── handlers/        # HTTP request handlers (Controllers)
├── templates/       # Askama HTML templates (Views)
├── middleware/      # Request/response middleware
└── main.rs         # Application composition
```

**Technology choices:**

- **Askama**: Type-safe, compiled Rust templates with syntax similar to Jinja2
- **HTMX**: Lightweight JavaScript library for AJAX, WebSockets, and Server-Sent Events using HTML attributes
- **CSS**: Standard CSS (framework TBD based on design needs)

### How It Works

1. **Initial request**: User requests a page → Server renders HTML with Askama → Full page sent to browser
2. **Subsequent interactions**: HTMX intercepts clicks/form submissions → Sends AJAX request → Server returns HTML fragment → HTMX swaps fragment into page
3. **Real-time updates**: HTMX can use WebSockets or Server-Sent Events for push notifications

### Example

**Template (Askama):**
```html
<div id="message-list">
  {% for message in messages %}
    <div class="message">{{ message.content }}</div>
  {% endfor %}
</div>

<form hx-post="/messages" hx-target="#message-list" hx-swap="beforeend">
  <input type="text" name="content" />
  <button type="submit">Send</button>
</form>
```

**Handler (Rust):**
```rust
async fn create_message(form: MessageForm) -> Result<Html<String>> {
    let message = message_service.create(form.content).await?;
    render_message_template(message) // Returns HTML fragment
}
```

HTMX sends the form via AJAX, receives an HTML fragment of the new message, and inserts it into the message list—no JavaScript code written.

## Rationale

### Performance Benefits

1. **Fast Time-to-First-Byte (TTFB)**: No JavaScript bundle to download and parse before rendering
2. **Fast First Contentful Paint (FCP)**: HTML arrives ready to render
3. **Reduced bandwidth**: Only HTML fragments are sent for updates, not JSON + templating logic
4. **Server-side caching**: Rendered HTML can be cached at multiple layers (CDN, reverse proxy, application)
5. **Compiled templates**: Askama templates are compiled to Rust code at build time, eliminating runtime template parsing overhead

**Impact**: Users on slow networks or low-end devices get a fast, usable experience immediately.

### Security Benefits

1. **Reduced attack surface**: No large JavaScript bundles that could contain vulnerabilities or be compromised
2. **Server-side validation**: All data validation happens on the server where it can't be bypassed
3. **CSP-friendly**: Content Security Policy is easier to implement without inline scripts or eval
4. **XSS protection**: Askama automatically escapes output by default, preventing most XSS vulnerabilities
5. **Less client-side state**: Sensitive data doesn't need to live in browser memory or local storage
6. **CSRF protection**: Traditional CSRF tokens work naturally with form submissions

**Impact**: Smaller attack surface = fewer vulnerabilities = better security for users.

### Accessibility Benefits

1. **Progressive enhancement**: Core functionality works without JavaScript
2. **Semantic HTML**: Server-rendered HTML encourages proper semantic structure
3. **Screen reader friendly**: Standard HTML navigation and form patterns work by default
4. **Keyboard navigation**: Native browser keyboard support without custom JavaScript

**Impact**: Platform is accessible to more users, including those with disabilities.

### Developer Experience Benefits

1. **Single language**: Developers write Rust for backend and frontend logic
2. **Type safety**: Askama templates are type-checked at compile time
3. **Simpler mental model**: No state synchronization between client and server
4. **Less tooling**: No separate build pipeline for frontend JavaScript
5. **Faster builds**: No npm, webpack, or similar frontend build tools needed
6. **Easier debugging**: Server-side rendering means all logic is debuggable on the server

**Impact**: Faster development, fewer bugs, easier onboarding.

### Cost Benefits

1. **Lower infrastructure costs**: Server-side rendering on Rust is extremely efficient
2. **Simpler architecture**: No separate frontend build/deployment pipeline
3. **Reduced monitoring complexity**: One application to monitor instead of frontend + backend
4. **Lower bandwidth usage**: HTML fragments are smaller than JSON + client-side rendering

**Impact**: Lower costs = ability to serve more users.

## Consequences

### Positive

1. **Excellent performance**: Fast initial loads, efficient updates, low bandwidth usage
2. **Strong security posture**: Reduced attack surface, server-side validation, automatic XSS protection
3. **High accessibility**: Progressive enhancement ensures broad compatibility
4. **Simple architecture**: One codebase, one deployment, one language for full-stack development
5. **Type-safe templates**: Compile-time checking prevents entire classes of template errors
6. **Cost-efficient**: Rust's performance means excellent resource utilization

### Negative

1. **Server load**: All rendering happens server-side, which increases CPU usage compared to client-side rendering
2. **Latency sensitivity**: Each interaction requires a server round-trip (mitigated by HTMX's efficiency)
3. **Rich interactions**: Very complex, highly interactive UI components (e.g., canvas-based drawing) are harder to implement
4. **Limited offline capability**: Application requires server connectivity for most functionality
5. **Learning curve**: Team members familiar with React/Vue/Angular need to adjust mental models

### Mitigation Strategies

- **Caching**: Aggressive use of HTTP caching, CDN caching, and server-side fragment caching to reduce server load
- **Horizontal scaling**: Stateless web servers scale easily to handle increased load
- **WebSockets**: Use WebSockets for real-time features to reduce round-trip latency
- **Progressive enhancement**: Enhance with vanilla JavaScript for truly performance-critical interactions if needed
- **Edge deployment**: Deploy close to users to minimize latency

### When Client-Side JavaScript Makes Sense

We're not dogmatic. Client-side JavaScript is appropriate for:
- Very complex, highly interactive UI components (e.g., rich text editors, drawing tools)
- Real-time collaborative features that need optimistic updates
- Offline-first functionality

If these needs arise, we can integrate targeted JavaScript components while maintaining the SSR foundation.

## Alternatives Considered

### React/Vue/Angular SPA

**Pros**: Rich ecosystem, excellent tooling, familiar to many developers, great for complex interactions

**Cons**: Large bundle sizes, slow initial loads, complex state management, security vulnerabilities in dependencies, requires separate frontend deployment pipeline, higher infrastructure costs, JavaScript fatigue

**Why not chosen**: Conflicts with our principles of performance, simplicity, and cost-efficiency. Most users don't need the complexity of an SPA.

### Next.js/Nuxt.js (JavaScript SSR)

**Pros**: Server-side rendering with JavaScript, large ecosystem, good performance

**Cons**: Still requires JavaScript for hydration, complex build pipeline, Node.js runtime (less efficient than Rust), dependency management overhead, separate frontend codebase

**Why not chosen**: Adds complexity and inefficiency compared to native Rust SSR. Doesn't leverage our choice of Rust.

### Traditional Server-Side MVC (without HTMX)

**Pros**: Simple, proven approach, works without JavaScript

**Cons**: Full page reloads for every interaction, poor perceived performance, feels dated to modern users

**Why not chosen**: HTMX provides modern UX without the downsides of SPAs, giving us the best of both worlds.

## Migration Path

This architecture supports future evolution:

1. **Phase 1 (current)**: SSR with HTMX for all frontend interactions
2. **Phase 2 (if needed)**: Add targeted client-side JavaScript for specific complex components
3. **Phase 3 (if needed)**: Extract specific features to separate SPA if truly necessary

The SSR foundation can coexist with client-side JavaScript where it adds value.

## RESTful API

In addition to the MVC-style frontend, the web application will serve a **RESTful HTTP API** for:

- Mobile applications (future)
- Third-party integrations (future)
- Programmatic access

The API will:
- Accept and return JSON
- Use the same business logic as the web frontend (through application layer services)
- Be versioned to maintain backward compatibility
- Use token-based authentication (JWT or similar)

Both the web frontend and the API are served by the same **web** application, ensuring consistency and simplicity.

## References

- [HTMX Documentation](https://htmx.org/)
- [Askama Documentation](https://djc.github.io/askama/)
- [Hypermedia Systems by Carson Gross](https://hypermedia.systems/)
- [The Cost of JavaScript in 2023 by Addy Osmani](https://web.dev/the-cost-of-javascript-in-2019/)
- [Progressive Enhancement (MDN)](https://developer.mozilla.org/en-US/docs/Glossary/Progressive_Enhancement)

## Notes

This ADR establishes the frontend architecture for cwrdd. Specific design patterns, component structures, and HTMX usage patterns will be documented as the frontend develops.

The choice of SSR + HTMX aligns perfectly with cwrdd's core principles: **people deserve high-performance, bug-free technology that respects their privacy and security.**
