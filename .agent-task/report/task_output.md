issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC previously relied on Chatwoot as an external dependency for omnichannel messaging. This introduced latency, data privacy concerns (multi-tenant isolation leaks), and broke our Zero Trust architecture. Non-technical owners (like Maya and Carlos) need a seamlessly integrated inbox that handles Instagram DMs, WhatsApp, SMS, and Web Chat without leaving the OHC interface or configuring third-party integrations.

  ## Research Report
  - **Competitor Analysis**: Leading platforms like Shopify (Shopify Inbox) and HubSpot unify messaging internally. By owning the messaging layer, they can apply AI directly to the data stream for cart recovery, quoting, and customer support.
  - **Chatwoot Source Code Audit**: Investigating Chatwoot (`https://github.com/chatwoot/chatwoot`), the core architecture relies on:
    - **Inboxes/Channels**: Adapters for different platforms (Web Widget, API, Email, SMS, WhatsApp). E.g. `app/models/inbox.rb` (channel_type, account_id) and `app/models/channel/api.rb` (webhook_url, hmac_mandatory).
    - **Conversations**: Central entity tracking message threads. E.g. `app/models/conversation.rb` (status, identifier, inbox_id, contact_id, account_id).
    - **Messages**: Individual payloads with rich media support and read receipts. E.g. `app/models/message.rb` (content, message_type, conversation_id, sender_id, account_id).
    - **Contacts**: Omnichannel identity resolution. E.g. `app/models/contact.rb` (email, phone_number, identifier, account_id) and `app/models/contact_inbox.rb` (contact_id, inbox_id, source_id).
    - **WebSocket Real-time Messaging**: For live presence and typing indicators.
  - **OHC Gap Analysis**: We currently lack a native Rust implementation for this data model. We need a performant, multi-tenant inbox system built directly into OHC inside `onehumancorp/mono`.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      CONTACT ||--o{ CONTACT_INBOX : has
      MESSAGE {
          uuid id
          text content
          enum status
      }
      CONTACT {
          uuid id
          string email
          string phone_number
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen**: A clean, list-based view showing unread conversations. Badges for Instagram, WhatsApp, etc.
  - **Conversation Thread**: Apple Messages-style bubbles with clear timestamps.
  - **Action Bar**: Persistent bottom bar to type replies, trigger AI drafts (Operations Assistant), or insert quick quotes.

  ### AI Agent Integration
  - **Work Triage**: Analyzes incoming messages and groups them into prioritized tasks in the owner's feed.
  - **Customer & Relationship Assistant**: Drafts replies automatically based on previous context and business rules.

  ### Key Design Decisions
  - **Rust Native**: High-performance Rust microservice/crate to handle high-concurrency WebSockets and data modeling.
  - **Multi-Tenant Isolation**: Row-Level Security (RLS) in PostgreSQL with `tenant_id` mandatory on all queries (replacing Chatwoot's `account_id`).

  ## Implementation Prompt
  **User Facing Outcome**: The owner can open the OHC app and see a unified inbox of all customer messages from Instagram, WhatsApp, and Web Chat, and reply directly or have the AI draft responses.

  **CUJ**:
  1. Owner logs into OHC on mobile (375px).
  2. Owner navigates to the "Inbox" tab.
  3. Owner sees a new Instagram DM from a customer.
  4. Owner taps the DM and reads the message.
  5. Owner taps "AI Draft" and sends the AI-generated reply.

  **Acceptance Criteria**:
  - Rust models for Inbox, Channel, Conversation, Message, Contact, and ContactInbox.
  - PostgreSQL schema with strict `tenant_id` RLS for all new tables.
  - WebSocket handler for real-time message delivery.
  - 100% Unit Test coverage for the new models.
  - E2E Playwright test proving the Inbox CUJ.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
