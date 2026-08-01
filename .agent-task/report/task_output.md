issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System (Chatwoot Replacement)

  **Problem Statement**:
  Currently, OHC lacks a native, high-performance omnichannel inbox. Relying on an external service like Chatwoot breaks our multi-tenant Zero-Trust security model, complicates deployments, and forces our non-technical owner/operators (like Maya the baker and Carlos the handyman) to depend on external tooling for critical customer communications. They need a seamless, built-in assistant that consolidates Instagram DMs, WhatsApp, email, and web chat into a single prioritized queue, backed by AI agents that can draft replies automatically.

  **Research Report**:
  - **Competitor Analysis**: Tools like Shopify Inbox, WeCom, and DingTalk provide native unified inboxes, reducing friction.
  - **Chatwoot Source Code Benchmarking**:
    - Chatwoot's architecture relies heavily on separate channel adapters (WhatsApp, Email, FB Messenger), a central `Conversation` and `Message` model, and WebSockets for real-time updates.
    - Key entities to replicate: `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`.
  - **Architectural Gap**: OHC needs a Rust-based, multi-tenant gRPC/REST chat microservice that supports these same real-time streaming capabilities without the overhead of maintaining Ruby/Rails (Chatwoot's stack).

  **Design Doc**:
  - **Architecture**:
    - PostgreSQL for persistence with strict Row-Level Security (`tenant_id` on all rows).
    - Redis for Pub/Sub WebSocket event routing.
    - Rust native microservice handling connection state and chat APIs.
  - **Mobile UX Flow (375px first)**:
    - **Inbox List**: Tabbed view (All, Unread, Action Needed). Large touch targets (44x44px min). Each row shows customer avatar, preview, and an "AI Drafted" badge if applicable. Translucent Glass materials on iOS.
    - **Chat Thread**: Native mobile chat UI. Real-time updates. A sticky bottom input bar with "Send" and "AI Suggestion" actions.
  - **AI Agent Integration**:
    - The Customer Assistant department automatically listens to new `Message` inserts via PostgreSQL `SKIP LOCKED` queues.
    - Drafts are attached to the `Conversation` with a `status=draft` for owner approval.
  - **Key Design Decisions**:
    - Use Rust for the high-performance chat backend.
    - Enforce PostgreSQL row-level security (`tenant_id`) on every table.
    - WebSockets (or SSE) for real-time mobile/PWA delivery. No third-party Chatwoot dependency.

  **Implementation Prompt**:
  As the Implementer agent, build the backend Rust microservice and the corresponding frontend UI for the native OHC Omnichannel Inbox.
  - **CUJ**: Maya receives an Instagram DM. It appears instantly in her OHC Inbox. She taps the notification, sees the AI-drafted reply ("Yes, we do vegan cakes!"), and taps "Approve & Send".
  - **Acceptance Criteria**:
    1. Rust backend handles generic `Inbox`, `Conversation`, and `Message` CRUD.
    2. Multi-tenant RLS is enforced on all database interactions.
    3. UI is fully responsive, targeting 375px width gracefully, passing the "grandmother test".
    4. Provide full Playwright E2E test coverage verifying the complete flow of receiving and replying to a message. No UI mock data.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
