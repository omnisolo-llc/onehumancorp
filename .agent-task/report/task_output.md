issue_title: "Implement Native Rust Omnichannel Inbox to Replace Legacy Third-Party Chat"
issue_description: |
  # Native Rust Omnichannel Inbox Replacement

  **Problem Statement:**
  OneHumanCorp (OHC) currently relies on a legacy third-party system for omnichannel customer communication. However, as an external service, it breaks our multi-tenant isolation guarantees, adds latency to our core workflows, and prevents deep integration with our AI assistant engine. We need to retire it completely and implement a high-performance, native Rust omnichannel inbox system within the OHC mono-repo.

  **Research Report:**
  Based on an audit of the external source code and leading competitors (Zendesk, Intercom, Slack), a modern omnichannel inbox requires:
  - **Core Entities:** `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`.
  - **Real-time Communication:** WebSockets for instant message delivery and typing indicators.
  - **Multi-tenancy:** Strict data isolation per tenant (Row-Level Security in PostgreSQL).
  - **AI Integration:** Seamless routing of conversations to AI agents (like the Customer & Relationship Assistant) for automated drafting and context summarization.

  **Design Doc:**

  *Architecture Overview:*
  1.  **Data Layer (Rust/PostgreSQL):**
      -   Implement core models mimicking the legacy system's capabilities but optimized for OHC's multi-tenant architecture.
      -   Entities: `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter` (Email, Web Widget, SMS, etc.).
  2.  **Service Layer (Rust/gRPC):**
      -   Create microservices/crates for managing inboxes, routing messages, and handling channel webhooks.
      -   Implement a WebSocket gateway for real-time client updates.
  3.  **Frontend (Flutter/PWA):**
      -   Build a unified "Work Triage" inbox UI using the OHC Premium Token library (macOS Translucent Glass + UniFi modular layouts).
      -   Ensure flawless 375px mobile responsiveness.

  *Mobile UX Flow (375px First):*
  -   **Home:** A unified feed showing urgent messages, new bookings, and pending AI drafts.
  -   **Inbox View:** A clean list of conversations, filterable by channel or status.
  -   **Conversation View:** A chat interface with native mobile keyboard support, quick AI reply suggestions, and customer context visible on a swipe-over drawer.

  *AI Agent Integration:*
  -   The **Customer Assistant** listens to new messages via an internal event bus (NATS).
  -   It drafts replies, which are stored with a `pending_approval` status, visible to the owner in the Work Triage feed.
  -   The **Work Triage** agent categorizes incoming messages and links them to existing customer profiles.

  **Implementation Prompt:**
  As an Implementer agent, build the core data models and service layer for the native Rust omnichannel inbox.
  -   Implement the PostgreSQL schema for `Inbox`, `Conversation`, and `Message` with strict multi-tenant RLS.
  -   Create the Rust service layer for creating inboxes and routing messages.
  -   Ensure the implementation supports future WebSocket integration and channel adapters.
  -   DO NOT implement specific external channels (like WhatsApp) yet; focus on the core internal inbox engine.
  -   Ensure 100% unit test coverage.

  **Priority:** P0 (Critical for platform independence)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
