issue_title: "[Architecture] Native Rust Omnichannel Chat System Replication (the legacy chat system Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OHC is retiring the external the legacy chat system dependency to consolidate infrastructure, reduce latency, and improve multi-tenant isolation. Maya, Carlos, and Priya need a lightning-fast, unified inbox that seamlessly captures messages from WhatsApp, Instagram DMs, web widgets, and email, without the overhead and synchronization complexity of an external third-party service. This requires a native Rust replacement built inside `onehumancorp/mono` that achieves feature parity with the legacy chat system's core messaging engine while adhering strictly to OHC's Zero-Trust multi-tenant isolation.

  ## Research Report
  - **the legacy chat system Source Code Audit:**
    - Analyzed the legacy chat system's core data models (`Conversation`, `Message`, `Inbox`, `Contact`).
    - **Inboxes** encapsulate different channel configurations (e.g., WhatsApp, Email, Web Widget) and link directly to a specific `account_id` (tenant).
    - **Conversations** aggregate messages for a contact within an inbox, tracking status (`open`, `resolved`, etc.), assignees, and SLA metrics.
    - **Messages** store the actual content, content type, sender references, and attachments.
  - **Competitive Analysis:**
    - Shopify Inbox and Zendesk provide unified messaging but often lack the deep operational integrations required by SMB owners (like immediate booking or quoting from a DM).
    - OHC's native implementation will allow AI Agent Departments (Customer Service, Operations) to seamlessly hook into the message stream, auto-draft replies, and trigger background tasks using local Rust traits and PostgreSQL `SKIP LOCKED` queues without network hops.

  ## Design Doc
  ### Data Model & Invariants (PostgreSQL + RLS)
  - `Tenant` isolation is strictly enforced via `tenant_id` on all tables, utilizing PostgreSQL Row Level Security (RLS).
  - **Core Entities:**
    - `Inbox`: Configuration for a specific channel (e.g., WhatsApp, Web Widget). Fields: `id`, `tenant_id`, `name`, `channel_type`, `config (JSONB)`.
    - `Contact`: The customer/end-user. Fields: `id`, `tenant_id`, `name`, `identifier` (e.g., phone number, email).
    - `Conversation`: The thread. Fields: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, snoozed, resolved), `assignee_id`.
    - `Message`: Individual messages. Fields: `id`, `tenant_id`, `conversation_id`, `sender_type` (user, contact, agent_bot), `content`, `content_type`, `external_source_id`.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Mobile-First UX Flow (375px)
  - **Home Dashboard:** The owner (e.g., Maya) opens OHC. A unified "Urgent DMs" card surfaces open conversations needing attention.
  - **Unified Inbox View:** Tapping the card opens a full-screen, native-feeling chat list view. Each row shows the contact's avatar, channel icon (WhatsApp, IG), snippet, and unread count.
  - **Conversation View:** Tapping a thread reveals the chat timeline. AI-drafted replies appear in a translucent glass container at the bottom, offering a 1-tap "Send" or "Edit" action.
  - **Performance:** WebSocket-based real-time updates ensure zero-refresh syncing. Messages are aggressively cached and lazy-loaded for low-bandwidth environments (Fatima).

  ### AI Agent Integration Points
  - **Work Triage:** A background Rust worker listens for `MessageCreated` events. It invokes the LLM (Gemini Pro) to classify intent (e.g., "quote request", "complaint").
  - **Customer Assistant:** Automatically drafts responses based on tenant context (e.g., checking Maya's availability for a cake delivery) and saves them as pending drafts in the `Message` table.

  ### Key Design Decisions and Why
  - **PostgreSQL RLS over Schema-per-Tenant:** RLS is natively supported by our stack and simplifies database migrations and connections, fitting our current tenant model best.
  - **Background Rust Workers over Webhooks:** Reduces network overhead and enables tighter integration with local AI traits and state.
  - **WebSocket over Long-Polling:** Critical for real-time mobile performance where battery and bandwidth are constrained.

  ## Implementation Prompt
  Implement the core Rust data models, PostgreSQL migrations (with RLS), and internal gRPC services for the Native Omnichannel Chat System within `onehumancorp/mono`.
  - Define the `Inbox`, `Conversation`, and `Message` entities mirroring the the legacy chat system audit, optimized for Rust and standardizing on `tenant_id` for isolation.
  - Create the foundational `ChatService` with endpoints for creating conversations, appending messages, and listing active threads.
  - Integrate WebSocket handlers for real-time delivery to the Flutter frontend.
  - Ensure all new logic is fully tested via unit tests and E2E Playwright tests covering a business owner receiving and viewing a new message.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
