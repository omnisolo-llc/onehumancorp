issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: Native Rust Omnichannel Chat System

  ## Executive Summary
  Per the OHC Engineering Standards, the external third-party Chatwoot service is being 100% retired. OHC requires a highly performant, multi-tenant omnichannel customer support & chat engine natively built in Rust within `onehumancorp/mono`. This report outlines the architecture and design necessary to achieve 100% feature parity with Chatwoot, focusing specifically on WhatsApp and Web Widget integration, which are critical for our target personas like Maya (Home Baker) and Fatima (Food Cart Operator).

  ## 1. Track 1: Architectural Gap & Scaling Discovery
  - **Current State:** OHC currently relies on external systems or lacks a unified inbox. The `src/server/integrations/chat/` directory exists but is essentially empty, only containing a README.
  - **Chatwoot Source Code Audit:**
    - Analyzed Chatwoot's Ruby on Rails source code (from `https://github.com/chatwoot/chatwoot`).
    - Key models identified: `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::Whatsapp`, `Channel::WebWidget`.
    - Real-time communication relies on ActionCable (WebSockets).
    - Webhooks are used extensively to receive messages from providers like Meta (WhatsApp).
  - **The Gap:** OHC needs a Rust-native equivalent to Chatwoot's core data models, WebSocket handling for real-time Web Widget chat, and webhook ingestion for WhatsApp, all strictly enforcing multi-tenancy (Row-Level Security via `tenant_id`).

  ## 2. Track 2: Selected Architecture Deep Dive
  ### Business Journey Mapping
  - **Acquisition/Onboarding:** Maya links her WhatsApp Business account via OHC's clean mobile UI (using Meta Embedded Signup).
  - **Operation:** Customers message Maya on WhatsApp or via her OHC storefront Web Widget.
  - **Agent Triage:** Incoming messages trigger the Work Triage system. The "Ambassador" Agent (Customer Success) intercepts, queries the DB for customer history/inventory, and drafts a reply.
  - **Resolution:** Maya sees the drafted reply in her mobile feed (375px view), taps "Approve", and the Rust omnichannel gateway dispatches the message back to the appropriate channel.

  ### Data Model & Invariants
  - **Inbox:** Represents a unified message queue for a tenant.
  - **Channel (Enum/Trait):** Defines the source (WhatsApp, WebWidget).
  - **Conversation:** Links a Contact (Customer) to an Inbox.
  - **Message:** The actual communication payload.
  - **Multi-Tenancy:** Every table must have a `tenant_id` with Postgres RLS enforced.
  - **Distributed Locks:** Use Redis (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to prevent race conditions when agents and humans try to reply simultaneously.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] --> B(Omnichannel Gateway - Rust)
      C[Web Widget WebSocket] <--> B
      B --> D[Message Ingestion & RLS Checks]
      D --> E[(PostgreSQL Central Ledger)]
      D --> F[Event Mesh / Redis PubSub]
      F --> G[Work Triage / AI Agents]
      G -->|Drafts Reply| E
      E --> H[Mobile API 375px]
      H --> I[Owner UI: Approve Reply]
      I --> J[Channel Dispatcher]
      J -->|Sends| A
      J -->|Sends| C
  ```

  ### AI Department Coordination
  - **Customer Success Agent ("The Ambassador"):** Subscribes to new message events. Uses RAG against the tenant's product catalog and past conversations to draft replies.

  ## 3. Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX Flow:** The unified inbox must render flawlessly on a 375px screen. Instead of a complex multi-column chat interface, it should present a prioritized feed of actionable cards (e.g., "Drafted Reply for Sarah - [Approve] [Edit]").
  - **Performance Targets:** Webhook ingestion must acknowledge receipt (200 OK) in < 100ms. WebSocket latency for Web Widget should be < 50ms for real-time feel.
  - **Zero Trust & Security:** Strict RLS implementation in Postgres. Webhook signature verification (e.g., Meta's `X-Hub-Signature`) must be implemented to ensure payload authenticity.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** Business owners can connect WhatsApp and install a Web Widget, viewing and responding to all customer inquiries in a single, agent-assisted feed on their mobile device, without relying on third-party tools like Chatwoot.

  **Critical User Journeys (CUJ) & Acceptance Criteria:**
  1. **Data Models:** Implement Rust structs and SeaORM/SQLx entities for `Inbox`, `Conversation`, `Message`, `ChannelAdapter` (supporting WhatsApp and WebWidget types) inside `src/server/integrations/chat/`. Ensure `tenant_id` is present on all models.
  2. **WhatsApp Gateway:** Implement an Axum route to receive Meta WhatsApp webhooks, verify signatures, and parse incoming text messages into the `Message` table.
  3. **Web Widget WebSocket:** Implement an Axum WebSocket handler that allows real-time two-way communication, saving messages to the database.
  4. **Agent Integration:** Emit an event (e.g., via Tokio broadcast or Redis pub/sub) when a new message is saved, allowing the Ambassador Agent to pick it up.
  5. **Automated Testing:** Provide comprehensive unit tests (100% coverage for the new domain) and Playwright E2E tests simulating an incoming WhatsApp webhook and an owner approving an agent-drafted reply.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
