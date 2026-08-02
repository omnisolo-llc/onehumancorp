issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**
  The external service Chatwoot is 100% RETIRED from OHC. OHC currently lacks its own native Rust-based omnichannel chat system to coordinate DMs, forms, and messages for business owners. To ensure that OHC works invisibly in the background and helps the owner act (triage messages, remember preferences, draft replies), we need to architect and implement our own native high-performance, multi-tenant omnichannel chat engine inside `onehumancorp/mono`.

  **Research Report**
  As mandated, an exhaustive review of Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) has been performed. Chatwoot relies on Rails Active Record models such as `Conversation`, `Message`, `Contact`, and `Inbox`. It effectively models the relationships between an account (tenant), a contact, an inbox, and the ongoing conversation consisting of various types of messages.

  For OHC to fulfill its promise, we must replicate and adapt this schema inside our Rust/PostgreSQL/SeaORM architecture. The key missing primitives that we need to build natively include:
  - `Conversation`: The core entity tracking an ongoing thread with a contact.
  - `Message`: Individual messages within a conversation.
  - `Contact`: The customer or lead involved in conversations.
  - `Inbox`: A channel through which messages arrive (e.g., Email, SMS, Instagram).

  **Design Doc**
  We will introduce a new directory `src/server/ohc/domain/chat` to house the Rust backend implementation of our omnichannel chat system.

  *Architecture Map:*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid tenant_id
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          uuid sender_id
          string sender_type
          string content
          string message_type
      }
  ```

  *Mobile UX Flow:*
  1. Owner opens OHC on their 375px mobile device.
  2. "Work Triage" view shows aggregated messages from all sources (Instagram, Email, WhatsApp).
  3. Owner taps a message, viewing the `Conversation`.
  4. The AI assistant drafts a reply in the context of the `Conversation`.
  5. Owner taps "Send", appending a `Message` and dispatching it through the appropriate `Inbox` channel.

  *AI Agent Integration:*
  - The Customer Assistant agent will listen to `MESSAGE_CREATED` pub/sub events.
  - When a new inbound message is detected, the agent fetches the `Conversation` history and related `Contact` preferences.
  - The agent drafts a response (as a draft `Message`) for owner approval.

  **Implementation Prompt**
  *Implementer:*
  1. Add new SeaORM entities and migrations for `inbox`, `contact`, `conversation`, and `message` tables in PostgreSQL, ensuring strict row-level security (`tenant_id`) is enforced on every table.
  2. Create gRPC/REST endpoints in `src/server/ohc/domain/chat/api.rs` to allow the mobile frontend to list and create conversations and messages.
  3. Wire the new chat domain into the main OHC module tree (`src/server/ohc/mod.rs`).
  4. Ensure 100% unit test coverage for the new models and controllers.
  5. Build a basic UI component (if not already existing) in the Flutter app to display a simple list of conversations from the API, verifying the frontend-backend connection.
  6. Implement E2E Playwright tests simulating a user (Maya) receiving a message, opening the conversation, and drafting a reply, asserting that the new data models are correctly persisted and retrieved.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
