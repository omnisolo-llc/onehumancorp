issue_title: "Omnichannel Inbox Architecture & Adapter Strategy"
issue_description: |
  # Problem Statement
  Small business owners and operators like Maya the baker and Carlos the handyman receive customer demand across scattered channels—Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Currently, this fragmentation causes them to lose leads, miss follow-ups, and waste time context-switching. They need a unified, native "Inbox" that aggregates all customer communications into one view, allowing the OHC Assistant to triage messages, draft replies, and link conversations directly to quotes or bookings.

  # Research Report
  - **Market Context**: Platforms like HubSpot, Intercom, and Shopify Inbox provide unified messaging, but they often expose technical setup to the user (API keys, webhook routing). Operator tools like WeCom/Lark prioritize extreme multi-channel integration without cognitive overload.
  - **Codebase Findings**: Initial hooks for `omni_inbox_messages` exist in `src/server/db.rs` and an `omnichannel_service.rs` stub, but the core routing architecture, multi-tenant boundaries, and real-time (WebSocket) delivery mechanism are underdeveloped.
  - **Competitor Insights**: Best-in-class omnichannel inboxes separate the "Channel Adapter" (which normalizes incoming webhook payloads) from the "Unified Inbox" (which handles state, read receipts, and UI updates). This prevents platform-specific logic from leaking into the core application.

  # Design Doc
  ## System Architecture
  ```mermaid
  graph TD
      A[External Webhooks: IG, WA, SMS] -->|Webhook Payload| B(Channel Adapters)
      B -->|Normalized Message| C[Omnichannel Router]
      C -->|Persist| D[(PostgreSQL: omni_inbox_messages)]
      C -->|Event| E[Redis Pub/Sub]
      E -->|Real-time| F[WebSocket Gateway]
      F -->|Push| G[Flutter Client]
      C -->|Trigger| H[AI Triage Agent]
      H -->|Drafts Reply/Action| D
  ```
  - **Channel Adapters**: Native Rust handlers that ingest raw payloads from Stripe, Meta (WhatsApp/IG), and Twilio, normalizing them into a standard `ConversationEvent` struct.
  - **Data Model**: `Conversation` (tenant_id, channel, customer_id), `Message` (conversation_id, sender_type, content, status). Strict RLS via `tenant_id`.
  - **Real-Time Engine**: WebSocket connections authenticated via SPIFFE/SPIRE, subscribing to Redis channels scoped by `tenant_id`.

  ## Mobile UX Flow (375px)
  1. **Home Command Center**: Notification badge on "Inbox" tab.
  2. **Unified Inbox List**: List of active conversations, clearly badged with channel icons (e.g., 🟢 WhatsApp, 📸 Instagram). Unread messages are bold.
  3. **Conversation View**: Chat bubbles. At the bottom, standard composer *plus* a persistent "AI Draft" suggestion based on context.
  4. **Action Sidebar/Drawer**: Swipe left on a conversation to reveal quick actions: "Create Quote", "Book Appointment".

  ## AI Agent Integration Points
  - **Triage Hook**: Every new inbound message triggers the `Operations Assistant` to evaluate urgency and extract intent (e.g., "Wants a cake on Tuesday").
  - **Draft Hook**: For standard inquiries, the `Customer Assistant` drafts a reply and stores it with `status=DRAFT`, ready for 1-tap approval by the owner.

  # Implementation Prompt
  Implement the core native Rust Omnichannel Inbox architecture.
  1. **Data Layer**: Formalize the `Conversation` and `Message` entities with RLS enforcing `tenant_id` isolation.
  2. **Adapter Interface**: Create a `ChannelAdapter` trait and implement a basic generic webhook intake that normalizes external payloads.
  3. **Real-Time Delivery**: Implement a WebSocket handler that pushes new messages to the connected client.
  4. **Acceptance Criteria**: A simulated webhook payload must result in a new persisted message, a WebSocket event sent to the correct tenant, and the AI agent triggered to generate a draft reply.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
