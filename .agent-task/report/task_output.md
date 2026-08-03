issue_title: "Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  OHC currently relies on an external third-party integration (Chatwoot) for omnichannel messaging (Instagram, WhatsApp, SMS, Web). This creates a fractured architecture, complicates multi-tenant isolation, and prevents our AI agents (like The Ambassador) from having deeply integrated, low-latency access to the unified inbox. We need to retire the external Chatwoot dependency entirely and build a native, high-performance Rust omnichannel chat system within `onehumancorp/mono`.

  # Research Report
  **Findings from Chatwoot Source Code Audit:**
  Based on an audit of the `chatwoot/chatwoot` repository, the core data model revolves around a few key entities:
  - `Message`: Represents an individual message, polymorphic via `content_type` and `message_type` (incoming/outgoing). Contains a `jsonb` column for `content_attributes`.
  - `Conversation`: Links messages to a specific `contact_id`, `inbox_id`, and `assignee_id`. Tracks state like `status`, `last_activity_at`, and `snoozed_until`.
  - `Inbox`: Configuration for a specific channel (e.g., an Instagram page, a WhatsApp number, or a Web Widget).
  - `Channel Adapters`: Chatwoot uses separate models (e.g., `Channel::Instagram`, `Channel::Whatsapp`, `Channel::WebWidget`) that polymorphicly link to an `Inbox`.

  **OHC Architectural Gap:**
  To achieve parity, OHC needs a Rust-based real-time event pipeline (WebSockets) and a persistent data layer (PostgreSQL with Row Level Security for multi-tenancy) that maps these core concepts. The system must support bidirectional webhook ingestion (from Meta, Twilio, etc.) and unified dispatch.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[External Channels: IG, WA, SMS] -->|Webhooks| B(Rust Edge Ingestion Service)
      C[Web Chat Widget] <-->|WebSockets| D(Rust Realtime Gateway)
      B --> E{Omnichannel Event Router}
      D --> E
      E --> F[(PostgreSQL: Messages, Conversations, Inboxes)]
      E --> G[Agent Event Bus / AI Job Queue]
      G --> H[The Ambassador Agent]
      H -->|Drafts Reply / Auto-Responds| F
      H -->|Notifies UI| D
      F --> I[Owner Mobile App Feed 375px]
      I -->|Approves/Sends| D
      D --> J(Rust Dispatcher Service)
      J --> A
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed:** The owner sees unified conversation cards natively in the OHC app, indistinguishable from other tasks (like fulfilling an order).
  - **Interaction:** Tapping a conversation opens a native chat view. The AI-drafted reply is pre-filled in the text box or shown as a floating "Approve" card.
  - **Real-time:** The UI uses WebSockets to show typing indicators and instantly append new messages without polling.

  ### Data Model Invariants (Rust/SQLx)
  - Every table (`messages`, `conversations`, `inboxes`, `contacts`) MUST have a `tenant_id` column.
  - Row Level Security (RLS) policies MUST enforce tenant isolation on all queries.
  - `inboxes` will have an `adapter_type` enum (WebWidget, Instagram, WhatsApp, API) and adapter-specific credentials stored securely (Zero Trust/SPIFFE injected where possible, or encrypted in DB).

  ### AI Agent Integration Points
  - When a new message is inserted into the DB, a notification is sent to the Agent Event Bus.
  - The Ambassador Agent processes the conversation history, retrieves catalog context via RAG, and inserts a `Message` with `status = 'draft'`.
  - The owner's UI subscribes to conversation updates and immediately displays the drafted message for approval.

  # Implementation Prompt
  **User-Facing Outcome:** The business owner manages all customer communications (Instagram, Web, WhatsApp) natively inside the OHC mobile app. The UI is blisteringly fast due to the Rust backend, and AI agents seamlessly draft replies based on past customer history.
  **CUJ & Acceptance Criteria:**
  1. Setup a Rust gRPC/REST service for the Omnichannel Inbox.
  2. Define SQLx migrations for `inboxes`, `conversations`, `messages`, and `contacts` with strictly enforced `tenant_id` RLS.
  3. Implement a Webhook ingestion endpoint that normalizes payloads from a simulated external provider (e.g., a dummy WhatsApp webhook) into the unified schema.
  4. Implement a WebSocket gateway that broadcasts new messages to the authenticated tenant's frontend.
  5. Create Playwright E2E tests: A test script fires a webhook simulating a new customer message. The Playwright test verifies that the OHC frontend (running at 375px) receives the message via WebSocket and displays it in the unified inbox UI.
  6. **Important:** Remove all existing configuration, references, or dependencies related to external Chatwoot services from the codebase.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
