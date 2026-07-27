issue_title: "Native Rust Omnichannel Chat System - Architecture & Design"
issue_description: |
  # Native Rust Omnichannel Chat System - Architecture & Design

  ## Problem Statement
  OneHumanCorp currently lacks a native omnichannel inbox for our personas (Maya, Carlos, Priya, Leo, Fatima). Following the mandate to retire external third-party services like Chatwoot, we need to build a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside our monolith to handle customer interactions from various channels (Instagram DMs, WhatsApp, SMS, Web Widget, Email, etc.). This ensures better integration with our AI agents, tighter data isolation, lower latency, and full control over the mobile-first UX.

  ## Research Report
  - **Market Research**: Modern SMBs expect a unified inbox. Platforms like Shopify, Wix, and specialized tools like Chatwoot and Intercom provide this. Our personas need to respond to inquiries efficiently, and AI needs to draft these replies.
  - **Codebase Audit**: We have `src/proto/inbox.proto` defining `OmniMessage` and `Conversation`, but we need a complete backend implementation and data model in Rust, plus the channel adapters and AI integration. We are mandated to inspect the Chatwoot repository to ensure feature parity and understand standard models (Inbox, Conversation, Message, Channel, Contact, Agent).
  - **Chatwoot Benchmarking**: Chatwoot’s architecture separates `Inbox` (the entry point for a specific channel), `Conversation` (the thread), `Message` (the individual interaction), `Contact` (the customer), and various `Channel::*` models (WhatsApp, Facebook, Twitter, WebWidget, etc.). OHC's implementation will use a Rust-native approach with gRPC APIs, row-level tenant isolation, and async processing.

  ## Design Doc

  ### Architecture
  We will introduce a new module `src/server/inbox` and expand `src/proto/inbox.proto`.

  #### Data Model (Entities)
  - `Inbox`: Represents a channel connection (e.g., "Maya's Instagram", "Carlos' SMS"). Fields: ID, Tenant ID, Name, Channel Type, Config (JSON).
  - `Contact`: Represents the customer interacting. Fields: ID, Tenant ID, Name, Email, Phone, Avatar.
  - `Conversation`: A thread between a Contact and an Inbox. Fields: ID, Tenant ID, Inbox ID, Contact ID, Status (Open, Snoozed, Resolved), Assignee ID, Created/Updated At.
  - `OmniMessage`: A single message. Fields: ID, Tenant ID, Conversation ID, Content, Message Type (Incoming, Outgoing, Note, Template), Sender Type (Contact, Agent, Bot), Sender ID, Channel Source ID.

  #### Multi-Tenancy & Isolation
  - All database tables will have a `tenant_id` column.
  - PostgreSQL Row Level Security (RLS) policies will enforce tenant isolation automatically for all queries.

  #### Channel Adapters
  - Implement a trait `ChannelAdapter` with methods `send_message`, `receive_message`, `verify_webhook`.
  - Initial adapters: Web Widget (WebSocket-based), Dummy/Local (for testing), and structure for future Webhook-based channels (WhatsApp/Instagram).

  #### AI Agent Integration
  - **Work Triage / Customer Assistant**: When an `OmniMessage` (Incoming) arrives, an async job is spawned. The AI agent analyzes the message, drafts a reply, and saves it to the database (or updates the message record with `draft_reply`), triggering a notification to the owner.

  #### Mobile-First UX Flow (375px)
  - **Inbox List**: A clean, unified list of conversations. Unread indicators.
  - **Conversation View**: Chat bubbles. Translucent glass sticky header with Contact info. Bottom input area with native keyboard support.
  - **AI Suggestions**: Just above the input area, a pill or card showing the AI-drafted reply with a one-tap "Send" or "Edit" action.

  ### Implementation Prompt
  **To the Implementer:**
  Implement the backend foundation for the Native Rust Omnichannel Inbox.
  1. Update `src/proto/inbox.proto` with comprehensive definitions for Inbox, Contact, Conversation, and OmniMessage matching the described data model. Generate Rust code.
  2. Implement the gRPC service `InboxService` in `src/server/inbox` covering CRUD operations for these entities.
  3. Create the database migrations (PostgreSQL) including RLS policies using `tenant_id`.
  4. Implement a foundational Webhook ingestion endpoint for external channels to push messages into the system.
  5. Ensure 100% unit test coverage for the service and integration tests covering the gRPC endpoints.
  6. Ensure all UI/E2E considerations (Playwright) are planned for the frontend implementation (which may be a separate task, but ensure backend provides necessary hooks).

  **Acceptance Criteria:**
  - gRPC server can create an Inbox, create a Contact, start a Conversation, and append Messages.
  - RLS successfully blocks cross-tenant data access in tests.
  - Test suite passes (`bazel test //...`).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
