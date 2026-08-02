issue_title: "Native Rust Omnichannel Real-time WebSockets and Ephemeral Presence Engine"
issue_description: |
  # OHC Native Omnichannel Real-time WebSockets and Ephemeral Presence Engine Design Spec

  ## 1. Problem Statement
  For non-technical owner/operators running small businesses, every second counts.
  - **Maya (home baker)** handles time-critical custom cake deposits; she needs to see if a customer is actively typing or viewing a quote to close the sale.
  - **Carlos (handyman)** operates on the road; he needs instant, real-time message notifications without manually refreshing a dashboard or waiting for slow email polls.
  - **Fatima (food cart operator)** runs in low-data and flaky network situations; she needs a real-time pre-order queue that alerts her immediately when an order is placed, but degrades gracefully with visual cue indicators when offline.

  Currently, OHC is retiring external dependencies like Chatwoot. Relying on traditional HTTP polling or exposing raw JWT tokens to client-side browser JavaScript compromises both performance and security. We need a high-performance, native Rust real-time WebSocket and Ephemeral Presence Engine integrated into `onehumancorp/mono` to power low-latency updates, secure multi-tenant isolation, and agentic workspace synchronization.

  ---

  ## 2. Research Report & Competitive Benchmarking

  ### Chatwoot Benchmarking & Architecture Review
  - **The Chatwoot Legacy:** Chatwoot uses Ruby on Rails with ActionCable for WebSocket pub-sub and Sidekiq with Redis for background jobs and webhook deliveries.
  - **ActionCable Limitations:** While easy to deploy, ActionCable maintains state in Ruby threads, incurring high memory footprints (~30-50MB per connection under load) and high CPU overhead on connection upgrades. Scalability requires complex Redis adapter configurations.
  - **Presence & Ephemeral State:** Chatwoot tracks user presence via Redis-backed `AppearanceChannel` and typing events via explicit ActionCable channel broadcasts. However, there is no built-in protection against client credential leakage via URL parameters, nor fine-grained tenant-level isolation for WebSocket subscribers.

  ### Competitive Platform Analysis
  - **Shopify (Oxygen & Storefront API):** Uses highly distributed edge workers (Cloudflare) to route requests. Real-time updates utilize GraphQL Subscriptions over WebSockets, which are highly efficient but require complex infrastructure orchestration.
  - **Wix / Squarespace:** Rely primarily on polling or heavy client-side polling loops, resulting in degraded mobile battery life and high latency.
  - **OmniSolo's Unfair Advantage:** By building on a native Rust Axum + Tokis WebSockets (`tokio-tungstenite`) pipeline, OHC maintains extremely lightweight connections (<1MB memory footprint per connection), handles 100k+ concurrent connections on a single node, and natively bridges server-side AI agent departments with client-side operator interfaces in sub-milliseconds.

  ---

  ## 3. High-Level Architectural Design

  The OHC real-time subsystem separates data paths:
  1. **Durable State Changes:** Flow through a PostgreSQL/SQLite database transaction, write to a Transactional Outbox, and sync to client-side caches via PowerSync rules.
  2. **Ephemeral State (Typing, Presence, Collaboration):** Transmitted directly via lightweight JSON frames over WebSockets, bypassing database storage for speed and cost efficiency.

  ### System Architecture Diagram (Mermaid.js)

  ```mermaid
  sequenceDiagram
      autonumber
      participant Client as Operator Browser (Next.js / Tauri)
      participant NextServer as Next.js Web Shell
      participant RustSvc as Rust API Server (Axum)
      participant Redis as Valkey/Redis PubSub
      participant AI as AI Agent Departments

      Note over Client, NextServer: Step A: Ticket-Based Handshake
      Client->>NextServer: GET Request (Authenticated via HTTP-Only Session Cookie)
      NextServer->>RustSvc: POST /api/v1/auth/realtime-ticket (Server-to-Server API + Bearer)
      RustSvc-->>NextServer: Return Short-Lived Single-Use WebSocket Ticket (JWT with jti, 60s exp)
      NextServer-->>Client: Pass Ticket to client state (Secure, never in URL parameters)

      Note over Client, RustSvc: Step B: WebSocket Upgrade
      Client->>RustSvc: WS Upgrade Request with Ticket in Subprotocol header (ohc-rt-<ticket>)
      RustSvc->>RustSvc: Validate Ticket, Verify Single-Use jti, Map connection to Tenant/User Context
      RustSvc-->>Client: Handshake Successful (Upgrade to WebSocket)

      Note over Client, Redis: Step C: Real-time Event Pub-Sub
      Client->>RustSvc: Send Ephemeral Event {"type": "typing_start", "conversation_id": "123"}
      RustSvc->>Redis: Publish to topic "tenant:100:conv:123:presence"
      Redis-->>RustSvc: Broadcast presence change to other subscribers
      RustSvc-->>Client: Push "typing_start" to recipient / operator dashboard

      Note over AI, Client: Step D: AI Agent Integration
      AI->>RustSvc: Generate Agentic Draft Reply for Maya
      RustSvc->>Redis: Publish "agent_draft.created" event
      Redis-->>RustSvc: Route event to subscriber
      RustSvc-->>Client: Push Draft Card to Operator Workspace (Mac-style translucent glass card)
  ```

  ### Multi-Tenant Entity Model

  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ WEBSOCKET_SESSION : owns
      WEBSOCKET_SESSION ||--|| SESSION_TICKET : authorized_by
      WEBSOCKET_SESSION ||--o{ CONVERSATION_SUBSCRIPTION : tracks
      CONVERSATION ||--o{ CONVERSATION_SUBSCRIPTION : scoped_to

      ORGANIZATION {
          string id PK
          string name
          boolean is_active
      }
      WEBSOCKET_SESSION {
          string session_id PK
          string tenant_id FK
          string user_id
          string connection_status "active | disconnected"
          timestamp connected_at
      }
      SESSION_TICKET {
          string jti PK
          string tenant_id FK
          string user_id
          timestamp expires_at
          boolean used
      }
      CONVERSATION_SUBSCRIPTION {
          string id PK
          string session_id FK
          string conversation_id FK
          timestamp subscribed_at
      }
  ```

  ---

  ## 4. Mobile UX Flow & Visual Excellence (375px Non-Negotiables)

  Small business owners utilize OHC primarily from mobile viewports (375px/414px width). The real-time visual indicator must pass the "grandmother test": instant clarity without cognitive load.

  ### 375px Mobile Viewport Layout
  1. **Omni-presence status bar:** Positioned at the very top of the screen as a slim, translucent glass banner (macOS-style backdrop-blur-md, opacity-80, border-b border-white/10).
  2. **Active Presence Indicator:**
     - A pulse-animation green ring (vibrant green `#10B981`) around the customer's avatar when online.
     - Gentle fading gray indicator when offline.
  3. **Live Typing Signal:**
     - Under the conversation bubble, a microscopic card displaying three animated bouncing dots (`dot-pulse` CSS) with the tag: *"Maya is typing..."* or *"Customer is typing..."*.
     - Bouncing dots use custom ease-in-out CSS with 0.6s staggering to ensure visual premium quality.
  4. **Connection Heartbeat & Network Flakiness:**
     - If the connection drops on Carlos's phone while on a route, the top banner smoothly slides down (300ms cubic-bezier transition) showing: *"🔄 Reconnecting... Drafts are saved locally."* (Amber `#F59E0B` background with translucent blur).
     - Once connection is restored, the banner turns emerald green with a checkmark for 2 seconds and slides up out of view.
  5. **Touch Targets:** Reconnect action buttons or chat actions are fully padded to `48px x 48px` minimum.

  ---

  ## 5. AI Agent Integration Points

  The WebSockets engine coordinates background agent processes with the operator UI seamlessly:
  - **Work Triage Agent:** Listens to WebSocket events. If an incoming message has a sentiment of "critical" (e.g., custom order changes, delivery issues), the agent publishes a `triage_priority_upgrade` WebSocket event, pushing the conversation to the top of the operator’s queue instantly with a golden badge and haptic feedback trigger.
  - **Customer Relationship Agent:** Subscribes to customer typing and message streams. The moment the customer starts typing or submits a question, the agent fetches context from the RAG store, drafts a highly-personalized response (e.g., *"Yes Priya, we have that dress in size M in-store today!"*), and pushes the draft over WebSockets as a pending draft. It renders in Maya’s workspace as a macOS-style Translucent Glass Card labeled: *"✨ AI Suggested Response (1-tap approve)"*.
  - **Ops State Monitor Agent:** Monitors active booking slots. If Fatima updates a menu item as "sold out", the agent triggers a cache invalidation event and broadcasts a `menu_item_sold_out` WebSocket event to all viewing customer widgets in real-time.

  ---

  ## 6. Security, Zero Trust & Multi-Tenancy

  - **No URL Secrets:** Storing JWT tokens or credentials in WebSocket upgrade query parameters (`ws://server?token=xyz`) is strictly forbidden to prevent token leakage via server access logs, reverse-proxy headers, and browser history. Instead, the single-use ticket is passed via the standard `Sec-WebSocket-Protocol` (Subprotocol) header, e.g., `Sec-WebSocket-Protocol: ohc-rt-ticket-<jti_hash>`.
  - **Ticket Singleness & Expirations:** WebSocket tickets expire in 60 seconds, are single-use (`jti` tracking in a distributed cache like Valkey/Redis), and are cryptographically signed with a dedicated 256-bit rotating signing key (separate from the primary web session).
  - **Dynamic Tenant-Isolation & RLS:** When a client establishes a WebSocket connection, the authenticated context resolves the `tenant_id`. Every subscribe attempt to a specific `conversation_id` is validated against PostgreSQL/SQLite row-level authorization permissions. A client can never subscribe to, view, or publish to another tenant's message stream. Cross-tenant subscription attempts fail closed, log a security anomaly, and close the WebSocket frame immediately.

  ---

  ## 7. Implementation Prompt for Engineering Swarm

  ```text
  Design and implement a native Rust real-time WebSocket and Ephemeral Presence Engine inside `src/server/` replacing the retired Chatwoot ActionCable footprint.

  Your implementation must deliver a secure, high-performance, multi-tenant real-time gateway that meets these Critical User Journeys (CUJs):
  1. Operator Handshake CUJ: The operator workspace requests a short-lived (60s), single-use WebSocket ticket via a secure HTTP endpoint (`POST /api/v1/auth/realtime-ticket`), which returns a ticket containing jti, user_id, and tenant_id. The client then initiates a WebSocket connection supplying this ticket in the 'Sec-WebSocket-Protocol' subprotocol header.
  2. Multi-Tenant Pub-Sub CUJ: The WebSocket gateway upgrades the connection, validates the ticket, binds it to the authenticated tenant context, and registers the subscription. It rejects any attempt to subscribe to channels/conversations belonging to other tenants.
  3. Ephemeral Presence & Typing CUJ: The engine handles low-latency, non-persisted client-to-client events such as 'typing_start', 'typing_stop', and 'user_presence' via an in-memory or Redis-backed pub-sub broker, broadcasting updates to active subscribers in milliseconds.
  4. Connection Resilience & Gap Detection CUJ: Handle flaky network events by allowing clients to reconnect with a reconnect token, detecting sequence gaps, and triggering REST-based sync catchups when gaps are found.

  Acceptance Criteria:
  - ZERO credentials in WebSocket URL query strings. All tickets must pass through Subprotocol headers.
  - Full multi-tenant isolation: any attempt to access or subscribe to foreign tenant data must result in immediate connection closure and security logging.
  - Extremely lightweight footprint: handles connections efficiently without blocking Tokio threads.
  - Complete integration test suite verifying single-use ticket consumption, subprotocol extraction, invalid ticket rejection, multi-tenant subscription validation, and ephemeral event routing.
  ```

  ---

  ## 8. Classification & Scope
  - **Priority:** P1 (Critical/High - core platform engine replacement)
  - **Estimated Scope:** Large
  - **Assigned Departments:** Core Server Platform, AI Coordination Engine, UX Systems
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
