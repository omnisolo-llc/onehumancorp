issue_title: "Architecture & Implementation: Native Rust Omnichannel Chat & WhatsApp Integration"
issue_description: |
  # Native Rust Omnichannel Chat System & WhatsApp Integration

  ## Problem Statement
  OneHumanCorp (OHC) is replacing a legacy external dependency with a highly-performant, multi-tenant Native Rust Omnichannel Chat System. Non-technical owners (like Maya the Baker or Carlos the Handyman) currently suffer from fragmented communication channels (Instagram DMs, WhatsApp, SMS, Web Chat). They need a unified Work Triage inbox where AI assistants like "The Ambassador" can proactively read contexts, generate AI-drafted replies, and offer an "Approve & Send" workflow—all directly from a 375px mobile UI, with zero configuration.

  ## Research Report
  - **Source Audit**: We audited external repositories. Key capabilities identified for replication in Rust:
    - `Conversation` & `Message` models handling thread states (`open`, `snoozed`, `resolved`) and assigning entities (Agents vs Bot).
    - Channel Adapters: Specifically the WhatsApp Cloud API integration using embedded signup, webhook ingestion (`hub.verify_token`, `hub.challenge`), and message delivery via the Graph API (`v19.0`).
    - WebSocket streaming for real-time inbox updates.
  - **Meta WhatsApp Cloud API**:
    - Embedded Signup allows users to connect their business numbers effortlessly.
    - Webhooks stream `messages`, `statuses`, and optionally `calls`.
    - No need for complex local client orchestration; Meta hosts the API infrastructure.
  - **The OHC Opportunity**: Integrating chat natively into `onehumancorp/mono` guarantees strict row-level security (`tenant_id`), Zero Trust (SPIFFE/SPIRE), and seamless handoffs to KAIROS orchestrator for AI triage without network hops or syncing external databases.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp / Web Chat] -->|Webhooks / WebSockets| B(OHC Omnichannel Gateway)
      B --> C{Event Router & Rate Limiter}
      C -->|Persist| D[(PostgreSQL Central Ledger)]
      D --> E[KAIROS Teammate Mesh]
      E --> F[The Ambassador Agent]
      F -->|RAG Context Lookup| G[(Vector DB / Memory)]
      F -->|Draft Reply| H[Shared Task List]
      H --> I[Flutter Mobile UI 375px]
      I -->|Owner Approves| J[WhatsApp Cloud API Client]
      J --> A
  ```

  ### Data Model & Invariants
  - **Conversation**: Links to `tenant_id`, `customer_id`, `channel_id`, `status` (`pending`, `open`, `snoozed`, `resolved`).
  - **Message**: Links to `conversation_id`, `tenant_id`, `sender_type` (Customer, Owner, AI_Agent), `content`, `external_id`.
  - **ChannelConfiguration**: Stores OAuth tokens, `phone_number_id`, `waba_id`, and provider type (e.g., `whatsapp_cloud`).
  - **Invariants**: Strict RLS via `tenant_id` on all tables. Distributed locks via Redis (e.g., `ohc:lock:{tenant_id}:message_dedup:{external_id}`) to prevent processing the same webhook twice.

  ### Mobile UX Flow (375px First)
  - **Feed View**: A unified chronological feed of active conversations prioritizing pending tasks (e.g., unread messages or drafted AI replies waiting for approval).
  - **Detail View**: Translucent Glass styling. Top half shows customer context (past purchases). Bottom half shows the conversation thread and a floating action area with a 1-tap "Approve & Send" button if the AI has drafted a response.
  - **Touch Targets**: Minimum 44x44px for all actionable elements.

  ### AI Agent Integration
  - **Work Triage / The Ambassador**: Automatically triggers on incoming `Message` events via the KAIROS mesh. It queries the user's inventory/FAQ, generates a reply, and updates the `Conversation` state to `pending_approval`.

  ## Implementation Prompt
  **User-Facing Outcome:** An owner connects their WhatsApp account via embedded signup. When a customer messages them, the KAIROS Ambassador agent instantly drafts an accurate reply based on business context. The owner sees a notification, taps "Approve" on their phone, and the message is sent.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Build the Rust `WhatsAppCloudClient` and webhook handlers capable of receiving Meta's `hub.challenge` and incoming message JSON payloads.
  2. Implement the `Conversation` and `Message` PostgreSQL schema ensuring row-level security (`tenant_id`).
  3. Ensure idempotency using Redis locks to handle duplicate Meta webhooks gracefully.
  4. Connect the incoming message pipeline to the KAIROS Teammate Mesh to trigger KAIROS agents.
  5. Playwright E2E Test: Simulate an incoming WhatsApp webhook, verify the conversation is created in the DB, and ensure the 375px UI displays the new thread correctly without mock data.
  6. 100% Backend Unit Test Coverage for the new Rust modules.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
