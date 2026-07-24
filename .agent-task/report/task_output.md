issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Native Rust Omnichannel Chat System Architecture

  **Problem Statement**
  The current platform relies on a third-party chat provider, which limits control, performance, multi-tenancy guarantees, and deep integration with our backend systems. For owner/operators like Maya, Carlos, and Priya, customer messages (DMs, SMS, Email, WhatsApp) are currently scattered or dependent on external latency. We need a blazing-fast, natively integrated chat system built in Rust that strictly guarantees tenant isolation and powers our AI assistants.

  **Research Report**
  As mandated, Chatwoot has been fully retired as an external service. A detailed codebase audit of `chatwoot/chatwoot` reveals its core strengths:
  - Robust `Conversation`, `Message`, and `Contact` data models.
  - Channel adapters for web widget, API, email, and social.
  - ActionCable/WebSocket integration for real-time presence and message delivery.
  - Inbox routing and SLA rules.
  However, replicating this in Ruby/Rails is insufficient for our latency and scaling goals. A native Rust implementation using Axum (for HTTP) and Tokio/Tungstenite (for WebSockets), backed by PostgreSQL (with Row-Level Security for multi-tenancy) and Redis (for pub/sub and presence), is required.

  **Design Doc**
  - **Architecture Diagram (Mental Model)**
    - Clients (Flutter Mobile/Web) <-> Nginx/Ingress <-> Rust API Gateway (Axum)
    - Rust API Gateway <-> Auth/Identity Service (SPIFFE/SPIRE)
    - Rust API Gateway <-> Chat Service (Rust, Tokio)
    - Chat Service <-> PostgreSQL (RLS enabled: `tenant_id` forced via session context)
    - Chat Service <-> Redis (Pub/Sub for WebSocket message fanout across pods)
    - Chat Service <-> Background Job Queue (for sending emails/SMS out)
  - **Mobile UX Flow**
    - 375px first: Unified Inbox view. Tap conversation -> Chat detail view.
    - Glassmorphism design tokens applied to message bubbles.
    - Persistent bottom compose bar with AI-drafting suggestions visible.
  - **AI Agent Integration**
    - The `Customer & Relationship Assistant` runs as a daemon. It subscribes to the Redis topic for new messages.
    - For relevant messages (e.g., Maya's Instagram DMs), the agent uses Gemini Pro to draft a response and saves it as a `Draft` message in the database. The UI reflects this draft state immediately via WebSocket.
  - **Key Design Decisions**
    - **Multi-tenancy:** Strict PostgreSQL RLS on all tables (`conversations`, `messages`, `contacts`). The database connection pool sets the `tenant_id` role for every request.
    - **Real-time:** Rust Axum WebSocket handlers upgrade connections. Redis pub/sub fans out messages so any connected device gets updates instantly.
    - **Offline-first:** Flutter frontend caches conversations locally. Mutations are queued and retried with idempotency keys.

  **Implementation Prompt**
  "Implement the core Rust backend for the native omnichannel chat system.
  1. Define the PostgreSQL schema (Conversations, Messages, Contacts, Inboxes) with strict Row-Level Security (`tenant_id`).
  2. Build the Rust Axum REST API endpoints for fetching conversations and sending messages.
  3. Implement the WebSocket server in Axum, integrating with Redis for pub/sub message distribution.
  4. Ensure 100% unit test coverage for the API and WebSocket logic.
  5. The system must support the 'Maya' persona (unified inbox for DMs) with zero mock data. All data must flow through the real database."

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
