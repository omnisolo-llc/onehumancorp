issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat Engine (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) is replacing its reliance on external dependencies like Chatwoot with a highly-scalable, native Rust implementation. Small-business owners and operators (our key personas: Maya, Carlos, Priya, Leo, Fatima) need a unified, lightning-fast inbox to triage customer messages across Instagram, WhatsApp, Email, and Web chat. They cannot suffer latency or offline-sync issues caused by third-party webhooks. A native Rust chat system integrated directly into our monorepo guarantees zero-trust multi-tenancy, real-time WebSocket syncing, and seamless OHC AI agent coordination (Operations, CS, Sales).

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture revolves around the following entities: `Account` (Tenant), `User` (Agent), `Contact` (Customer), `Inbox` (Channel configuration), `Conversation` (Thread), and `Message` (Payload).

  Unlike Chatwoot's Ruby on Rails architecture, OHC's implementation will use Rust (via `SQLx`/PostgreSQL) to ensure strict memory safety, extreme concurrency, and single-binary deployment.

  ## Design Doc & Architecture

  ### Data Models & Rust Mapping
  - `Tenant` (replaces `Account`): Row-level security for tenant isolation.
  - `Agent` (replaces `User`): The OHC user or AI agent handling the conversation.
  - `Contact`: The external customer.
  - `Inbox`: Channel configurations (e.g., WhatsApp, IG, Web Widget).
  - `Conversation`: Thread linking an Inbox, Contact, and Agent.
  - `Message`: Chat payload (Text, Image, System Event) linked to a Conversation.

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "has"
      Tenant ||--o{ Contact : "owns"
      Tenant ||--o{ Agent : "employs"
      Inbox ||--o{ Conversation : "contains"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"
      Agent ||--o{ Message : "sends"
      Contact ||--o{ Message : "sends"
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home):** Unified list of `Conversations` sorted by `last_activity_at`. Unread messages have a subtle translucent badge.
  2. **Conversation View:** Full-screen chat interface. Native mobile keyboard support. Messages bubble style (Customer left, Agent/AI right).
  3. **Action Drawer:** Swipe up to reveal quick actions: "Generate AI Reply", "Send Payment Link", "Book Appointment".
  4. **Offline Resilience:** Messages sent offline are cached locally (PWA/Flutter) and marked with a pending clock icon, syncing via WebSocket upon reconnection.

  ### AI Agent Integration
  - **CS Agent:** Automatically drafts replies to common inquiries (e.g., "Do you do vegan cakes?") by analyzing `Messages` within a `Conversation`.
  - **Operations Agent:** Monitors `Conversations` for intent (e.g., booking a service) and transitions the state to create a calendar event or quote.
  - **Redis Coordination:** Lock key pattern `ohc:lock:{tenant_id}:conversation:{conversation_id}` ensures only one AI agent or human drafts/sends a response at a time.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core database schema (using SQLx/PostgreSQL) and Rust service layer for the native OHC Chat Engine.
  1. Define the SQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` is present on every table with Row-Level Security (RLS) enabled.
  2. Create the Rust data structs and CRUD operations for these entities.
  3. Implement a basic WebSocket broadcasting channel (using Tokio/Axum) to emit `message.created` events to connected clients.
  4. Ensure 100% unit test coverage for the service layer and multi-tenant isolation.

  *Do not prescribe specific frontend Flutter UI code in this PR; focus on the backend Rust architecture and data invariants.*

  ## Scope & Priority
  **Estimated Scope:** Large
  **Priority:** P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust]
assignees: []
