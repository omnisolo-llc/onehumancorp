issue_title: "[Platform] Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OmniSolo currently relies heavily on integrating third-party solutions for customer communications. The external dependency on Chatwoot restricts complete multi-tenant control, complicates zero-trust enforcement via SPIFFE/SPIRE, and adds latency and points of failure. Non-technical owner/operators (e.g. Maya the Baker handling IG DMs, Carlos the Handyman handling service inquiries) need a unified, instant, zero-configuration communications inbox directly inside their OmniSolo desktop/mobile application, fully owned by their tenant.

  ## Research Report
  Based on the source code of Chatwoot (https://github.com/chatwoot/chatwoot) and industry research into omnichannel communications architectures, a unified inbox requires:
  1.  **Multi-channel Adapters**: Pluggable interfaces to ingest messages from Instagram, WhatsApp, Email, Web Widget, etc. Chatwoot models this beautifully with the `Channel` polymorphic concern.
  2.  **Conversational Data Model**: A flexible model to store Messages, Conversations, Contacts, and Inboxes while maintaining strict tenant isolation. From auditing Chatwoot's models (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`), we see a robust pattern of tying `conversation_id`, `inbox_id`, and `account_id` (which we will map to `tenant_id`) directly to messages for fast indexing and RLS.
  3.  **Real-time WebSocket Transport**: Instant bidirectional delivery of events and messages to the connected UI clients (Flutter/Tauri).
  4.  **Agent & Automations Engine**: The ability for AI agents (or rule-based macros) to intercept, draft replies, or auto-respond based on OHC's internal `kairos` state machine.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : manages
      Tenant ||--o{ ChannelAdapter : configures
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : has
      ChannelAdapter ||--o{ Inbox : feeds
  ```

  ### Data Model & Invariants (Rust / Postgres)
  *   `Tenant`: Root isolation boundary.
  *   `Inbox`: A collection point for a specific channel (e.g., "Maya's IG DM Inbox").
  *   `Contact`: The external user communicating with the Tenant.
  *   `Conversation`: A session of messages between a Contact and an Inbox.
  *   `Message`: Individual message payloads (Text, Image, Interactive).
  *   `ChannelAdapter`: The configuration and state for connecting to an external network (IG, WhatsApp).

  **Invariants**: Every table must include `tenant_id`. Row-Level Security (RLS) must be enabled in Postgres and enforced by the Rust API layer.

  ### Real-time Architecture
  1.  **Ingestion**: Webhooks from external networks hit the Rust Axum API.
  2.  **Storage**: Messages are committed to Postgres.
  3.  **Pub/Sub**: Insert events trigger notifications via PostgreSQL `LISTEN/NOTIFY` (or Valkey PubSub).
  4.  **Delivery**: The Rust server maintains active WebSocket connections with Tauri/Flutter clients and pushes real-time updates.

  ### Mobile-First UX (375px)
  *   **Inbox List**: Simple, swipeable list of recent conversations ordered by last message or urgency.
  *   **Conversation View**: Standard chat UI with native mobile keyboard support. Input area includes quick actions for AI drafts ("Ask Agent to reply").
  *   **Translucent Glass Design**: The UI must use the OHC Premium Token library, featuring blurred translucent materials and clean typography to ensure the interface doesn't feel like a legacy CRM.

  ### AI Integration Points
  *   **Triage**: New incoming messages trigger the `Work Triage` AI to categorize the intent (e.g., Support, Sales, Complaint).
  *   **Drafting**: The `Customer & Relationship Assistant` automatically generates draft replies for the owner to review and send.

  ## Implementation Prompt
  Implement the foundation of the native Rust Omnichannel Chat system in the `src/server/services/chat` directory.
  1.  Define the gRPC Protobuf definitions for `ChatService` (ListConversations, GetMessages, SendMessage).
  2.  Implement the database schema and migrations for `inboxes`, `conversations`, `contacts`, and `messages`, ensuring `tenant_id` RLS is applied.
  3.  Create the Axum WebSocket handler for real-time message delivery to clients.
  4.  Integrate the `Work Triage` AI to analyze new messages upon ingestion.
  Ensure all new code has 100% unit test coverage and add a Playwright E2E test verifying a user can send and receive a message in the UI.

  ## Estimated Scope
  **Large**

  ## Actionable Steps for Engineering Swarm
  *   **Phase 1**: Database schema & Protobuf definitions.
  *   **Phase 2**: Rust API & WebSocket implementation.
  *   **Phase 3**: Tauri/Flutter UI implementation (Mobile-first 375px).
  *   **Phase 4**: AI Assistant integrations.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
