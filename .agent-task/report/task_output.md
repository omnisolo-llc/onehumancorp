issue_title: "Native Rust Omnichannel Real-Time Engine & Inbox Architecture"
issue_description: |
  **Title**: Native Rust Omnichannel Real-Time Engine & Inbox Architecture

  **Problem Statement**:
  Business owners like Maya (baker) and Carlos (handyman) receive inquiries across multiple channels (Instagram DMs, WhatsApp, SMS, web chat) but struggle to keep track of conversations, leading to missed opportunities. Currently, they have to switch between 4-5 apps. They need a unified inbox where an AI work assistant can seamlessly triage messages, suggest drafts, and sync customer context in real time. OHC requires a highly scalable, multi-tenant, real-time messaging architecture natively built in Rust to unify these streams invisibly for the owner.

  **Research Report**:
  - **Market Context**: We benchmarked Tencent Workbuddy, Front, Intercom, and Shopify Inbox. Operators consistently report that external aggregators are too technical or decoupled from their business data (orders, bookings, quotes).
  - **Competitor Insights**: Shopify Inbox wins for small businesses by coupling DMs directly with cart/checkout state. Intercom handles large scale but is too complex for small owners. OHC must integrate core commerce (quotes, deposits, bookings) natively into the chat layer.
  - **Technical Findings**: High-performance, low-latency, and safe concurrency are best achieved via native Rust. Many modern high-scale chat infrastructure providers (like Discord) leverage Rust for its memory safety, Tokio-based async model, and WebSocket handling capabilities, handling millions of concurrent connections efficiently.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    graph TD
      A[Client: Flutter Mobile & PWA] -->|WebSocket/SSE| B(Rust Real-Time Gateway)
      B --> C(Omnichannel Inbox Controller)
      C --> D[WhatsApp Adapter]
      C --> E[Instagram DM Adapter]
      C --> F[Web Chat Adapter]
      C --> G[SMS Adapter]
      C --> H(AI Agent Triage Department)
      C --> I[(PostgreSQL: RLS Tenant Isolated)]
      C --> J[(Redis: Redlock & Session Cache)]
    ```
  - **UI wireframes or screen flow description (375px first)**:
    - **Inbox Tab (375px)**: Ubiquiti-style unified list of conversations. Status dots indicating unread or pending agent drafts.
    - **Conversation View (375px)**: Translucent glass sticky header showing customer context (e.g. "Maya's custom cake order: $50 deposit pending"). Messages inline. A prominent "Agent Draft" card floats above the native keyboard, allowing the owner to tap "Send" or edit.
  - **Mobile UX flow**:
    1. Owner receives push notification of new Instagram DM.
    2. Taps notification, opens OHC Inbox (loads < 200ms via offline caching).
    3. Sees the customer's message alongside past order history.
    4. AI Customer Assistant has already drafted a response proposing a delivery date.
    5. Owner taps 'Send'. The Rust gateway routes it back to Instagram instantly.
  - **AI agent integration points**:
    - The AI Triage Department intercepts incoming messages before they trigger mobile push.
    - AI generates contextual drafts and tags urgency.
    - Data models must support `SessionOperationEnvelope` and `AttemptCommandEnvelope` for async AI interventions.
  - **Key design decisions and why**:
    - **Native Rust Engine**: Chosen for safe concurrency and minimal latency in handling WebSocket/SSE connections for real-time messaging.
    - **Multi-Tenant RLS**: Total isolation at the PostgreSQL level using `tenant_id` on every message and conversation table.
    - **Open Source Leverage**: Use `tokio` for async runtime, `axum` for HTTP/WebSocket routing, `sqlx` for RLS-aware Postgres queries, and `redis-rs` for distributed locking, adhering to the standard of using mature open-source crates over bespoke infrastructure.

  **Implementation Prompt**:
  Build the native Rust omnichannel real-time engine and inbox system in the `omnisolo-llc/onehumancorp` backend.
  - Implement a WebSocket gateway capable of maintaining thousands of concurrent connections.
  - Create the multi-tenant data models for `Conversation`, `Message`, `ChannelAdapter`, and `Inbox` ensuring strict `tenant_id` isolation.
  - Scaffold channel adapter interfaces for WhatsApp and Instagram.
  - Ensure the AI Triage agent can securely hook into the incoming message stream to draft replies.
  - Provide complete unit testing for the real-time events and data persistence.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
