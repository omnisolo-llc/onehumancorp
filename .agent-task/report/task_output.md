issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Title: Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot as an external third-party service for omnichannel customer communications. Relying on an external service breaks our multi-tenant Zero Trust architecture, introduces latency, and fractures the AI assistant's memory context. For owner/operators like Maya (who manages cake orders via Instagram DMs) and Carlos (who manages service quotes via SMS/WhatsApp), a unified, instantaneous, and deeply integrated inbox is critical. We must retire Chatwoot completely and build a native Rust omnichannel chat system within `onehumancorp/mono`.

  ## Research Report
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals its core architecture centers around the following entities:
  - **Account**: The multi-tenant boundary.
  - **Inbox**: The routing destination for specific channels (e.g., WhatsApp, Email, Web Widget).
  - **Contact**: The unified customer profile spanning multiple inboxes.
  - **Conversation**: The specific thread of messages between a contact and an inbox.
  - **Message**: Individual payloads (text, attachments, template messages).
  - **Channel Adapters**: Interfaces for Facebook, Twitter, WhatsApp, API, etc.

  **Comparison**: Shopify Ping, Wix Inbox, and HubSpot all utilize unified inbox data models to abstract the underlying channel. OHC's implementation will use Rust for high concurrency, WebSocket real-time event distribution, and PostgreSQL for strict row-level security (RLS) tenant isolation, natively bridging the gap between operations and customer communications.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Account ||--o{ Inbox : owns
      Account ||--o{ Contact : owns
      Account ||--o{ Conversation : owns
      Contact ||--o{ Conversation : participates
      Inbox ||--o{ Conversation : routes
      Conversation ||--o{ Message : contains
      Inbox ||--o{ ChannelAdapter : configured_with
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home)**: The owner opens the app. Unread conversations bubble up in the AI-prioritized action list.
  2. **Unified Inbox List**: Swipeable list items showing contact avatar, channel icon (e.g., IG, SMS), and AI-generated summary of the last message. Touch targets are 44x44px.
  3. **Conversation View**:
     - Translucent glass sticky header with Contact Name and quick actions (Quote, Book, Tag).
     - Scrollable message thread.
     - Bottom input area: Native mobile keyboard integration, prominent "Magic Draft" button.
     - Offline tolerance: Messages sent offline are queued and marked with a pending clock icon, utilizing local PWA/Flutter storage.

  ### AI Agent Integration Points
  - **Customer Assistant Agent**: Automatically drafts replies upon receiving new `Message` webhooks. Context is built from the `Contact`'s previous interactions and active `Conversation`.
  - **Work Triage Agent**: Evaluates incoming messages to extract intent (e.g., "quote request") and auto-labels the `Conversation` and alerts the owner.

  ### Key Design Decisions
  - **Multi-Tenancy**: Every table must include `tenant_id` with PostgreSQL Row Level Security (RLS) enabled.
  - **Real-Time Distribution**: Rust-based WebSocket server with Redis Pub/Sub for horizontal scaling and lock coordination (Redlock).
  - **Data Types**: Use `jsonb` for channel-specific metadata in `Message` and `Contact` to maintain a flexible schema without adapter-specific tables.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core data model and backend services for the native Rust omnichannel chat system. Your goal is to replicate the core Chatwoot inbox experience natively inside OHC.
  - Set up the PostgreSQL tables for `inboxes`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` RLS is enforced.
  - Build the Rust gRPC/REST APIs to list conversations, fetch messages, and create new messages.
  - Implement a basic WebSocket pub/sub mechanism to emit `message.created` events to connected clients.
  - Do not worry about specific channel integrations (like WhatsApp or IG) yet; focus on the core API and a mock/sandbox channel adapter.
  - Create at least 5 Playwright E2E tests verifying that an owner can view a conversation, send a message, and receive a WebSocket update using the standard OHC mobile UI layout.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
