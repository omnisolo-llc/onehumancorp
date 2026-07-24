issue_title: "Architecture Design: Native Rust Multi-Tenant Omnichannel Unified Inbox"
issue_description: |
  # Native Rust Multi-Tenant Omnichannel Unified Inbox

  ## Problem Statement
  OHC requires a high-performance, natively-integrated multi-tenant omnichannel unified inbox. Currently, the legacy external chat service is 100% retired. Small business owners like Maya, Carlos, and Fatima receive demand via multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat) and they need a unified, lightning-fast "Work Triage" view to coordinate these conversations. Without a native solution, OHC cannot provide the deep AI-agent integration necessary for autonomous customer relationships and cart recovery operations. We need a native Rust implementation that achieves feature parity with the legacy external chat service while adhering to our zero-trust, multi-tenant architectural constraints.

  ## Research Report & Feature Benchmarking
  An exhaustive audit of the original upstream source code was conducted to extract the core conversational data model and functional invariants:
  - **Conversations & Messages:** Central entity linking an Account (tenant), Inbox, Assignee, and Contact. Messages belong to Conversations.
  - **Inboxes & Channels:** Inboxes act as routing targets, backed by specific Channel models (e.g., WhatsApp, Email, Web Widget, API).
  - **Real-Time WebSocket Sync:** Real-time event propagation via WebSocket (ActionCable in Ruby, needing a Tokio/Axum WebSocket replacement in Rust).
  - **AI & Automation:** Webhooks, Automation Rules, SLA Policies, Macros, and Canned Responses all hinge on conversation status changes.

  Compared to Shopify Inbox or Wix Chat, this new OHC Unified Inbox must go further by natively exposing these real-time streams to OHC's AI job queue (via Postgres `SKIP LOCKED` / Redis) so that the Customer Assistant agent can instantly draft responses.

  ## Design Doc
  ### High-Level Architecture
  - **Microservice/Crate:** A new `ohc_chat` Rust crate within the mono repo.
  - **Transport Layer:** Axum (HTTP + WebSockets) running on Tokio, utilizing SPIFFE/SPIRE for service-to-service auth.
  - **Data Model (PostgreSQL):**
    - `tenant_id` on every table with Row Level Security (RLS) enabled.
    - Entities: `inboxes`, `channel_configs`, `contacts`, `conversations`, `messages`, `conversation_participants`.
  - **Distributed Events & Real-time:**
    - Incoming webhooks/messages are published to Redis (Pub/Sub for connected WebSockets) and PostgreSQL (for persistent event sourcing).
    - AI Agent background processing via Postgres-backed job queues.
  - **Mobile UX Flow (375px first):**
    - The "Triage" tab on mobile displays a unified list of active conversations.
    - Translucent glass UI components over a clean UniFi-style layout.
    - 44x44px touch targets for quick actions (Draft AI Reply, Snooze, Resolve).
    - Offline-tolerant reads using Flutter local database cache.

  ### AI Agent Integration
  - When a message is saved, an event `ohc:chat:message_created` triggers a check against the tenant's AI routing rules.
  - The `Customer & Relationship Assistant` can be invoked automatically to draft a reply. Its draft is saved with `status="pending_approval"`, appearing distinctively in the owner's mobile feed.

  ## Implementation Prompt
  **User Facing Outcome:** A business owner opens the OHC mobile app (Triage screen) and sees messages from WhatsApp, Instagram, and Web Chat in one seamless feed. They can tap to let the AI draft a reply, or type a response that routes back through the native channel.

  **Task for Implementer:**
  Create the initial Rust crate (`ohc_chat`) under the mono repo with the base data model and an Axum server handling WebSocket connections and basic message CRUD.
  - Define PostgreSQL migrations for `conversations`, `messages`, `inboxes`, and `contacts`. Ensure `tenant_id` and RLS are strictly enforced.
  - Implement a token-authenticated WebSocket endpoint in Axum that broadcasts message payloads to subscribed clients (using Redis Pub/Sub for horizontal scaling).
  - Provide a Flutter (mobile-first 375px) stub for the unified inbox screen fetching data from this API.
  - Acceptance Criteria: A message created via REST API is successfully broadcasted via WebSocket to a connected mock Flutter client for the same `tenant_id`. Zero mock data—all must go through the Postgres DB.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
