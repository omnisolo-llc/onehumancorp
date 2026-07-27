issue_title: "Implement Native Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol Brief
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native omnichannel chat and customer support architecture. Historically, OHC might have relied on or planned to rely on external systems like Chatwoot, which is strictly forbidden by the `Chatwoot Retirement & Custom Rust Omnichannel Chat System Standard`.

  Small business owners (like Maya the Baker or Carlos the Handyman) need a single, unified inbox to manage customer inquiries across channels (Instagram DMs, Web Chat, Email, SMS) without switching apps. They need to seamlessly transition from answering a question to booking a service or capturing a lead.

  We need to build a native Rust multi-tenant omnichannel chat engine inside OHC that achieves feature parity with Chatwoot's core inbox capabilities, tailored to the OHC owner-first philosophy.

  ## Research Report
  - **Chatwoot Source Code Audit**:
    - Audited Chatwoot's core data models (`conversations`, `messages`, `inboxes`, `contacts`, `channel_*`).
    - Chatwoot uses polymorphic associations heavily (e.g., `sender_type` and `sender_id` on `Message`).
    - Multi-tenancy is handled via `account_id` on almost every record.
    - Webhooks and WebSocket events drive the real-time UI.
  - **OHC Gaps**:
    - OHC needs robust Rust microservices/crates for handling incoming messages from various channel providers (Meta Graph API, Twilio, Resend, etc.).
    - Needs a multi-tenant unified `Conversation` and `Message` data model with strict row-level security (RLS) via `tenant_id`.
    - Needs AI Agent integration hooks so the "Customer Assistant" can auto-draft replies or triage messages based on context.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ ChannelAdapter : configures
      Contact ||--o{ Conversation : has
      Inbox ||--o{ Conversation : holds
      Conversation ||--o{ Message : contains
      Message }o--|| Sender : sent_by

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean is_active
      }
      ChannelAdapter {
          uuid id PK
          uuid inbox_id FK
          string channel_type "web, instagram, sms"
          jsonb credentials
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          string summary
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string sender_type "contact, agent, ai"
          uuid sender_id
          timestamp created_at
      }
  ```

  ### System Components
  1. **Omnichannel Core (Rust)**:
     - `ConversationService`: Manages lifecycle (create, resolve, snooze, assign).
     - `MessageService`: Handles message delivery, persistence, and real-time event emission.
  2. **Channel Adapters**:
     - Webhooks from external providers (Meta, Twilio) are normalized into the OHC standard `Message` format.
  3. **Real-time Engine**:
     - WebSocket server (Rust) pushing events (`message.created`, `conversation.updated`) to authenticated clients.
  4. **AI Assistant Integration**:
     - Whenever a new `Message` is created by a contact, a background job is enqueued for the AI Customer Assistant to analyze intent, extract context, and draft a reply.

  ### Mobile UX Flow (375px)
  1. **Unified Inbox Screen**: A clean, scrollable list of active conversations. Unread indicators clearly visible. No technical jargon. Filter by "Needs Action" vs "Resolved".
  2. **Conversation View**: Similar to standard messaging apps (iMessage/WhatsApp style). The key addition is an "Assistant Bar" below the input field suggesting the next best action (e.g., "Draft Reply: Yes, we do vegan cakes", or "Send Deposit Link").
  3. **Contact Context Sheet**: Swiping left or tapping a contact icon brings up a bottom sheet with the customer's history, previous orders/bookings, and preferences.

  ## Implementation Prompt
  Implement the core data schema and Rust service layer for the native OHC Omnichannel Chat system.

  **Requirements**:
  1. Define PostgreSQL migrations for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages`. All tables must use `tenant_id` for multi-tenancy and have strict RLS policies enabled.
  2. Implement the Rust backend services (`ConversationService`, `MessageService`) to handle CRUD operations and state transitions for these entities.
  3. Ensure that when a new message is created, a real-time event payload is prepared (to be consumed by a WebSocket server layer).
  4. **CUJ to verify**: As an owner, I should be able to create a Web Inbox, receive a message from a new Contact, view it in a Conversation, and send a reply Message back.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
