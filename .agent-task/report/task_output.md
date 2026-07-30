issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System Research Report

  ## Problem Statement
  OHC previously relied on Chatwoot as an external third-party service for omnichannel customer support (WhatsApp, Web Chat, Instagram, Email, SMS). This external dependency is now 100% RETIRED as per the OHC Engineering Standards. We need to implement a native, high-performance, multi-tenant omnichannel chat engine in Rust directly inside `onehumancorp/mono` to achieve feature parity with Chatwoot, ensuring tight integration with our AI assistants and avoiding external service dependencies. This is critical for our owner/operator personas like Maya (home baker using IG DMs) and Carlos (field service using SMS/WhatsApp) to manage customer interactions directly within their unified OHC workspace.

  ## Research Report
  - **Goal**: Build a native Rust replacement for Chatwoot's omnichannel capabilities.
  - **Source Analysed**: Checked out the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture.
  - **Key Features to Replicate**:
    - Multi-channel support (Web widget, Email, API-based channels like Twilio SMS/WhatsApp, Meta Instagram/Messenger).
    - Unified Inbox/Conversations view.
    - Message routing and assignment.
    - Canned responses / Macros.
    - Webhook event system for integrations.
    - Real-time updates via WebSockets.
  - **Architecture Alignment**: The native solution will fit into the OHC multi-tenant SaaS architecture (Rust/gRPC/PostgreSQL/Redis), providing better performance and deeper integration with OHC's AI triage capabilities than an external Ruby-on-Rails application (Chatwoot).

  ## Design Doc
  - **Backend (Rust)**:
    - Create a new domain under `src/server/integrations/chat` or a dedicated `src/server/chat` module.
    - Define database schemas for `conversations`, `messages`, `contacts`, `inboxes`, and `channel_credentials` using PostgreSQL with tenant isolation.
    - Implement a WebSocket server (using `tokio-tungstenite` or `axum::ws`) for real-time delivery to the frontend.
    - Create adapters for external channels (e.g., Twilio, Meta) using existing integration structures.
    - Implement a routing engine to handle incoming webhooks from external channels and map them to unified `conversations`.
  - **Frontend (Flutter)**:
    - Develop a Unified Inbox view.
    - Implement real-time WebSocket connection to receive new messages and send updates.
    - Build a web chat widget component that can be embedded by users.

  ## Implementation Prompt
  Implement the core backend infrastructure for the native Rust omnichannel chat system.
  1. Define the PostgreSQL database schema for core entities (Contacts, Conversations, Messages, Inboxes) ensuring strict multi-tenant row-level security.
  2. Implement the gRPC API definitions for listing, creating, and updating these entities.
  3. Implement the Rust service layer for the APIs.
  4. Create a unified webhook ingest endpoint that can be extended with specific channel adapters later.
  5. **Acceptance Criteria**: The core data model is in place, and the API allows creating and querying conversations and messages for a specific tenant. Real-time WebSocket delivery and specific channel adapters (like WhatsApp) can follow in subsequent phases. The user-facing outcome is that the groundwork is laid for owners to view all messages in one place.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
