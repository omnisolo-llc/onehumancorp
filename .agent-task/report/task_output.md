issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Research Report: Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing the external Chatwoot dependency with a native, high-performance Rust omnichannel chat engine. This new system must support seamless integration of multiple communication channels (Email, Facebook, Instagram, Line, SMS, Telegram, TikTok, Twilio SMS, Twitter, Web Widget, WhatsApp) into a single unified inbox for our owner/operator personas (Maya, Carlos, Priya, Leo, Fatima). Relying on an external provider creates unnecessary complexity, latency, and operational overhead. We need a native system that integrates perfectly with OHC's multi-tenant architecture and AI agents.

  ## Research Findings & Benchmark
  We reviewed the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`) to understand its architecture and map it to a native Rust implementation:

  *   **Data Models**:
      *   `Account` (Tenant): Maps to OHC's `tenant_id` for multi-tenancy.
      *   `Inbox`: Belongs to a tenant and acts as a container for conversations.
      *   `Channel`: Specific integrations (e.g., WhatsApp, Email) tied to an Inbox.
      *   `Conversation`: A thread of messages between a contact and the business.
      *   `Message`: Individual text/media payloads with status (sent, delivered, read).
      *   `Contact`: The customer identity across channels.
  *   **Core Interactions**:
      *   Webhooks from providers (Stripe, Twilio, Meta) ingest messages into the appropriate Channel/Inbox.
      *   Messages are routed to Conversations.
      *   Real-time updates via WebSockets push new messages to the UI.
  *   **Multi-tenancy**: Must strictly use OHC's `tenant_id` row-level security.

  ## Architecture Design

  ### 1. Data Model (PostgreSQL & Rust Structs)
  *   `inboxes` table: `id`, `tenant_id`, `name`, `channel_type`.
  *   `conversations` table: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, closed, snoozed), `assignee_id`.
  *   `messages` table: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, agent, bot), `content`, `content_type`, `status`.
  *   `contacts` table: `id`, `tenant_id`, `name`, `email`, `phone`, `identifier`.
  *   `channel_adapters` tables (e.g., `channel_whatsapp`, `channel_email`): Configs for specific channels.

  ### 2. Microservices / Crates (`src/server/ohc/domain/chat/`)
  *   **Ingestion Service**: Handles incoming webhooks from external providers, normalizes payloads, and queues them using PostgreSQL `SKIP LOCKED`.
  *   **Message Processing Worker**: Dequeues normalized messages, creates/updates `Contact` and `Conversation` records, and saves `Message` rows.
  *   **Real-time Event Broadcaster**: Uses Redis Pub/Sub or similar mechanism to broadcast updates to connected WebSockets.
  *   **AI Agent Integration**: Automatically processes new messages for triage (Work Triage) and drafting replies (Customer Assistant).

  ### 3. Mobile-First UX Flow (375px)
  *   **Inbox List**: A clean, unified list of conversations.
  *   **Conversation View**: Apple Messages-style chat bubbles. Clear distinction between customer messages and AI-drafted responses pending owner approval.
  *   **Quick Actions**: One-tap buttons to approve AI drafts, request payment, or schedule a booking directly from the chat.
  *   All UI must use OHC Premium Token library (Translucent materials, Ubiquiti-style hierarchy).

  ### 4. Sequence Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Cust as Customer (WhatsApp)
      participant Ingest as OHC Webhook Ingestion
      participant Queue as PG Job Queue
      participant Worker as Message Worker
      participant DB as PostgreSQL
      participant AI as AI Customer Assistant
      participant WS as WebSocket Broadcaster
      participant Owner as Owner (Mobile App)

      Cust->>Ingest: Send WhatsApp Message
      Ingest->>Queue: Enqueue Raw Payload
      Queue->>Worker: Dequeue Job
      Worker->>DB: Upsert Contact & Conversation
      Worker->>DB: Insert Message
      Worker->>AI: Trigger AI Triage/Draft
      AI->>DB: Save AI Draft Reply
      Worker->>WS: Broadcast New Message & Draft
      WS->>Owner: UI Update (Real-time)
  ```

  ## Implementation Prompt (For Implementer Agent)
  **User Facing Outcome:** The business owner can open the OHC app and see a unified inbox of all customer messages (from web widget, SMS, etc.). They can read messages and see AI-drafted replies ready for their approval.

  **Acceptance Criteria:**
  1.  Create the database schema (PostgreSQL with RLS on `tenant_id`) for `inboxes`, `conversations`, `messages`, and `contacts` based on the Chatwoot model but adapted for OHC's Rust backend.
  2.  Implement the basic Rust service layer (`src/server/ohc/domain/chat/`) for CRUD operations on these entities.
  3.  Implement a webhook ingestion endpoint capable of receiving simulated channel messages and saving them to the database.
  4.  Ensure 100% unit test coverage for the new Rust modules.
  5.  Ensure no external Chatwoot dependencies are used.

  ## Priority & Scope
  **Priority:** P0 (Critical Path for OHC's core value proposition)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
