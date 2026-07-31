issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol: Architectural Design for Native Rust Omnichannel Chat System

  ### Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as an external third-party dependency for its omnichannel customer support and chat system. Relying on an external service creates latency, complicates multi-tenant isolation, and prevents tight integration with OHC's internal AI agents (Customer & Relationship Assistant, Operations Assistant). A native Rust implementation ensures strict multi-tenant isolation (Zero Trust/SPIFFE), high-performance WebSocket messaging, and a mobile-first (375px) agent workflow optimized for non-technical owner/operators (e.g., Maya the baker, Carlos the handyman).

  ### Research Report & Findings
  An exhaustive audit of the `chatwoot/chatwoot` repository (v3.x) revealed the following core architectural pillars that need to be replicated and enhanced in OHC's native Rust environment:
  - **Data Models:** The foundational models revolve around `Account` (Tenant), `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  - **Channel Adapters:** Chatwoot relies on polymorphic channel associations (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Api`, `Channel::Whatsapp`, `Channel::Twitter`).
  - **Real-Time Communication:** Driven by ActionCable (Ruby on Rails). In Rust, this must be replaced by a high-throughput WebSocket server (e.g., `tokio-tungstenite` or `axum` WebSockets) with Redis Pub/Sub for cross-node broadcasting.
  - **AI Agent Integration:** Chatwoot uses `AgentBot` and `AgentBotInbox`. OHC will integrate its proprietary AI job queue (PostgreSQL `SKIP LOCKED`) and LLM orchestration natively.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ USER : employs
      INBOX ||--o{ CONVERSATION : receives
      INBOX ||--|{ CHANNEL : connects
      CONTACT ||--o{ CONVERSATION : participates
      USER ||--o{ INBOX_MEMBER : assigned_to
      INBOX ||--o{ INBOX_MEMBER : has
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }|--|| CONTACT : belongs_to
      MESSAGE }|--|| CONVERSATION : belongs_to
  ```

  #### Mobile-First UX Flow (375px)
  - **Screen 1 (Work Triage Dashboard):** 375px optimized feed. Unified view of new messages across all channels (WhatsApp, Web, Email). Large 44x44px touch targets.
  - **Screen 2 (Conversation View):** Translucent glass UI for the message thread. Native mobile keyboard support. Real-time updates via WebSockets. AI "Draft Reply" button prominent at the bottom, above the text input.
  - **Screen 3 (Contact Context):** Swipe-right drawer showing contact tags, past purchases, and AI-summarized sentiment.

  #### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Natively subscribes to the Redis `ohc:events:message_created` stream. Evaluates the incoming message using tenant-scoped memory and drafts a response in the `Message` table with `status = pending_approval`.
  - **Agent Routing & Handoff:** If the AI cannot resolve the intent, it transitions the `Conversation` status to `open` and notifies human agents via WebSocket.

  #### Key Design Decisions
  - **Multi-Tenant Isolation:** Enforced via PostgreSQL Row-Level Security (RLS) on all tables (`tenant_id`).
  - **Native Rust Axum API:** Utilizing `axum` for standard REST routes and WebSocket termination, offering significantly lower latency than the previous Ruby-based architecture.
  - **Strict Mobile Parity:** The frontend will be a Flutter PWA conforming strictly to OHC Premium Token library with translucent materials.

  ### Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core database schema, models, and Axum routing for the Native Rust Omnichannel Chat Engine based on the design document above.
  **CUJ:** Maya the home baker receives an Instagram DM. The message is ingested via a webhook, creates a `Contact` (if new), a `Conversation`, and a `Message` in the OHC database. The Customer Assistant AI drafts a reply, and Maya views the conversation and the drafted reply on her 375px mobile screen.
  **Acceptance Criteria:**
  1. PostgreSQL schema implemented for `Inboxes`, `Conversations`, `Messages`, and `Contacts` with `tenant_id` RLS.
  2. Rust `axum` backend handles a basic webhook ingestion for a new message.
  3. WebSocket endpoint broadcasts the new message to subscribed clients.
  4. Flutter frontend renders a mobile-first (375px) conversation view with translucent styling and minimum 44x44px touch targets.
  5. Playwright E2E tests verify the end-to-end ingestion and display of a message. No mock data; test against the live Docker stack.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
