issue_title: "Implement Custom Rust Omnichannel Chat System Parity with Chatwoot"
issue_description: |
  # Mission Queue Protocol: Custom Rust Omnichannel Chat System

  ## Problem Statement
  OHC requires a custom-built, native Rust omnichannel customer support chat system that provides 100% feature parity with Chatwoot, which has been entirely retired as an external dependency. A non-technical owner (e.g., Maya, Carlos, Priya) needs to seamlessly interact with customers via Instagram, WhatsApp, SMS, Web Widget, Email, etc., from a unified mobile-friendly interface without ever knowing the underlying system is a high-performance Rust service. Currently, the multi-tenant real-time chat architecture is missing from `onehumancorp/mono`.

  ## Research Report
  - **Context:** The `chatwoot` source code (`https://github.com/chatwoot/chatwoot`) has been audited. The key missing models/architectures in our stack include:
      - `conversations`, `messages`, `contacts`, `inboxes`, `channel_*` (adapters for Twilio, Meta, Line, etc.)
      - Real-time WebSocket multiplexing
      - Unified multi-tenant data model (Row-Level Security / `tenant_id` pattern)
  - **Competitor Insights:** Systems like Intercom and Shopify Inbox heavily rely on edge-cached WebSocket layers and unified contact ledgers. Our system needs to replicate the `app/models/channel/*` patterns natively in Rust, backing them by PostgreSQL with strong `tenant_id` isolation, integrated into our existing Go/Bazel backend ecosystem if applicable, or purely as new Rust microservices inside `src/server/ohc`.

  ## Design Doc
  ### Data Model & Invariants (Multi-Tenant)
  - **Inbox:** Logical container for channels.
  - **ChannelAdapter (Trait/Interface):** Common interface for `WebWidget`, `Instagram`, `WhatsApp`, `Email`, `SMS`.
  - **Conversation:** Links a `Contact`, an `Inbox`, and an `Assignee`.
  - **Message:** Real-time updates, polymorphic attachments.
  - **Multi-Tenancy:** Every table MUST include a `tenant_id` and utilize PostgreSQL RLS policies or strict query scoping to ensure tenant isolation.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : manages
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL_ADAPTER : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : holds
      MESSAGE }|--|| TENANT : belongs_to
  ```

  ### Mobile UX Flow (375px Target)
  - **Inbox List:** Unified feed mapping to the "Work Triage" screen.
  - **Conversation View:** Real-time message list with translucent glass header, native keyboard push-up, and quick AI action chips (e.g., "Draft Reply").
  - **Omnichannel Context:** Clear visual indicators (icons) showing the source channel (e.g., Instagram vs. SMS) for each message.

  ### AI Agent Integration
  - **Customer Assistant:** Listens to the `Conversation` queue, generates auto-responses (or drafts), and updates the UI in real time via the unified backend.
  - **Work Triage:** Analyzes incoming messages to tag priority and intent, pushing structured data to the `Conversation` context.

  ## Implementation Prompt
  Implementer Agent: Your objective is to build the core Rust backend architecture for OHC s native omnichannel chat system, replacing Chatwoot.
  1. Define the Sea-ORM or SQLx data entities for `Inbox`, `Conversation`, `Message`, `Contact`, and at least two channel configurations (e.g., `WebWidget`, `Email`) inside `src/server/ohc/domain/chat`.
  2. Implement strict multi-tenant isolation utilizing `tenant_id`.
  3. Create the real-time WebSocket service module to handle message ingestion and broadcast.
  4. Build the core GRPC/REST API endpoints necessary to load conversations and send messages from the Flutter/PWA client.
  5. Ensure 100% unit test coverage for the new models and services.
  6. All code must reside within `src/server/ohc` or the designated Rust service boundaries in the monorepo.
  Do not prescribe exact DDL or specific library wiring here; use your judgment to integrate cleanly with the existing Bazel and Rust ecosystem (Tokio, Tonic, Axum, SeaORM as configured in `Cargo.toml`).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
