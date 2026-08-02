issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System for OHC

  ## Problem Statement
  OHC requires a high-performance, unified, multi-tenant omnichannel chat system to serve its diverse owner/operator personas (e.g., Maya the Baker, Carlos the Handyman). Previously, there was consideration of using external services like Chatwoot, but as per the OHC Engineering Standards, Chatwoot as an external service is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`.

  This system must seamlessly handle WhatsApp Business and Web Widget messages in a unified, lightning-fast manner, unifying messages, tasks, and alerts into a prioritized owner feed. It must also ensure that advanced AI agents can draft replies and take actions.

  ## Research Report & Findings
  Based on an exhaustive audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and competitive research of leading SMB omnichannel platforms:

  1. **Data Model Deficiencies:**
     Chatwoot relies on heavily relational Ruby on Rails schemas (`inboxes`, `conversations`, `messages`, `contacts`, `channel_web_widgets`, etc.). While functional, OHC needs these implemented in Rust with strict PostgreSQL RLS (Row Level Security) isolating by `tenant_id`.
  2. **WebSocket & Realtime:**
     Chatwoot uses ActionCable for real-time web widget and dashboard communication. OHC's native Rust implementation should leverage asynchronous I/O (e.g., Tokio + WebSockets/gRPC) for better resource utilization at scale.
  3. **AI Integration Gap:**
     Chatwoot's core architecture isn't built "AI-first". OHC requires native hooks for the AI Job Queue (PostgreSQL `SKIP LOCKED`) to allow the Customer & Relationship Assistant to draft replies for chat seamlessly.
  4. **Multi-Tenancy:**
     Chatwoot achieves multi-tenancy via standard foreign keys (`account_id`). OHC enforces deep tenant isolation via PostgreSQL RLS on `tenant_id`.

  ## Design Doc (Architecture Design)
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[End Users / Customers] -->|WebSockets / HTTP| B(Rust API Gateway)
      C[WhatsApp / Webhooks] -->|HTTP| B
      B --> D{Native Rust Omnichannel Engine}
      D -->|PostgreSQL RLS| E[(PostgreSQL: Inboxes, Conversations, Messages, Contacts)]
      D --> F[AI Job Queue - SKIP LOCKED]
      F --> G(AI Customer Assistant Worker)
      G -->|Proposes Drafts| D
      H[Owner / Operator Mobile App] -->|gRPC / REST| B
  ```

  ### Mobile UX Flow (375px First)
  - **Triage Feed (Home):** The owner opens the app (e.g., Carlos on Android). The main view is a prioritized list of `Conversations`. Unread messages and AI drafts have a translucent glass badge. Touch targets are 44x44px.
  - **Conversation View:** Tapping a conversation opens the thread. The AI's suggested draft is clearly visible above the keyboard. The owner can tap "Send" or edit.
  - **Offline Resilience:** Reads are cached. Pending sends are queued and retried with idempotency keys upon reconnection.

  ### AI Agent Integration Points
  - **Work Triage:** When a `Message` is inserted, a hook evaluates if it creates a new `Conversation` or appends. It emits an event to the AI Job Queue to classify the intent.
  - **Draft Generation:** The Customer Assistant picks up the job, reads `tenant` context (policies, previous messages), and generates a draft reply, persisting it to an `agent_draft` table associated with the `Conversation`.

  ### Key Design Decisions
  1. **Rust Native Microservices/Crates:** Complete departure from Ruby/Rails. Everything from routing webhook payloads to WebSocket connection management is done in Rust, yielding predictable latencies.
  2. **RLS by Default:** Every new table (`omni_inboxes`, `omni_conversations`, `omni_messages`) must have `tenant_id` and RLS policies enforcing tenant isolation.
  3. **Idempotency & Queuing:** Integration webhooks (Meta/WhatsApp) must use idempotency keys. Background processing leverages PostgreSQL `SKIP LOCKED` instead of external dependencies like Redis/Sidekiq for atomic job processing.

  ## Implementation Prompt (For Implementer Agent)
  **Objective:** Implement the backend foundation for the Native Rust Omnichannel Chat System, providing functional parity with Chatwoot's core `Inbox`, `Conversation`, `Message`, and `Contact` models, but built on OHC's stack (Rust, PostgreSQL with RLS, Bazel).

  **Acceptance Criteria:**
  1. Define the SQL schemas and RLS policies for `omni_inboxes`, `omni_conversations`, `omni_messages`, and `omni_contacts` in a new migration file.
  2. Implement the Rust data models, repository patterns, and core service logic for these entities.
  3. Implement the gRPC/REST API endpoints to list inboxes, create conversations, and send messages.
  4. Ensure 100% unit test coverage for the new Rust code and confirm `bazel test //...` passes completely.
  5. Adhere strictly to the OHC multi-tenant isolation standards (RLS).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
