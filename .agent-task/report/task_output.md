issue_title: "Architecture Design: Native Rust Omnichannel Chat System Replication (Chatwoot Feature Parity)"
issue_description: |
  # Native Rust Omnichannel Chat System Replication

  ## Problem Statement
  OHC has completely retired the external Chatwoot dependency, opting for a high-performance, native Rust omnichannel chat system within `onehumancorp/mono`. While the initial database schema (`217_native_omnichannel_chat.sql`) and foundational Rust services (`src/server/services/chat`) have been laid down, OHC lacks feature parity with Chatwoot. Specifically, OHC is missing deep channel adapter integration (e.g., WhatsApp, Instagram, Telegram, SMS, Email, Line, Facebook Page, Web Widget, etc.), real-time WebSocket messaging pipelines, and AI agent background coordination. For owner personas like **Maya** (Instagram DM cake orders) or **Carlos** (SMS repair quotes), this gap means they cannot currently manage all their demand in a unified OHC feed reliably.

  ## Research Report
  - **Chatwoot Source Audit:** We conducted a clone and deep audit of Chatwoot (`https://github.com/chatwoot/chatwoot`). Key architectural components discovered include:
    - **Models:** Omnichannel logic spans `Account`, `Inbox`, `Channel::*` (API, Email, FacebookPage, Instagram, Line, SMS, Telegram, TikTok, Twilio, Twitter, WebWidget, Whatsapp), `Contact`, `Conversation`, `Message`, `AgentBot`, and `AutomationRule`.
    - **WebSocket Real-time layer:** A robust action-cable based push system for live typing, presence, and instant message delivery.
    - **Multi-tenancy:** Uses `account_id` heavily, mapping to OHC's `tenant_id`.
  - **OHC Implementation Gap:** OHC's `src/server/migrations/217_native_omnichannel_chat.sql` correctly sets up Row Level Security (RLS) on `tenant_id` for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages`. However, the actual adapters to synchronize data with upstream social APIs and webhooks are incomplete. We lack the AI Department coordination (e.g., "Customer Success" agent auto-replying).

  ## Design Doc
  ### Data Model & Invariants
  - **Multi-tenant Strict Isolation:** Every table MUST use `tenant_id` with Postgres `ENABLE ROW LEVEL SECURITY` and `current_setting('app.current_tenant_id')`.
  - **Core Entities:**
    - `ChatInbox`: Logical container for channels.
    - `ChatChannel`: The channel adapter instance (WhatsApp, Instagram, etc.). Contains JSONB `config`.
    - `ChatConversation`: Binds a `ChatContact` to a `ChatInbox`.
    - `ChatMessage`: The immutable event representing a message in a conversation.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ChatInbox ||--o{ ChatChannel : "has"
      ChatInbox ||--o{ ChatConversation : "contains"
      ChatContact ||--o{ ChatConversation : "participates"
      ChatConversation ||--o{ ChatMessage : "has many"
      ChatChannel {
          UUID id
          UUID tenant_id
          String channel_type
          JSONB config
      }
      ChatConversation {
          UUID id
          UUID tenant_id
          String status
      }
      ChatMessage {
          UUID id
          UUID tenant_id
          String sender_type
          String content
      }
  ```

  ### AI Department Coordination
  - **Work Triage Agent:** Ingests the `ChatMessage` webhook. If it's a new `ChatConversation`, determines urgency.
  - **Customer & Relationship Assistant:** Drafts standard replies to Instagram DMs or Web Widget chats (e.g., "Do you do vegan cakes?"). Stores draft in `ChatMessage` with a pending status until owner approval.
  - **Operations Assistant:** Parses intents like "I need a quote for repair" to coordinate booking tasks and sync context to Carlos's feed.

  ### Mobile-First UX Flow
  - **375px Target:** The inbox screen shows a consolidated vertical feed of unread messages across all channels.
  - **Translucent Glass UI:** Message bubbles use macOS-style translucent glass backgrounds for a premium feel.
  - **Offline/Flaky Network:** Uses local PowerSync SQLite replica to render chat history immediately. Outbound messages are queued locally and synchronized when online.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Native Rust Omnichannel Chat Webhooks and Channel Adapters for OHC.
  1. Build a Rust REST endpoint (e.g., `/api/v1/webhooks/omnichannel/:channel_type`) to receive inbound messages from external providers (e.g., Meta/Instagram).
  2. Map these inbound webhooks to the `ChatService::send_message` and `ChatService::start_conversation` functions in `src/server/services/chat/service.rs`.
  3. Emit a real-time WebSocket event or PowerSync invalidation upon new message creation so the Flutter/PWA UI updates instantly on a 375px mobile screen.
  4. Ensure zero-trust multi-tenancy: The webhook processor must securely map the provider's payload to the correct `tenant_id` before inserting.
  5. **Acceptance Criteria:** A simulated inbound webhook from Instagram creates a `ChatMessage` in Postgres, correctly scoped to a `tenant_id`, and appears in the `/inbox` UI seamlessly.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
