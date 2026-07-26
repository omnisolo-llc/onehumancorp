issue_title: "Architecture: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ### Architecture: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)

  **Problem Statement**
  Currently, OHC lacks a native, deeply integrated omnichannel communication engine. Chatwoot has been explicitly retired as an external dependency. Our personas—Maya (Instagram DMs), Carlos (SMS/Phone), Priya (Email), and Fatima (WhatsApp)—need a single, unified inbox to talk to their customers across all channels without switching apps. We must build a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust, directly embedded within the OHC platform to guarantee Zero-Trust isolation, seamless AI agent coordination (Operations, CS, Sales), and sub-50ms latency.

  **Research Report**
  An audit of the `chatwoot/chatwoot` source repository (`app/models`) reveals the core architectural primitives required for an omnichannel inbox:
  1. **Accounts (Tenants)**: The root of isolation.
  2. **Inboxes**: Grouping mechanism for channels (e.g., "Support", "Sales").
  3. **Channels**: Adapters for specific platforms (`web_widget`, `api`, `whatsapp`, `telegram`, `sms`, `email`, `instagram`, `facebook`).
  4. **Contacts**: The end-user communicating with the business.
  5. **Conversations**: A thread between a Contact and an Inbox via a specific Channel.
  6. **Messages**: Individual payloads (text, attachments, templates) within a Conversation.
  7. **Agent Bots/Automation Rules**: Triggers for AI intervention.

  Building this in Rust within `src/server/ohc/domain/chat` requires:
  - **Data Layer**: PostgreSQL with strict Row-Level Security (RLS) on `tenant_id`.
  - **Real-time Layer**: WebSockets for live UI updates, handled by Rust async workers.
  - **Queueing Layer**: Background jobs (via PostgreSQL `SKIP LOCKED` or Redis) to handle external API rate limits and webhook processing asynchronously.

  **Design Doc**

  *Mobile UX Flow (375px First):*
  - **Inbox View**: A unified list of active conversations, styled with Translucent Glass. Each row shows the contact avatar, channel icon (e.g., WhatsApp, Insta), preview, and timestamp.
  - **Conversation View**: Chat bubbles. Customer on left, Owner (or AI Agent) on right. Bottom input bar with native keyboard, attachment icon, and an "AI Draft" toggle.
  - **Handoff**: Clear visual indicator when an AI Agent is handling the chat vs. waiting for Owner approval.

  *AI Agent Integration:*
  - **CS Agent**: Listens to `message.created` webhooks internally. If the conversation is unassigned, the CS Agent processes the text and drafts a reply.
  - **Sales Agent**: If intent involves pricing or quoting, the Sales Agent joins the thread to generate an actionable quote card.

  *Architecture Diagram:*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CHANNEL : configures
      CHANNEL ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

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
          uuid inbox_id
          string provider
          jsonb credentials
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
          uuid contact_id
          uuid inbox_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type
          uuid sender_id
      }
  ```

  **Implementation Prompt**
  Implement the Native Rust Omnichannel Chat system in `src/server/ohc/domain/chat`.
  1. **Data Models**: Create the Rust structs and traits for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`, ensuring all include a `tenant_id` for RLS.
  2. **Core Service API**: Implement a Rust service layer that allows creating a conversation, routing incoming messages from a webhook to the correct inbox, and dispatching outgoing messages to the respective channel adapter interface.
  3. **Channel Adapter Trait**: Define a `ChannelAdapter` trait with `send_message(message: &Message)`. Implement a mock or `LocalApi` adapter first for testing.
  4. **Acceptance Criteria**: A complete end-to-end test where a webhook payload creates a `Message`, updates a `Conversation`, and triggers a local event without any Chatwoot dependencies. All code must have 100% unit test coverage.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
