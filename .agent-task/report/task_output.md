issue_title: "Native Rust Omnichannel Chat System Implementation"
issue_description: |
  # Problem Statement

  Currently, OneHumanCorp (OHC) relies on external tools or incomplete internal implementations for handling omnichannel chat. The market research highlights the critical need for a unified inbox (Omnichannel Gateway) where business owners like Maya (the baker) or Carlos (the handyman) can seamlessly manage Instagram DMs, WhatsApp, SMS, and email all in one place. Relying on external services like Chatwoot is explicitly retired by our engineering standards. We need a robust, native Rust implementation for omnichannel chat that ensures multi-tenant isolation, real-time sync, and serves as the foundation for the Customer Success Agent (The Ambassador) to draft proactive replies.

  # Research Report

  **Findings & Competitive Analysis:**

  - **Chatwoot Source Audit:** We cloned and analyzed the Chatwoot source code (`https://github.com/chatwoot/chatwoot`). Key components include `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` models, alongside numerous channel adapters (Facebook, WhatsApp, SMS, Line, Telegram, Web Widget, Twitter, Instagram).
  - **OHC Architecture:** The current Rust service for chat (`src/server/services/chat/`) has basic models but lacks comprehensive webhook handling, real-time WebSocket broadcasting (event mesh), robust channel-specific adapters (API definitions for Facebook, Twilio, WhatsApp, etc.), and integration with our AI drafting agent.
  - **Security & Multi-Tenancy:** The new chat architecture must strictly enforce row-level security (RLS) in PostgreSQL, isolating `tenant_id` at every level.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[External Webhooks: IG, WhatsApp, SMS] -->|Webhook Adapter| B(Omnichannel Gateway Rust Service)
      B --> C{Identity Resolution Engine}
      C -->|Find/Create| D[ChatContact]
      B --> E[Message Persistence & RLS Check]
      E --> F[(PostgreSQL OHC Ledger)]
      E --> G[Redis Pub/Sub / Event Mesh]
      G --> H[WebSocket Server]
      H --> I[Mobile App 375px Client]
      G --> J[The Ambassador Agent]
      J -->|Draft Reply| K[Agent Draft Queue]
      K --> I
  ```

  ### Mobile UX Flow (375px First)

  - **Unified Feed:** The primary interface is a unified inbox feed showing all active conversations regardless of channel (Instagram icon, WhatsApp icon, etc. indicate source).
  - **Conversation View:** Standard chat interface (speech bubbles). Critical addition: When The Ambassador agent drafts a reply, it appears as a distinct "Draft" bubble with a primary "Approve & Send" button and a secondary "Edit" button.
  - **Touch Targets:** All interactive elements must exceed 44x44px.
  - **Offline/Flaky Network:** Utilize optimistic UI updates with "pending" indicators.

  ### AI Agent Integration Points

  - **The Ambassador:** Subscribes to the `omnichannel_message_created` event. When a new message arrives from a customer, the agent queries the customer's history and product catalog (via the context memory graph), and generates an `agent_draft` linked to the `work_item`/`conversation`.

  ### Key Design Decisions

  - **Native Rust:** 100% Rust implementation eliminating the Chatwoot dependency.
  - **Unified Data Model:** Consolidate `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` with strict `tenant_id` enforcement.
  - **Webhook Reliability:** Implement an idempotent webhook ingestion queue to handle bursts and transient failures from Meta/Twilio APIs.

  # Implementation Prompt

  **User-Facing Outcome:** Maya receives an Instagram DM about a vegan cake. She opens the OHC mobile app, sees the unified inbox, and finds the message alongside an AI-drafted reply ("Hi, yes we do vegan cakes!"). She taps "Approve" and the message is sent back to Instagram instantly.

  **CUJ & Acceptance Criteria:**

  1.  **Schema Implementation:** Create/Update PostgreSQL schemas with RLS for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages`.
  2.  **Core Services:** Implement robust Rust service methods for creating conversations and sending messages, ensuring tenant validation.
  3.  **Webhook Ingestion:** Implement a generic webhook receiver endpoint that validates incoming payloads (e.g., from a simulated Meta API) and routes them to the correct channel adapter.
  4.  **Agent Trigger:** Publish events to the Event Mesh upon message creation to trigger The Ambassador agent.
  5.  **Testing:** Write comprehensive unit tests for the Rust service and at least one Playwright E2E test simulating a webhook triggering a new message and verifying the draft generation flow in the UI.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
