issue_title: "Implement Native Rust Omnichannel Inbox Architecture"
issue_description: |
  # Native Rust Omnichannel Inbox Architecture

  ## Problem Statement
  As a business owner like Maya (the baker) or Carlos (the handyman), managing inquiries from Instagram, WhatsApp, website chat, and SMS natively within OneHumanCorp is critical for converting leads into bookings. Previously, this relied on integrating with external services like Chatwoot.

  The mandate requires 100% retirement of Chatwoot as an external service. We need a native Rust omnichannel customer support & chat engine built natively inside the `onehumancorp/mono` codebase, mirroring the rich channel adapter and inbox models of Chatwoot but strictly designed for our small-business personas with high multi-tenant isolation, performance, and offline-first mobile synchronization.

  ## Research Report
  - **Chatwoot Source Audit:** We audited `https://github.com/chatwoot/chatwoot`, specifically `app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`, and `app/models/channel/`.
  - **Key Models:**
    - `Inbox`: The central hub for a tenant's conversations, connected to specific channels.
    - `Channel`: The interface (WebWidget, Instagram, WhatsApp, SMS, Email).
    - `Conversation`: The threaded state between a Contact and the Business (Tenant), optionally assigned to an Agent/Bot.
    - `Message`: Immutable entries in a conversation, including text, attachments, and system events.
    - `Contact`: The end-user communicating with the business.
  - **OHC Specific Needs:**
    - **Multi-Tenancy:** PostgreSQL `tenant_id` on every table with Row Level Security (RLS).
    - **Performance:** Asynchronous message processing via background workers (Redis/Postgres job queue).
    - **Mobile-First:** Synchronization via CRDTs or PowerSync for offline resilience (vital for Fatima the food cart operator on slow networks).
    - **AI Integration:** Native "Agent Bot" routing where OHC's internal AI agents can triage, draft replies, and execute bookings before human intervention.

  ## Design Doc

  ### Architecture
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : configures
      Inbox ||--o{ Channel : has
      Channel ||--o{ Conversation : spawns
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : contains
      Tenant ||--o{ Contact : owns

      Tenant {
          uuid tenant_id
          string name
      }
      Inbox {
          uuid id
          uuid tenant_id
          string name
          boolean working_hours_enabled
      }
      Channel {
          uuid id
          string type_enum
          jsonb credentials
      }
      Conversation {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          timestamp snoozed_until
      }
      Message {
          uuid id
          uuid conversation_id
          string content
          string sender_type
          uuid sender_id
          string message_type
      }
      Contact {
          uuid id
          uuid tenant_id
          string name
          string identifier
      }
  ```

  ### Core Components
  1. **Omnichannel Core Service (Rust):** A new module under `src/server/omnichannel/` handling CRUD for Inboxes, Channels, Conversations, and Messages.
  2. **WebSocket / Real-Time Gateway:** Push updates to connected mobile/web clients when new messages arrive.
  3. **Channel Adapters:** Interfaces for Web Widget, Instagram (Graph API), and WhatsApp (Cloud API) to translate vendor webhooks into OHC standard `Message` formats.
  4. **AI Triage Worker:** A background job that intercepts incoming messages, uses the Tenant context to draft a reply or create a task, and updates the Conversation status.

  ### Mobile UX Flow (375px)
  1. **Work Feed (Home):** The owner opens the app. The primary view is a unified inbox showing urgent messages across all channels.
  2. **Conversation View:** Tapping a message opens a clean, iMessage-style thread.
  3. **AI Assist:** Above the keyboard, a translucent "Drafting..." pill appears. The AI suggests a reply (e.g., "Yes, we have vegan options! I can add that to your order.") based on the business's knowledge base.
  4. **Action:** The owner can tap to approve the draft, edit it, or type manually.
  5. **Offline:** If Carlos is in a basement with no signal, he can type replies and hit send. The message goes to a local outbox and syncs when he reconnects.

  ### AI Agent Integration
  - **Work Triage Agent:** Listens to the `message.created` event queue. Analyzes intent.
  - **Customer Relationship Agent:** Drafts replies based on historical context and `Tenant` knowledge.
  - **Operations Agent:** If the message implies a booking ("Can you come fix my sink on Tuesday?"), it drafts a quote/booking action embedded in the chat UI.

  ## Implementation Prompt
  Implement the foundation for the native Rust Omnichannel Inbox.
  1. Create the database migrations for `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Ensure strict multi-tenant RLS (Row Level Security) with `tenant_id`.
  2. Build the Rust core data models and service layer in `src/server/omnichannel/` to support creating an inbox, starting a conversation, and sending a message.
  3. Implement a simple "Web Widget" channel adapter that can receive HTTP requests and spawn messages.
  4. Integrate with the existing AI job queue: when a new message is received from a contact, dispatch a `DraftReplyJob` for the AI assistant.
  5. **Acceptance Criteria:** A non-technical owner can receive a message from the web widget, see it in their unified database, and the system queues an AI triage job. All tests must pass (`bazel test //...`), and unit test coverage must be 100%.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
