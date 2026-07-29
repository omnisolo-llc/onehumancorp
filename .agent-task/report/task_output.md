issue_title: "Implement Custom Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Problem Statement:** OneHumanCorp currently lacks a native, unified omnichannel customer support engine. Chatwoot has been fully retired as an external service due to high latency, poor multi-tenant scaling, and the inability to natively integrate with OHC's AI agents. Small business owners (like Carlos and Maya) require an integrated, high-performance inbox that seamlessly handles Instagram DMs, WhatsApp, SMS, and email. This inbox must be deeply embedded into OHC to allow our AI agents (The Ambassador, The Manager) to instantly draft context-aware replies without relying on third-party webhooks.

  **Research Report:**
  - **Competitor Audit:** Platforms like Shopify Inbox provide basic aggregation but fail at proactive AI drafting and unified customer identity resolution across channels. Zendesk and Intercom are too complex for solopreneurs.
  - **Chatwoot Source Code Audit:** Chatwoot relies on Ruby on Rails controllers and PostgreSQL for multi-tenancy, using Sidekiq for background jobs and ActionCable for WebSockets. Its `Account` model maps to OHC's `Tenant`, its `Inbox` and `Channel` models map to external connectors, and `Conversation` links a `Contact` to a `Message` stream.
  - **OHC Opportunity:** By building a 100% native Rust clone of the Chatwoot feature set, we eliminate external dependencies. We can build this using `axum` (for REST APIs), `tokio-tungstenite` (for high-concurrency WebSockets), and strictly isolated row-level security in PostgreSQL for multi-tenancy.

  **Design Doc:**
  - **Architecture Diagram:**
  ```mermaid
  graph TD
      A[Social Media/Email Channels] -->|Webhooks| B(Omnichannel API Gateway)
      B --> C[Rust Chat Services axum]
      C --> D[Unified Identity Resolution Engine]
      C --> E[(PostgreSQL: RLS Isolated)]
      E --> F[Agent Action Queue]
      F --> G[The Ambassador AI Agent]
      G -->|Proactive Drafts| E
      C --> H[WebSocket Pub/Sub tokio-tungstenite]
      H --> I[Mobile Flutter Client 375px]
  ```
  - **Data Model Invariants (Rust/SQLx):**
    - `ChatInbox`: `id`, `tenant_id`, `name`
    - `ChatChannel`: `id`, `tenant_id`, `inbox_id`, `channel_type`, `config`
    - `ChatContact`: `id`, `tenant_id`, `name`, `email`, `phone`, `omnichannel_ids`
    - `ChatConversation`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status`
    - `ChatMessage`: `id`, `tenant_id`, `conversation_id`, `sender_type`, `content`
    *Every table MUST include `tenant_id` enforced by PostgreSQL Row Level Security (RLS).*

  - **Mobile UX Flow (375px First):**
    1. **Inbox Feed:** Owner opens the app and sees a unified vertical feed of unread conversations. Each conversation card (minimum 44x44px touch target) clearly indicates the channel source (e.g., WhatsApp icon) and highlights if an AI draft is pending.
    2. **Conversation View:** Tapping a card opens a detailed chat view. The top half displays customer context (recent orders, lifetime value). The bottom half contains the message thread and native mobile keyboard input.
    3. **Agent Interaction:** If a customer asks a question, an "AI Draft Ready" translucent glassmorphism card appears above the input field. The owner can tap "Approve & Send" to instantly dispatch the message.

  - **AI Agent Integration Points:**
    - Incoming messages trigger a background worker (via PostgreSQL `SKIP LOCKED` job queue) that invokes **The Ambassador AI Agent**.
    - The Ambassador queries the customer's history and current product catalog, drafts a response, and inserts it into the database with `status = pending_approval`.
    - This pending message is pushed to the owner's mobile client via WebSockets.

  **Implementation Prompt:** Implement the backend Rust microservices and database migrations to fully support this omnichannel chat architecture.
  1. Create the necessary SQL migrations for the entities (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) with strict RLS multi-tenancy.
  2. Implement the `axum` API endpoints for managing these entities, ensuring all endpoints require a valid tenant context from SPIFFE/SPIRE claims.
  3. Build a WebSocket handler using `tokio-tungstenite` that allows the mobile client to subscribe to real-time conversation updates.
  4. Provide a suite of Playwright E2E tests simulating a customer sending an external message, the AI drafting a response, and the owner approving it via the mobile UI. Ensure no external APIs are mocked in the E2E flow (use local adapters if necessary).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust, chat]
assignees: []
