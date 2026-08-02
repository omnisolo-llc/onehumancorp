issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently relies on an external Chatwoot deployment for multi-channel customer communication. This creates deployment complexity, fragmented state, and fails our goal of a unified, native backend for owners. Our non-technical users need a seamless, invisible omnichannel experience built directly into OHC without managing third-party webhook syncs or secondary platform integrations.

  ## Research Report
  Based on an audit of the Chatwoot source code (`app/models/`, `app/models/channel/`, `lib/`), modern helpdesk platforms, and our current Rust backend:
  - Chatwoot handles channels via separate adapter models (Email, WhatsApp, SMS, Web Widget, Twitter, etc.), passing all data through an abstraction layer into a unified Inbox and Conversation model.
  - Chatwoot manages real-time messaging using websockets with Redis PubSub.
  - We need a native Rust implementation that matches this capability to fulfill the "MANDATORY Complete Chatwoot Retirement" mandate.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Channels: SMS, Email, WhatsApp, Web Widget] -->|Webhooks/API| B(Channel Adapters)
      B --> C{Inbox Router}
      C --> D[Conversation Manager]
      D --> E[(Database: Messages, Conversations, Contacts)]
      D --> F[WebSocket Manager]
      F --> G(Owner UI / Tauri App)
  ```

  ### Data Model Invariants
  1.  **ChannelAdapter**: Defines the type (e.g., WhatsApp, Email) and holds configuration (API keys, endpoints) per tenant.
  2.  **Inbox**: A logical grouping of ChannelAdapters. A tenant can have multiple Inboxes (e.g., "Support", "Sales").
  3.  **Contact**: Universal representation of a customer, linked across channels.
  4.  **Conversation**: Represents a thread between a Contact and the Inbox. Contains multiple Messages.
  5.  **Message**: Individual chat bubbles, supporting attachments and rich text.

  ### Core Multi-Tenancy Rules
  - All database tables MUST include a `tenant_id` column.
  - Row-Level Security (RLS) MUST be enforced for all queries to ensure strict isolation.
  - Redis cache keys MUST follow the pattern `ohc:cache:{tenant_id}:{resource_type}:{resource_id}`.

  ### Mobile UX Flow (375px first)
  1.  **Home Tab**: Consolidated "Inbox" list showing unread messages across all channels, prioritized by agent triage.
  2.  **Conversation View**: Standard chat interface (bottom input, scrollable history). Clear visual indicator of which channel the message is from. Native keyboard integration.
  3.  **Action Menu**: Within a chat, the owner can trigger actions (create quote, schedule booking) directly from the conversation context.

  ### AI Integration Points
  - **Triage Agent**: Listens to new Conversations, categorizes them, and suggests the next action (or drafts a reply).
  - **Context Agent**: Provides a summary of the Contact's history and active orders alongside the chat.

  ## Implementation Prompt
  Implement the core database schema and Rust service layer for the native omnichannel chat system.
  - Define SeaORM entities for `ChannelAdapter`, `Inbox`, `Contact`, `Conversation`, and `Message`.
  - Ensure all entities enforce strict multi-tenancy with a `tenant_id` field.
  - Create the `ChatService` in Rust with methods to handle incoming messages, create conversations, and route them to the correct inbox.
  - Implement a basic WebSocket endpoint in Axum for real-time message delivery to connected clients.
  - Do NOT implement the specific third-party integrations (Twilio, WhatsApp) yet; focus on the unified abstraction layer and database models.

  Acceptance Criteria:
  1. All SeaORM models are created and tests pass.
  2. `ChatService` can create and retrieve conversations and messages.
  3. WebSocket connection can be established and receives a simple ping.
  4. 100% unit test coverage for the new service layer.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
