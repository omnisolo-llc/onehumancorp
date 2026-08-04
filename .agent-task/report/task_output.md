issue_title: "Design: Native Rust Omnichannel Chat Foundation"
issue_description: |
  # Native Rust Omnichannel Chat Foundation

  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support engine built natively in Rust. Previously, Chatwoot was considered for integration but is now 100% RETIRED due to architectural bloat, lack of zero-trust isolation, and misalignment with our mobile-first, AI-native goals. Owners like Maya (baker managing Instagram DMs) and Carlos (field service scheduling via SMS) need a unified inbox that is completely transparent to them, consolidating demand from all channels into actionable tasks without an external chat platform.

  ## Research Report
  Based on an exhaustive audit of the `chatwoot/chatwoot` source codebase and OHC's current `src/server/services/chat` directory:
  - **Chatwoot's Approach**: Relies on a monolithic Rails application with heavy data models (`Conversation`, `Message`, `Contact`, `Inbox`, `Account`). Channel adapters are deeply intertwined. WebSocket presence relies on ActionCable and Redis, which introduces operational complexity for OHC's self-contained cloud/desktop parity.
  - **OHC Native Alignment**: Our existing Rust foundations in `src/server/services/chat/models.rs` define lightweight entities (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) with row-level security (`tenant_id`). However, they lack robust omnichannel channel adapters, SLA management, and secure webhook ingress for platforms like Meta, Twilio, and Resend.
  - **Market Context**: Shopify Inbox and Wix Chat provide native solutions that are closely tied to commerce capabilities. By building this natively in Rust, OHC can leverage our `Agent Department` architecture to instantly translate incoming WhatsApp/Instagram messages into active workflows (e.g., automated quoting, booking calendars).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ ChatInbox : owns
      ChatInbox ||--o{ ChatChannel : configures
      ChatChannel ||--o{ WebhookIngress : accepts
      Tenant ||--o{ ChatContact : manages
      ChatContact ||--o{ ChatConversation : initiates
      ChatInbox ||--o{ ChatConversation : routes
      ChatConversation ||--o{ ChatMessage : contains

      ChatMessage {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          uuid sender_id
          string content
          jsonb metadata
          string status
      }

      ChatChannel {
          uuid id
          uuid tenant_id
          uuid inbox_id
          string channel_type
          jsonb provider_config
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View**: A translucent glass tab bar routes the owner to the "Unified Inbox". Unread messages are clustered by urgency, not just chronological order.
  - **Conversation View**: Full-screen message thread. The bottom input area supports native mobile keyboards.
  - **Agent Intervention**: AI-drafted replies appear as "Ghost Text" or a translucent "Suggested Action" card above the keyboard. One tap sends the AI-generated quote or booking link.

  ### AI Agent Integration
  - **The Promoter/Operations Agent**: Subscribes to `ChatMessageCreated` events. If a message contains intent to book or purchase, the agent generates a draft reply and queues a side-effect (e.g., `CreateQuoteTask`).
  - **Distributed Locks**: Uses Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to prevent multiple agents from replying simultaneously.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to solidify the Native Rust Omnichannel Chat foundation.
  1. Expand the PostgreSQL schemas in `src/server/services/chat/models.rs` to support secure Webhook Ingress and delivery state tracking (Outbox pattern).
  2. Implement strict row-level security using `tenant_id` for all new queries.
  3. Construct the API boundary (`src/server/api/chat.rs`) to expose these endpoints to the Next.js/Flutter frontend.
  4. Build the core Flutter UI components for the "Unified Inbox" using OHC Premium Tokens (translucent materials, clean spacing) targeting a 375px viewport.
  5. The Critical User Journey (CUJ): A business owner (Carlos) receives a new SMS lead, sees it in the Unified Inbox, reviews an AI-drafted reply, and taps "Send" with no UI mock data used at any stage. Ensure 100% unit and Playwright E2E coverage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
