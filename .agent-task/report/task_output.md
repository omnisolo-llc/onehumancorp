issue_title: "Migrate Legacy Omnichannel Inbox to Native Rust Architecture"
issue_description: |
  ## Problem Statement
  The OHC platform currently lacks a native omnichannel unified inbox. As per the strict mandate, external third-party legacy messaging services are 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust. This gap severely limits operators like Maya (baker) or Nora (agency principal) who need to seamlessly view, reply to, and track Instagram DMs, WhatsApp messages, and website widget chats within their single OHC interface without relying on external dependencies.

  ## Research Report
  - Analyzed the legacy codebase including its core models: `account`, `inbox`, `channel`, `conversation`, `message`, `contact`, and various channel integrations (`whatsapp.rb`, `web_widget.rb`, etc.).
  - The legacy multi-tenancy relies on `account_id`, whereas OHC requires strict `tenant_id` Row Level Security (RLS) in PostgreSQL.
  - The legacy service uses Rails/ActionCable; OHC will utilize native Rust (e.g., Tokio, Axum/Tonic, async WebSockets) for lightning-fast real-time messaging.
  - Existing industry standards (Shopify Inbox, Meta Business Suite) combine multiple sources into one chronological event feed with agent (AI or Human) context.

  ## Design Doc
  ### Data Model & Invariants (Rust & PostgreSQL)
  - `Tenant` (Strict isolation boundary via `tenant_id`)
  - `Inbox`: Aggregates conversations for a specific purpose or team.
  - `Channel`: The integration adapter (e.g., `ChannelWhatsapp`, `ChannelWebWidget`, `ChannelInstagram`).
  - `Contact`: Represents the customer across channels.
  - `Conversation`: A threaded discussion tied to a Contact and an Inbox.
  - `Message`: Individual message payloads (text, images, attachments) within a Conversation.
  - **Invariants**: Every entity MUST have a `tenant_id`. All DB queries MUST be scoped to the `tenant_id`.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CHANNEL : owns
      INBOX ||--o{ CONVERSATION : contains
      CHANNEL ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Mobile UX Flow (375px first)
  1. **Omni-Inbox View**: The owner opens the app. The "Inbox" tab shows a unified list of active conversations, badged by channel (WhatsApp icon, Web icon).
  2. **Conversation View**: Tapping a thread opens a chronological chat view. The UI clearly distinguishes AI agent drafts from customer messages and owner manual replies.
  3. **Reply Action**: The owner taps the input field, native keyboard opens. They can hit "Send" to reply instantly, which routes back through the native Rust backend to the correct external channel.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Monitors new `Message` inserts. Summarizes intent and urgency.
  - **Customer Assistant Agent**: Subscribes to new `Conversation` creations to automatically draft contextual replies based on tenant knowledge base.

  ## Implementation Prompt
  Implement the core Rust data structures, PostgreSQL schema (with RLS), and channel adapters (starting with Web Widget and WhatsApp) to support the unified inbox. Expose gRPC/REST APIs for the Flutter frontend to list inboxes, view conversations, and send messages. Ensure all logic strictly adheres to tenant isolation requirements. Add corresponding Playwright E2E tests verifying a message sent via a mock webhook appears in the unified inbox UI and an owner reply successfully triggers the outbound adapter.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
