issue_title: "Implement Native Rust Omnichannel Chat System (ExternalChat Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems (like ExternalChat) for omnichannel customer communication, which introduces latency, multi-tenancy risks, and fragments the unified owner experience. Non-technical owners (like Maya the baker and Carlos the handyman) need a seamless, instantaneous inbox where DMs, emails, and SMS messages appear in a single unified thread inside OHC. The reliance on third-party integrations violates the "One Human Corp" promise of a fully integrated, lightning-fast work assistant and limits our ability to inject AI agents directly into the chat stream with full tenant context.

  ## Research Report
  - **ExternalChat Source Code Audit**: An analysis of the ExternalChat `db/schema.rb` reveals core entities necessary for an omnichannel inbox:
    - `conversations`: Tracks the thread lifecycle across channels, bounded by `account_id` and `inbox_id`.
    - `messages`: Stores individual chat payloads, maintaining sender context, content type, and metadata, scoped to `conversation_id`.
    - `inboxes` & `channel_*` tables (e.g., WhatsApp, Email, Web Widget): Act as adapters parsing incoming external payloads into normalized messages.
  - **Competitive Analysis**: Shopify Inbox and WeCom excel because their chat infrastructure is native to the platform, meaning order data, inventory, and chat context share the exact same database transaction space.
  - **Identified Gap**: OHC requires a native Rust implementation of the ExternalChat data model and WebSocket delivery system to guarantee multi-tenant row-level security (RLS) in PostgreSQL, sub-10ms message delivery, and real-time AI agent interjection.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : holds
      CONVERSATION }o--|| CONTACT : involves

      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          string content
          enum message_type
          jsonb metadata
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          enum status
          uuid contact_id
      }
  ```

  ### Mobile-First UX Flow (375px First)
  1. **Triage Feed (Home Screen)**: Unified list of active conversations with unread indicators and AI-generated summary snippets (e.g., "Maya, this customer is asking about vegan options").
  2. **Conversation View**: Full-screen chat interface on mobile. Custom UI components for rich message types (e.g., displaying a quoted order or a generated payment link directly in the chat bubble).
  3. **Agent Action Tray**: A persistent sticky action bar at the bottom allowing the owner to tap to "Generate Quote", "Request Deposit", or "Approve AI Draft" with zero typing.

  ### AI Agent Integration
  - **Operations & Customer Agents**: Agents listen to the real-time event bus (NATS/Redis PubSub). When a new message arrives, the `CustomerAssistant` reads the `conversation_id` context, drafts a potential reply, and stores it as an internal AI draft message. The UI renders this draft with a glowing "Approve" button for the owner.

  ## Implementation Prompt
  Implement the core native Rust omnichannel chat system inside `onehumancorp/mono`.
  - **CUJ**: A business owner (Maya) receives an Instagram DM. The message appears instantly in her OHC mobile app. An AI agent drafts a response. Maya taps "Approve" to send the reply back to Instagram.
  - **Acceptance Criteria**:
    - Rust data models (SeaORM/SQLx) for `Conversations`, `Messages`, and `Inboxes` with strict row-level security for `tenant_id`.
    - Real-time WebSocket delivery of messages to the Flutter PWA client.
    - An adapter interface for external channels (mocked for this iteration with an HTTP webhook endpoint).
    - AI Agent hook allowing a background worker to append draft messages to a conversation.
    - A 375px-optimized Flutter chat interface displaying the conversation stream.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []