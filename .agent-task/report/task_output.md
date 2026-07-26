issue_title: "Implement Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) relies on AI assistants to coordinate customer relationships and triage messages. The prior architecture depended on an external Chatwoot installation, which breaks our multi-tenant, Zero-Trust model and fragments the operator's experience. We need a native Rust omnichannel communication engine built directly into OHC to serve operators like Maya (handling Instagram DMs) and Carlos (handling SMS leads) from their 375px mobile screens, natively integrated with our AI agents.

  ## Research Report
  - **Market Context:** Small business operators require a unified inbox. Disparate systems (e.g., separate Whatsapp, Instagram, and web chat apps) lead to missed leads.
  - **Chatwoot Audit:** Audited `chatwoot/chatwoot` source. Key models: `Account` (Tenant), `Inbox`, `Channel::*`, `Conversation`, `Message`, `Contact`. Their architecture relies heavily on Postgres, Webhooks, and ActionCable (WebSockets).
  - **Competitive Landscape:** Platforms like Shopify Inbox, Zendesk, and Front have proven the "Unified Inbox" model, but lack deep AI-agent first integration.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    erDiagram
      ACCOUNT ||--o{ INBOX : "has many"
      INBOX ||--o{ CONVERSATION : "contains"
      ACCOUNT ||--o{ CONTACT : "has many"
      CONVERSATION }|--|| CONTACT : "belongs to"
      CONVERSATION ||--o{ MESSAGE : "contains"
      ACCOUNT ||--o{ MESSAGE : "owns"
    ```
  - **Architecture:**
    - Replicate the `Inbox`, `Conversation`, `Message`, and `Contact` models natively in Rust (`src/server/ohc/inbox`, etc.).
    - Use Tokio / Axum for WebSockets to handle real-time delivery to the Flutter/Tauri UI.
    - Implement `ChannelAdapters` (traits in Rust) to handle Web Widget, WhatsApp, Instagram, and Email.
    - AI Agent Integration: AI agents (Customer Assistant) will subscribe to `MessageCreated` events via the background queue and draft replies as "Agent Bot" participants directly into the `Conversation` state.
  - **Mobile UX Flow (375px):**
    - "Unified Inbox" tab: A clean, Apple/Ubiquiti style list of open conversations.
    - Conversation View: Native mobile chat bubble layout, touch targets > 44px, offline-tolerant optimistic UI for message sending.
    - AI Draft UI: Translucent glass pane suggesting a reply ("Customer Assistant drafted a reply... [Approve/Edit/Discard]").
  - **Multi-Tenancy:**
    - Row-level security / strict `tenant_id` filtering on all `Conversation` and `Message` entities.
    - `ohc:lock:{tenant_id}:conversation:{id}` for cross-agent coordination during drafting.

  ## Implementation Prompt
  **Goal:** Implement the backend Rust API and data models for a unified omnichannel inbox that replaces Chatwoot.
  **CUJ:** A non-technical owner receives a message via the Web Widget, the Customer Assistant drafts a reply, and the owner approves the reply on their mobile phone.
  **Acceptance Criteria:**
  - Create the Postgres schema and Rust models for `Inbox`, `Conversation`, `Message`, and `Contact`, strictly isolated by `tenant_id`.
  - Implement basic REST/gRPC endpoints to list conversations, fetch messages, and send a message.
  - Implement a basic WebSocket hub in Axum to broadcast new messages to connected clients.
  - No external Chatwoot dependencies.
  - E2E Playwright test covering the web widget message reception and owner reply flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
