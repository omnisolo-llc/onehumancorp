issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems like legacy external dependency for customer communication. As per our architectural vision, we need to retire external legacy external dependency dependencies and build a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust. This enables deep integration with OHC AI agents (Operations, CS, Sales) and strict row-level security per tenant.

  ## Research Report
  - We audited the open-source legacy external dependency Ruby on Rails codebase (https://github.com/legacy_external_dependency/legacy_external_dependency) focusing on data models (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`).
  - Core entities include Conversation, Message, Contact, Inbox, ChannelAdapter.
  - legacy external dependency handles multiple channel types (WhatsApp, Web Widget, Email) via channel-specific tables or polymorphism.
  - Native Rust implementation needs:
    - High-performance Async runtime (Tokio)
    - WebSocket handling (for Web Widget)
    - Webhook receivers (for WhatsApp/Meta)
    - Multi-tenant data models matching legacy external dependency capabilities but utilizing PostgreSQL RLS.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client/Customer] -->|Web Widget WS/HTTP| B[OHC API Gateway]
      C[WhatsApp/Meta] -->|Webhooks| B
      B --> D[Rust Chat Service]
      D --> E[(PostgreSQL - RLS Enabled)]
      D --> F[Redis - Distributed Locks/PubSub]
      D --> G[AI Agent Triage Queue]
      G --> H[Customer Assistant Agent]
      H --> D
  ```

  ### Data Models (Rust / Diesel or SQLx)
  - **Inbox**: Configured channels per tenant.
  - **Conversation**: Links a contact to an inbox. Tracks status (open, resolved, snoozed), assignee, SLA.
  - **Message**: Individual message payload. Polymorphic `content_type` (text, attachment, template). Tracks `sender_type` (Contact, Agent, Bot) and `message_type` (incoming, outgoing, activity).
  - **Contact**: Customer details.

  ### Mobile UX Flow (375px First)
  - The Owner sees a unified "Inbox" tab.
  - Unread indicators bubble up across channels (WhatsApp, Web).
  - Tapping a thread opens the conversation view: messages on left (customer), right (owner/agent), and a distinct background for AI-drafted responses pending approval.
  - Sticky input bar at the bottom with quick replies and AI-draft suggestions.
  - Slide-out or collapsible right drawer for Contact Details and past orders (requires OHC ecosystem integration).

  ### AI Agent Integration
  - Incoming messages trigger an event on the AI Job Queue (PostgreSQL SKIP LOCKED).
  - **Customer Assistant Agent** consumes the event, reads context, and drafts a reply or automatically replies (if authorized).
  - Drafts are saved as `Message` records with a specific `status` (e.g., `draft`) pending Owner approval.

  ### Multi-Tenancy & Zero Trust
  - All database tables MUST include `tenant_id` and have `ENABLE ROW LEVEL SECURITY` configured.
  - Service authenticates via SPIFFE/SPIRE internally; APIs rely on JWTs with embedded `tenant_id`.

  ## Implementation Prompt
  **Goal:** Implement the foundation of the native Rust Omnichannel Chat system, replacing legacy external dependency.

  **Tasks for Implementer:**
  1. Define the SQL schema migrations for `inboxes`, `conversations`, `messages`, and `contacts` including `tenant_id` and RLS policies.
  2. Implement the core Rust gRPC/REST API layer to support basic CRUD for these entities.
  3. Implement the WebSocket handler for incoming Web Widget connections, supporting real-time message broadcasting within a conversation.
  4. Ensure strict tenant isolation in all queries.
  5. Add unit and integration tests (using the repository's test framework) covering message flow and tenant isolation.

  **Acceptance Criteria:**
  - Database schema includes necessary tables with RLS.
  - A test client can connect via WebSocket, send a message, and receive an echo/broadcast.
  - API endpoints for fetching conversations and messages work and enforce tenant boundaries.
  - 100% unit test coverage for new Rust code.
  - All existing `main` tests pass (`bazel test //...`).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
