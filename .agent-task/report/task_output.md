issue_title: "Native Rust Omnichannel Chat System: Core Architecture & Models"
issue_description: |
  ## Problem Statement
  Business owners like Maya, Carlos, and Priya receive customer inquiries from various platforms—Instagram DMs, WhatsApp, SMS, and website chat. Managing these across different apps is chaotic and leads to missed sales, dropped leads, and poor customer service. They need a single, unified "inbox" in their OHC assistant where all messages land. The assistant needs to seamlessly coordinate responses, identify the customer context, allow AI agents to draft replies or take action (like quoting a price or checking inventory), and let the owner take over when necessary. The current system lacks a robust, scalable native omnichannel chat architecture to replace the retired external Chatwoot dependency.

  ## Research Report & Competitive Analysis
  Based on a deep audit of the `chatwoot` open-source repository and our core platform requirements, several key architectural components are required to achieve parity and build a foundation for AI-first coordination.

  **Chatwoot Architecture Insights:**
  1.  **Data Models (from `app/models`):**
      - `Account` (matches OHC `tenant_id`)
      - `Inbox` (channels like Email, WhatsApp, Instagram are tied to an inbox)
      - `Channel` (specific configuration and credentials for the integration)
      - `Contact` & `ContactInbox` (Customer identity mapping)
      - `Conversation` (the core entity linking a contact to an inbox, tracking status/assignee)
      - `Message` (the individual events/chats within a conversation)
      - `AgentBot` / `AgentBotInbox` (used for AI handoffs)
  2.  **State Management:** Chatwoot uses extensive state machines (`status`: open, pending, snoozed, resolved) and event dispatching (`dispatcher_dispatch`) on `Conversation` to notify users via WebSockets and trigger automation rules.
  3.  **Extensibility:** Channels are abstracted (e.g., `Channel::WebWidget`, `Channel::TwitterProfile`), allowing uniform handling of incoming webhooks.

  **Platform Requirements for OHC:**
  -   Must be a **Native Rust** implementation under `src/server/services/chat/` using SQLx and PostgreSQL.
  -   Must enforce strict row-level `tenant_id` isolation.
  -   Must natively integrate with OHC's AI job queue and distributed locks (Redis Redlock).
  -   Must support offline-first mobile sync capabilities using PowerSync (SQLite on client -> PostgreSQL).

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CHAT_INBOX : owns
      TENANT ||--o{ CHAT_CONTACT : owns
      CHAT_INBOX ||--o{ CHAT_CHANNEL : configures
      CHAT_INBOX ||--o{ CHAT_CONVERSATION : contains
      CHAT_CONTACT ||--o{ CHAT_CONVERSATION : initiates
      CHAT_CONVERSATION ||--o{ CHAT_MESSAGE : has

      CHAT_INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean is_active
      }
      CHAT_CHANNEL {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string provider_type "e.g., instagram, web, whatsapp"
          jsonb credentials
      }
      CHAT_CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone
          string email
          jsonb attributes
      }
      CHAT_CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id "Optional human or bot ID"
          string status "open, resolved, pending, snoozed"
          timestamp last_activity_at
      }
      CHAT_MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id "Optional, null if system"
          string sender_type "contact, agent, bot, system"
          text content
          jsonb metadata
          timestamp created_at
      }
  ```

  ### 2. Mobile UX Flow (375px First)
  - **The Work Feed (Home):** The owner opens the app. The primary view is a unified feed of unresolved conversations, prioritizing new inquiries and agent drafts awaiting approval. Clean UniFi-style cards display the customer name, channel icon (e.g., WhatsApp), and a snippet of the latest message.
  - **Conversation View:** Tapping a card opens a chat interface.
    -   *Header:* Translucent glass effect showing customer name, status toggle (Open/Resolved), and Quick Actions (View Profile, Create Order).
    -   *Message List:* Distinct visual grouping for customer messages (left), owner replies (right), and AI agent actions/drafts (center/inline with distinct visual tint).
    -   *Composer:* Native mobile keyboard layout with attachment support. If an AI draft exists, it appears above the composer with "Approve" or "Edit" actions.

  ### 3. AI Agent Integration Points
  - **Triage & Routing:** When a new `CHAT_MESSAGE` is inserted via webhook, a PostgreSQL `SKIP LOCKED` job is queued. The "Work Triage" agent evaluates the message, links it to an existing `CHAT_CONTACT` (or creates one), and determines if immediate AI response drafting is needed.
  - **Drafting & Handoff:** If Maya receives an IG DM asking "Do you make vegan cakes?", the "Customer Assistant" agent drafts a reply based on knowledge base memory. The draft is stored as a pending message. The conversation `status` is set to `pending`. Maya reviews it in the feed, approves it, and the system sends it via the `CHAT_CHANNEL` provider.
  - **Context Sharing:** All agent departments (Operations, Sales) can read the `CHAT_CONVERSATION` history to understand customer intent when generating quotes or bookings.

  ### 4. Key Design Decisions
  -   **Single Unified Timeline:** `CHAT_MESSAGE` table will store all interactions (text, attachments, system events like "Quote Sent") to ensure the UI can render a continuous, chronological history.
  -   **Database-Backed Queues for Webhooks:** Incoming channel webhooks will quickly insert raw payloads into a staging table or Redis queue to acknowledge receipt instantly, while background Rust workers process them into normalized `CHAT_MESSAGE` records.
  -   **Strict Multi-Tenancy:** Every table includes `tenant_id` to leverage PostgreSQL Row-Level Security (RLS) ensuring absolute zero-trust data isolation between owner accounts.

  ## Implementation Prompt (For Implementer Agent)
  **Objective:** Implement the core PostgreSQL data models and foundational Rust service layer for the OHC Native Omnichannel Chat system to replace Chatwoot functionality.

  **Target Persona & CUJ:**
  - *Persona:* Carlos (handyman) managing inquiries on his Android phone.
  - *CUJ:* Carlos receives a new SMS inquiry. The system must create a `Contact`, an `Inbox` (if none exists for SMS), a `Conversation`, and the incoming `Message`. Carlos opens the app and sees the unread conversation grouped under the new contact. He taps "Resolve" when finished, updating the conversation status.

  **Acceptance Criteria:**
  1.  **Database Migration:** Create a new SQLx migration defining the tables: `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages`. All tables MUST include `tenant_id` and have constraints/indexes to enforce multi-tenancy.
  2.  **Rust Models:** Define the corresponding SQLx struct models in `src/server/services/chat/models.rs`.
  3.  **Service Layer:** Implement core CRUD operations in `src/server/services/chat/service.rs` (or similar domain structure) to create a conversation and append a message, ensuring transactional safety and `tenant_id` authorization.
  4.  **Testing (MANDATORY):** Write comprehensive Rust unit tests validating the models and service layer. Must verify that a user from Tenant A cannot access or append messages to a conversation in Tenant B.
  5.  **Clean Run:** `bazel build //...` and `bazel test //...` must pass with 100% coverage on the new code.

  **Constraints:**
  - Do NOT implement the external webhooks (e.g., Twilio/Meta APIs) in this task. Focus strictly on the core internal data model, domain entities, and service boundaries.
  - Use UUIDs for all primary keys.
  - Rely on `chrono::DateTime<Utc>` for timestamps.
  - Ensure compatibility with PowerSync requirements if applicable (e.g., standard `created_at` / `updated_at` triggers).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
