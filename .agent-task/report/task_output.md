issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Research Report: Custom Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust. The current lack of this core system forces reliance on third-party solutions, which violates the architectural constraint to eliminate external dependencies like Chatwoot and provide an owner/operator-first, zero-trust, integrated communication hub.

  ## Research & Competitive Analysis
  - **Chatwoot Source Code Audit**: Benchmarked against Chatwoot's architecture (`https://github.com/chatwoot/chatwoot`), the core requirements for an omnichannel system are:
    1. Unified Inbox for managing messages across multiple channels (Email, Web Widget, API, etc.).
    2. Multi-tenant isolation for different workspaces/accounts.
    3. Conversations and Messages data models linked to Contacts.
    4. Real-time updates via WebSockets (or gRPC streaming).
  - **OHC Architecture Alignment**:
    - The new implementation must leverage Rust (Tonic gRPC + Axum for web endpoints) within the `onehumancorp/mono` repo.
    - PostgreSQL for persistence, with strict row-level security for tenant isolation (`tenant_id`).
    - Valkey/Redis for PubSub and real-time state.

  ## Architecture & Design

  ### Data Model Invariants (PostgreSQL)
  - `ohc_chat_inboxes`: `id`, `tenant_id`, `name`, `channel_type` (e.g., 'web_widget', 'email'), `created_at`.
  - `ohc_chat_contacts`: `id`, `tenant_id`, `name`, `email`, `phone`, `identifier`.
  - `ohc_chat_conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, pending), `created_at`.
  - `ohc_chat_messages`: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, agent, bot), `sender_id`, `content`, `created_at`.

  ### Component Diagram
  ```mermaid
  graph TD
      Client[Web Widget / PWA] -->|HTTPS/WSS| Axum[Axum Gateway]
      Axum --> Tonic[gRPC Chat Service]
      Tonic --> DB[(PostgreSQL)]
      Tonic --> Redis[(Valkey Pub/Sub)]
      Redis -.->|Event| Axum
      Axum -.->|WSS| Client
  ```

  ### Mobile UX Flow (375px)
  1. The user (Maya, Carlos) opens the OHC app.
  2. "Work Triage" shows active conversations in a simple list view.
  3. Tapping a conversation opens a standard chat interface with a reply box anchored to the bottom.
  4. Real-time indicators show when a message is received or when the AI Agent drafts a reply.

  ### AI Integration
  - **Customer & Relationship Assistant**: The AI Agent subscribes to the Valkey Pub/Sub feed for `ohc_chat_messages`.
  - When a new message arrives, the agent drafts a response and saves it as a pending/draft message in `ohc_chat_messages`.
  - The owner sees the draft and can tap "Approve & Send" or edit it.

  ## Implementation Prompt
  Implement the core backend data models, gRPC service definitions, and Rust handlers for the native OHC Omnichannel Chat System.
  1. Define the Protocol Buffers (`src/proto/chat.proto`) for Inbox, Conversation, Contact, and Message management.
  2. Implement the database schema migrations for the chat entities ensuring `tenant_id` is present on all tables.
  3. Create the Rust gRPC server implementation in `src/server/services/chat/` to handle CRUD operations.
  4. Add unit and E2E tests covering the creation of an inbox, receiving a message, and reading the conversation.

  Acceptance Criteria:
  - All new gRPC endpoints are tenant-isolated.
  - Test coverage is 100% for the new Rust modules.
  - Playwright E2E tests verify that a message can be created and retrieved via the API/UI.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
