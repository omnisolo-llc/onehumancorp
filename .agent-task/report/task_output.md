issue_title: "[Architecture] Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems or lacks a native, high-performance, multi-tenant omnichannel chat engine. Our mandate is to fully retire Chatwoot as an external service and implement a native Rust replacement for OHC. Non-technical owners (like Maya the baker or Carlos the handyman) need a unified inbox where they can handle Instagram DMs, WhatsApp messages, emails, and website chat in one place, natively integrated with their OHC data (bookings, orders, payments) and powered by AI agents.

  ## Research Report
  - **Chatwoot Audit:** Audited `chatwoot/chatwoot` source code. The core models are `Inbox`, `Conversation`, `Message`, and `Contact`. An Inbox belongs to an Account (tenant) and a Channel. Conversations group Messages between a Contact and Agents/Bots.
  - **OHC Requirement:** OHC needs a native Rust backend (in `onehumancorp/mono`) handling the core entities: Tenants, Inboxes, Channels, Conversations, Messages, and Contacts. We need row-level security in Postgres, real-time WebSockets (or gRPC streaming), and AI agent hooks.
  - **Market Context:** Competitors like Shopify Inbox, Meta Business Suite, and Zendesk provide unified inboxes, but OHC's differentiation is the deep AI integration where the system not only routes messages but drafts replies and takes actions based on the specific business data.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Channel : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Conversation : contains
      Channel ||--o{ Inbox : routes_to
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : contains

      Tenant {
          uuid id
          string name
      }
      Inbox {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      Channel {
          uuid id
          uuid tenant_id
          string provider
      }
      Contact {
          uuid id
          uuid tenant_id
          string email
          string identifier
      }
      Conversation {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      Message {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          string content
      }
  ```

  ### Architecture
  - **Data Model (Postgres + RLS):**
    - `tenant_id` on all tables.
    - `ohc_inboxes`: `id`, `tenant_id`, `name`, `channel_type`, `config`.
    - `ohc_channels`: `id`, `tenant_id`, `provider` (email, whatsapp, web_widget).
    - `ohc_contacts`: `id`, `tenant_id`, `name`, `email`, `phone`, `identifier`.
    - `ohc_conversations`: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `assignee_id`.
    - `ohc_messages`: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, agent, bot), `sender_id`, `content`, `content_type`.
  - **Services (Rust):**
    - `InboxService`: Manage inboxes and channel configurations.
    - `ConversationService`: Handle conversation lifecycle, assignments.
    - `MessageService`: Process incoming/outgoing messages, trigger AI Agent jobs, handle WebSockets/SSE for real-time UI updates.
  - **Mobile UX Flow (375px):**
    - Bottom Nav -> Inbox.
    - List view of active Conversations with unread indicators and AI draft badges.
    - Conversation view: Chat bubbles, quick action buttons (Accept Draft, Send Payment Link, Book Appointment) natively integrated with OHC tools.

  ### AI Agent Integration Points
  - **Message Ingestion Hook:** When a message is created via `MessageService`, publish an event to the AI Job Queue.
  - **Draft Generation:** AI Assistant reads conversation history + tenant context (offers, calendar) -> creates a `Message` with `status=draft` or `is_ai_draft=true`.
  - **Action Proposals:** AI Assistant can attach structured `Action` payloads to drafts (e.g., UI cards to approve a booking).

  ## Implementation Prompt
  Implement the core native Rust omnichannel chat system data models and gRPC/REST APIs in `onehumancorp/mono`.
  1. Create database migrations for `ohc_inboxes`, `ohc_contacts`, `ohc_conversations`, and `ohc_messages` with strict `tenant_id` Row Level Security.
  2. Implement the Rust service layer for creating and fetching inboxes, conversations, and messages.
  3. Implement the API handlers (gRPC or REST based on current repo patterns).
  4. Ensure 100% unit test coverage for the new models and services.
  5. Provide a basic E2E/Playwright test verifying a user can view their inboxes and messages in the UI (mocking external channels, but testing the internal pipeline).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
