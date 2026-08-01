issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Parity)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot as an external dependency with a native, high-performance Rust omnichannel chat engine. Our current `src/server/services/chat` provides only a basic schema (Inbox, Channel, Contact, Conversation, Message). It lacks the advanced routing, channel adapters (WhatsApp, Email, Web Widget, Instagram), SLA policies, WebSocket real-time messaging, and multi-tenant row-level security required by non-technical operators (like Maya the baker or Carlos the handyman) to manage all customer interactions transparently from a unified, mobile-first feed.

  ## Research Report
  - **Chatwoot Audit:** Analyzed the Chatwoot source code repository (specifically `app/models/`, `app/models/channel/`, `app/controllers/`).
  - Chatwoot handles multi-channel integrations using polymorphic associations for `Channel` (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`).
  - Conversations have robust states (`open`, `snoozed`, `resolved`), assignee tracking, agent last seen, and SLAs.
  - Chatwoot uses background jobs (Sidekiq) heavily for message delivery, webhook processing, and automation rules.
  - **OHC Architecture Parity:** To replicate this in OHC's native Rust system, we need to extend the data models to support polymorphic channel configurations (Instagram, SMS, Web), introduce WebSocket capabilities for real-time inbox updates, and define strict multi-tenant Row-Level Security (RLS) in PostgreSQL. The system must support AI Agent routing so AI "Customer Assistant" can automatically draft replies.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid tenant_id
          uuid inbox_id
          string channel_type
          jsonb config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone
          string email
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          uuid assignee_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          uuid sender_id
          string content
      }
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : holds
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View:** A unified scrollable list of open conversations. Translucent glass sticky header showing "All Messages" vs "Assigned to Me".
  - **Conversation View:** Full height chat interface. Sticky bottom input with native keyboard. A prominent "AI Draft" button next to send.
  - **Channel Indicators:** Small badges on conversation list items (e.g., WhatsApp icon, IG icon) to indicate source.

  ### AI Agent Integration
  - **Customer & Relationship Assistant:** Subscribes to the `MessageCreated` event via the background job queue (Pg `SKIP LOCKED`). Automatically reviews the conversation context and drafts replies for the owner's review.
  - **Work Triage Agent:** Scans incoming messages to extract potential tasks (e.g., "quote requested"), creating actionable items in the owner's feed.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the extended database schema and Rust service layer for the native Omnichannel Chat System. Extend `src/server/services/chat` to include channel-specific configuration schemas (WhatsApp, IG, Web Widget). Implement WebSocket controllers for real-time delivery of messages to the UI.
  Ensure strict multi-tenant isolation using PostgreSQL RLS (every query must filter by `tenant_id`). The UI should reflect a premium macOS-style translucent glass interface for the conversation list and chat view. Ensure complete functionality on a 375px viewport layout. The AI assistant should be able to hook into conversation creation to propose drafts. Include at least 5 Playwright E2E tests validating the full chat flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
