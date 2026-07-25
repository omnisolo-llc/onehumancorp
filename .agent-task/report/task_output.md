issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as a third-party dependency for handling omnichannel customer communications. The current system lacks a native, high-performance, and tightly integrated chat system that aligns with OHC's multi-tenant architecture and mobile-first, Zero-Trust philosophy. Maya, Carlos, Priya, Leo, and Fatima need a seamless way to interact with customers across various channels (Instagram, WhatsApp, Web Widget, Email) directly from the OHC app, without relying on external services that compromise data sovereignty or performance.

  ## Research Report
  - **Chatwoot Source Code Audit**:
    - **Data Models**: Chatwoot uses `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, and specific `Channel` models (e.g., `Channel::Email`, `Channel::Whatsapp`).
    - **Channels**: Supports various channels like Web Widget, API, Email, Facebook Page, Instagram, Line, SMS, Telegram, Twilio SMS, Twitter Profile, and WhatsApp.
    - **Architecture**: Ruby on Rails backend with PostgreSQL and Redis (ActionCable for WebSockets).
  - **OHC Gaps**: OHC currently lacks native Rust implementations for these core chat entities, channel adapters, and the real-time WebSocket infrastructure required for a seamless mobile and web chat experience.
  - **Competitive Analysis**: Systems like Shopify Inbox and Stripe Customer Portal offer deeply integrated, native communication tools. OHC must replicate this native feel but with a broader omnichannel reach.

  ## Design Doc
  ### Architecture & Data Model (Native Rust)
  We will implement a high-performance, multi-tenant chat system in Rust within `onehumancorp/mono`.

  **Key Entities (PostgreSQL via Rust/SQLx):**
  - **`Tenant`** (existing): The business owner account.
  - **`Contact`**: A customer. Includes identifier, email, phone, custom attributes. Multi-tenant isolated.
  - **`Inbox`**: A grouping of conversations (e.g., "Main Support", "Sales"). Associated with a specific `Channel`.
  - **`Conversation`**: A thread between a `Contact` and an `Inbox`. Tracks status (open, closed, snoozed), assignee, and last activity.
  - **`Message`**: Individual messages within a `Conversation`. Supports attachments, text, rich content.
  - **`ChannelAdapter`**: Configuration for specific channels (e.g., Instagram Graph API creds, WhatsApp Business API config).

  **Architecture Diagram (Mermaid.js):**
  ```mermaid
  erDiagram
      Tenant ||--o{ Contact : has
      Tenant ||--o{ Inbox : has
      Inbox ||--o{ ChannelAdapter : configured_with
      Contact ||--o{ Conversation : participates_in
      Inbox ||--o{ Conversation : contains
      Conversation ||--o{ Message : has
      User ||--o{ Conversation : assigned_to
  ```

  ### Mobile UX Flow (375px First)
  1. **Omni-Inbox View**: A unified, fast-loading list of all active conversations across all channels, grouped by Inbox or priority. Translucent glass headers, clear unread indicators.
  2. **Conversation View**: Native-feeling chat interface. Message bubbles, typing indicators, quick reply options (AI drafted).
  3. **Contact Context**: A drawer/panel accessible from the conversation showing the customer's history, active orders, and custom attributes.

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Listens to new `Message` events via the internal event bus. Automatically drafts replies for incoming messages based on context, knowledge base, and past interactions. The drafted reply is presented to the owner for approval or auto-sent based on rules.
  - **Work Triage**: Analyzes new conversations and groups/prioritizes them in the owner's feed.

  ## Implementation Prompt
  Implement the core Rust data models, database migrations, and basic CRUD APIs for the native Omnichannel Chat System.
  1. **Database Migrations**: Create tables for `contacts`, `inboxes`, `conversations`, and `messages` with strict row-level security (RLS) based on `tenant_id`.
  2. **Rust Models**: Define the corresponding Rust structs in `src/server/domain/` or appropriate module.
  3. **Core API Endpoints**: Implement REST/gRPC endpoints to list conversations, fetch messages, and send messages.
  4. **Tests**: 100% unit test coverage for the models and endpoints. Must pass `bazel test //...`.
  5. **UX**: No UI implementation yet, focus on backend foundation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
