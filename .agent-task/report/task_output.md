issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  As of our latest architectural review, Chatwoot as an external third-party service/dependency is 100% RETIRED. Small business owners (like Maya the Baker or Carlos the Handyman) rely heavily on communicating across multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat) to coordinate work, capture leads, and serve customers. Currently, without an integrated chat system, OHC lacks the native omnichannel communication engine required to drive the "Work Triage" and "Customer Relationship" agent capabilities.

  From a non-technical owner's perspective: Maya shouldn't have to check Instagram, email, and WhatsApp separately. She needs one assistant-led inbox on her phone that organizes all conversations, drafts replies, and remembers customer preferences—without ever knowing what a "channel adapter" or "webhook" is.

  ## Research Report
  - **Chatwoot Source Code Audit**: A review of the `chatwoot/chatwoot` source repository reveals a mature Rails monolith. Key models include `Account` (Tenant), `Inbox`, `Channel::*`, `Conversation`, `Message`, and `Contact`. They rely heavily on ActionCable for WebSockets, background workers (Sidekiq) for webhooks and email parsing, and a PostgreSQL database.
  - **Competitor Analysis**: Shopify Inbox and WeCom unify messaging but often lack deep native AI orchestration out-of-the-box. Intercom and Zendesk are too complex for small operators.
  - **OHC Requirement**: OHC must implement this functionality natively in Rust inside our monorepo to ensure extremely low-latency WebSocket delivery, memory safety, and strict row-level multi-tenancy. This native chat engine will act as the foundational ingestion layer for our AI agents (Work Triage, Operations, and Sales).

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ ChannelAdapter : configures
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : initiates
      Conversation ||--o{ Message : contains
      Message ||--o| AI_Draft : "can have"

      %% Components
      Client -- WebSocket_Manager : "Subscribes (Real-time)"
      Webhook_Ingress -- ChannelAdapter : "Receives external events"
      ChannelAdapter -- Message_Processor : "Normalizes payload"
      Message_Processor -- Database : "Persists"
      Message_Processor -- Redis_PubSub : "Broadcasts"
      Redis_PubSub -- WebSocket_Manager : "Pushes to Client"
      Redis_PubSub -- AI_Job_Queue : "Triggers Triage/Draft Agents"
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View**: Unified feed of conversations. Each list item shows the contact name, channel icon (e.g., IG, WhatsApp), a snippet of the last message, and a subtle unread indicator.
  - **Conversation View**: Clean chat UI. Translucent glass sticky header with the contact's name and "Customer since [Date]". Below the message input, there's a distinct "AI Assistant" area.
  - **AI Drafting Flow**: When a new message arrives (e.g., "Do you do vegan cakes?"), the AI agent silently generates a draft. The owner sees a shimmering "AI Draft" bubble above the text input. Tapping it populates the input field where they can edit or hit send.
  - **No Technical Jargon**: Terms like "Inbox", "Channels", and "Webhooks" are hidden. The owner just sees "Connect Instagram" in the advanced settings.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Subscribes to the `ohc:chat:message_created` event. If a message is actionable (e.g., a booking request), it creates a pending Task in the owner's feed.
  - **Customer Relationship Agent**: Uses context from the `Contact` and past `Conversation` history to generate suggested replies (`AI_Draft`) as soon as an inbound message is persisted.

  ### Key Design Decisions
  - **Native Rust**: High-performance, secure, and memory-safe implementation to handle concurrent WebSocket connections and webhook ingress efficiently.
  - **Tenant Isolation**: Strict row-level security (RLS) implementation for all chat-related tables (`inboxes`, `conversations`, `messages`, `contacts`) based on `tenant_id`.
  - **Idempotent Ingestion**: Webhook ingress must use idempotent processing (caching external message IDs) to prevent duplicate messages from channel provider retries.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the core data model, Rust service layer, and foundational gRPC/REST APIs for the OHC Native Omnichannel Chat system.

  **User-Facing Outcome**: The business owner can see a unified list of conversations and send/receive messages in real-time on their mobile device (375px viewport) without needing third-party tools.

  **Acceptance Criteria**:
  1. Define and implement the database schema (PostgreSQL) for `inboxes`, `contacts`, `conversations`, and `messages` with strict `tenant_id` multi-tenancy.
  2. Implement the backend Rust service to handle creating conversations and adding messages.
  3. Implement a generic Channel Adapter trait/interface to allow future implementations of specific channels (e.g., Instagram, WhatsApp).
  4. Ensure 100% unit test coverage for the Rust service logic.
  5. Provide a Playwright E2E test that simulates an owner logging in, navigating to the "Messages" tab, and viewing a conversation.

  *(Note: Do not build specific third-party integrations like Meta/WhatsApp yet; focus on the core system, API, and the "Web Chat" channel adapter.)*

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
