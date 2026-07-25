issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chat-woot Replacement)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat System

  ## Problem Statement
  OHC requires a native, high-performance omnichannel chat system built in Rust to permanently retire the external Chat-woot dependency. The business owners (Maya, Carlos, Priya, Leo, Fatima) need a unified inbox that consolidates customer interactions across Instagram DMs, SMS, WhatsApp, Web Chat, and Email without context-switching. Current external dependencies lack the deep, unified integration with OHC's internal AI capabilities (triage, operations, sales, and knowledge assistance), and violate our goal of complete, seamless control over the multi-tenant architecture and mobile-first experience.

  ## Research Report
  - **Source Code Benchmarking:** Audited the `chat-woot/chat-woot` repository (v3+ architecture). Chat-woot's Ruby on Rails architecture heavily relies on polymorphic associations (Channels, Inboxes) and a unified `Conversation` model linked to `Messages`, `Contacts`, and `Account` (tenant).
  - **Data Model Mapping:**
    - `Account` -> OHC `Tenant`
    - `Contact` -> OHC `Contact`
    - `Conversation` -> OHC `Conversation` (tied to Tenant and Contact)
    - `Message` -> OHC `Message`
    - `Inbox` & `Channel::*` -> OHC `ChannelAdapter` and `Inbox`
  - **WebSockets:** Chat-woot uses ActionCable. OHC will use a Rust-based async WebSocket service (via Tokio/Axum/Tungstenite) coupled with Redis for pub/sub across horizontal instances to deliver real-time updates.
  - **Extensibility & AI:** Chat-woot uses AgentBots. OHC's architecture will natively integrate with our internal `ai_job_queue` using PostgreSQL `SKIP LOCKED`. When a new message arrives, it will immediately trigger OHC's AI triage and drafting capabilities before notifying the human operator.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : manages
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--o{ INBOX : feeds

      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid contact_id
          uuid inbox_id
          string status
          timestamp last_activity_at
      }

      MESSAGE {
          uuid id
          uuid conversation_id
          uuid tenant_id
          string content
          string message_type
          uuid sender_id
          string sender_type
          timestamp created_at
      }
  ```

  ### Mobile-First UX Flow (375px)
  - **Unified Inbox View:** A clean list of active conversations sorted by `last_activity_at`, showing contact avatar, name, channel icon (e.g., WhatsApp, Instagram), and the last message preview. Read/unread status indicated by typography weight and a subtle colored dot (Translucent Glass styling).
  - **Conversation View:** Edge-to-edge message bubbles. A sticky bottom composer with auto-expanding text area. Prominent AI agent suggestions ("Draft Reply", "Create Booking", "Send Quote") floating above the composer as pill buttons.
  - **Contact Context:** Swiping left on the conversation view reveals the Contact Profile (past orders, notes, tags) without leaving the chat context, critical for quick context recall on a 375px screen.

  ### AI Agent Integration
  - **Event Trigger:** Every new `Message` insert emits a domain event to the AI Job Queue.
  - **Triage & Draft:** The AI Customer & Relationship Assistant reads the message history, contact context, and knowledge base to draft a reply.
  - **Agent Action:** The drafted reply is saved as a `Message` with status `draft` and `sender_type = "ai_agent"`. The UI displays this distinctly, allowing the owner to "Approve & Send" or edit.

  ### Multi-Tenant & Security Invariants
  - Row-Level Security (RLS) is strictly enforced. Every query to `inboxes`, `conversations`, and `messages` MUST include the `tenant_id`.
  - All distributed locks (e.g., for channel sync) must use the pattern `ohc:lock:{tenant_id}:channel:{channel_id}`.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the core database schemas, Rust domain models, and primary API endpoints for the native OHC Omnichannel Chat System, matching the architecture defined above.

  1. Define the PostgreSQL migrations for `inboxes`, `conversations`, and `messages` ensuring `tenant_id` is present on all tables for RLS.
  2. Implement the Rust data models in `src/server/ohc/chat/domain` or equivalent, ensuring proper serialization and validation.
  3. Create the core REST/gRPC API endpoints to list conversations, fetch messages for a conversation, and send a message.
  4. Ensure zero mock data is used; all API responses must stem from the actual database schemas.
  5. Achieve 100% unit test coverage for the domain logic and repository layers.
  6. Ensure all code builds successfully with `bazel build //...` and tests pass with `bazel test //...`.

  **Acceptance Criteria:**
  - Database migrations execute cleanly.
  - Rust API endpoints successfully create and retrieve conversations and messages isolated by `tenant_id`.
  - The implementation proves readiness to connect to the Flutter frontend and the AI Job Queue.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
