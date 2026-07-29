issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  Small business owners like Carlos and Maya suffer from managing unlinked customer communications across Instagram, WhatsApp, SMS, and Email. Previously, OHC relied on an external third-party Chatwoot integration. Chatwoot as an external service is now 100% RETIRED. OHC must implement a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust to handle this, guaranteeing Zero-Trust multi-tenant isolation, real-time sync, and native AI integration (The Ambassador Agent).

  # Research Report
  **Findings from Chatwoot Audit (`https://github.com/chatwoot/chatwoot`):**
  - **Data Models:** Chatwoot uses `Account` (Tenant), `Contact` (Customer), `Inbox`, `Conversation`, and `Message` models. Each inbox maps to a specific `Channel` (e.g., Instagram, WhatsApp, Email, Web Widget).
  - **Omnichannel Architecture:** Chatwoot uses polymorphic associations for channel providers (e.g., `Channel::Email`, `Channel::Whatsapp`) feeding into unified `Inbox` and `Conversation` models.
  - **Real-Time Messaging:** Rely on WebSockets (ActionCable) for real-time messaging, pushing events to the frontend via pub/sub.
  - **OHC Implementation Gap:** OHC requires this architecture natively in Rust inside `onehumancorp/mono`. We need matching Rust microservices, crates, and database schemas with strict row-level security (`tenant_id`).

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram / WhatsApp / Email Webhooks] --> B(Ingress API / Channel Adapters)
      B --> C{Rust Chat Service}
      C --> D[(PostgreSQL Unified Models)]
      C --> E[Redis Pub/Sub]
      E --> F(WebSocket / CRDT Sync Engine)
      F --> G[Flutter PWA 375px Client]
      C --> H[AI Event Mesh]
      H --> I(The Ambassador Agent)
      I -->|Proactive Drafts| C
  ```

  ### Core Data Models (PostgreSQL + RLS)
  - `omni_inboxes`: `id`, `tenant_id`, `name`, `channel_type`, `provider_config`
  - `omni_contacts`: `id`, `tenant_id`, `name`, `email`, `phone`, `avatar_url` (Unified Identity)
  - `omni_conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status`, `assignee_id`
  - `omni_messages`: `id`, `tenant_id`, `conversation_id`, `sender_type`, `sender_id`, `content`, `message_type` (incoming, outgoing, template)

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** The home screen features a unified inbox view. Messages from WhatsApp or Instagram look native but carry small provider icons.
  - **Conversation Screen:** WhatsApp-style chat interface (375px). Sticky text input at the bottom, native keyboard support.
  - **AI Suggestions:** Floating "AI Draft Available" button above the text input. 1-tap approve.
  - **Offline/Flaky Network:** Optimistic UI updates. Messages get a "clock" icon until the Rust backend acknowledges persistence and WebSocket confirms delivery.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event via Redis. For incoming customer messages, it retrieves the context (past orders, CRM data) and writes an `ai_draft` message into the conversation for the owner to approve.

  ### Zero Trust & Security
  - **Row-Level Security (RLS):** Every table MUST include `tenant_id` with `ENABLE ROW LEVEL SECURITY`. All DB queries automatically enforce the tenant context.
  - **SPIFFE/SPIRE:** The Channel Adapters authenticate to the core Rust Chat Service via mTLS using SPIFFE IDs.

  # Implementation Prompt
  **User-Facing Outcome:** A non-technical owner receives an Instagram DM. The notification pops up on their 375px OHC mobile app. The Ambassador Agent has already drafted a context-aware response based on the customer's prior orders. The owner clicks "Approve", and the Rust backend dispatches the message natively without relying on external Chatwoot servers.

  **CUJ & Acceptance Criteria:**
  1. Create the `omni_inboxes`, `omni_conversations`, `omni_contacts`, and `omni_messages` PostgreSQL schemas with RLS enforced.
  2. Implement the Rust gRPC/REST APIs for listing conversations and sending messages.
  3. Implement Channel Adapter traits in Rust that simulate an incoming webhook (e.g., from WhatsApp).
  4. Ensure a new message triggers an AI background job to draft a response.
  5. Provide Playwright E2E tests: A simulated owner logs in, navigates to the unified inbox, sees the simulated incoming message, views the AI draft, and taps "Approve" (verifying the outgoing API call).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
