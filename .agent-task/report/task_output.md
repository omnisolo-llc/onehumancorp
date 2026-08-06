issue_title: "Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ### Problem Statement
  OneHumanCorp (OHC) is an AI work assistant for business owners/operators. A core capability is Customer & Relationship Assistant, providing unified omnichannel support (DMs, emails, WhatsApp, web chat). Currently, the architecture relies on an external third-party service, Chatwoot. However, the architectural mandate strictly requires "Complete Chatwoot Retirement". Chatwoot as an external service/dependency is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. This system needs to be fast, memory-safe, and deeply integrated with our AI Agents (e.g. for automatic replies, triage, sentiment analysis) while working beautifully on a 375px mobile screen.

  ### Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core entities and functionality required for parity include:
  - **Account/Tenant Isolation:** All records must be strictly isolated by `tenant_id` (`account_id` in Chatwoot).
  - **Inboxes & Channels:** Abstracting the source of messages (Email, WhatsApp, Web Widget, API, Instagram).
  - **Contacts & ContactInboxes:** Representing the end-user.
  - **Conversations:** The central entity grouping messages between a Contact and an Inbox, including statuses (Open, Snoozed, Resolved), assignments (Agent/Team/AI), and labels.
  - **Messages:** The atomic units of communication, supporting text, attachments, internal notes (private), and rich interactive templates.
  - **Real-time WebSockets:** For instant updates to the owner's dashboard and the web widget.
  - **AI & Automation:** Webhooks/internal events to trigger AI Agent workflows (triage, draft generation) for new messages.

  Competitor Analysis (Shopify Inbox, WeCom):
  - Prioritize a single, unified inbox view. Owners don't want to switch between "Instagram" and "WhatsApp" tabs; they want a unified "Needs Reply" feed.
  - AI auto-responders must be natively integrated rather than bolted on. The AI agent should be treated as a first-class "Assignee" or "Participant" in the conversation.

  ### Design Doc

  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      CONTACT ||--o{ CONTACT_INBOX : has
      INBOX ||--o{ CONTACT_INBOX : contains
      CONTACT_INBOX ||--o{ CONVERSATION : spawns
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }|..|{ AGENT : assigned_to

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
          jsonb credentials
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid contact_inbox_id
          string status
          uuid assignee_id
          datetime last_activity_at
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string message_type
          boolean is_private
          jsonb attachments
      }
  ```

  #### Mobile UX Flow (375px)
  1. **Unified Inbox (Home):** A clear list of active conversations. Each row shows the contact's avatar, name, channel icon (e.g., IG, Email), a snippet of the last message, and a timestamp. Unread/AI-drafted messages are highlighted.
  2. **Conversation View:** Tapping a row opens the chat thread. A sticky header shows the contact name and channel. The message list uses distinct bubbles for incoming, outgoing, and internal AI notes.
  3. **Reply Action:** A sticky bottom input area. A prominent "AI Draft" button allows the owner to instantly review and send an AI-generated reply. Native mobile keyboard integration.

  #### AI Agent Integration Points
  - **Incoming Message Webhook/Event:** When a `MESSAGE` is created with `message_type=incoming`, an event is emitted. The `Customer & Relationship Assistant` AI agent is triggered.
  - **AI Drafting:** The agent reads the conversation context and creates a `MESSAGE` with `is_private=true` and `ai_draft=true`, which the owner can approve/edit.
  - **Auto-Triage:** The agent can update the `CONVERSATION` tags/labels and assign it to specific team members based on intent (e.g., "Refund Request" -> Finance Team).

  #### Key Design Decisions
  - **Rust native:** Implement the core CRUD and WebSocket handlers in Axum (Rust).
  - **Database:** PostgreSQL with Row-Level Security (RLS) enforcing `tenant_id` on every table.
  - **Real-time:** Use Axum WebSockets and Redis Pub/Sub for horizontal scaling of real-time events.
  - **Schema:** Adopt UUIDs for primary keys to prevent enumeration and simplify offline-first mobile sync.

  ### Implementation Prompt
  Implement the core database schema, Rust models, and Axum API endpoints for the native omnichannel chat system (Conversations, Messages, Inboxes, Contacts).
  - Ensure strict multi-tenant isolation using `tenant_id`.
  - Create the Axum HTTP routes for CRUD operations on these entities.
  - The API must be consumed by a new Tauri (Flutter/React) mobile-first Unified Inbox UI.
  - Add robust unit tests (100% coverage) and Playwright E2E tests simulating a customer sending a message and the owner replying.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
