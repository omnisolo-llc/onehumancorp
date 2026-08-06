issue_title: "Implement Native Rust Multi-Tenant Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Title: Implement Native Rust Multi-Tenant Omnichannel Chat System

  ## Problem Statement
  Currently, OHC relies on external or disconnected systems to handle customer messaging, or lacks a unified, multi-channel inbox. For a business owner (like Maya the baker or Carlos the handyman), dealing with customer messages across Instagram DMs, WhatsApp, Website Chat, and Email is chaotic. They miss inquiries, lose context, and drop leads. OHC needs a centralized, highly-reliable omnichannel inbox that unifies all customer communications into one feed, tightly integrated with our AI agents to draft replies, track orders, and propose actions natively within the OHC platform, without relying on third-party SaaS like Chatwoot.

  ## Research Report
  - **Chatwoot Audit**: A deep dive into the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals a robust structure based on core models: `Account` (Tenant), `Inbox`, `Channel` (adapters for web widget, FB, Twitter, API, etc.), `Conversation`, `Message`, and `Contact`. They use WebSockets for real-time delivery and background jobs for webhooks/integrations.
  - **Competitive Analysis**: Shopify Inbox, Wix Inbox, and GoDaddy Conversations all provide a unified inbox for merchants. They abstract away the complexity of channel APIs (like Meta Graph API) and give the merchant one continuous thread per customer. Shopify Inbox deeply integrates product recommendations and order context into the chat.
  - **OHC Specific Needs**: Unlike Chatwoot which is a standalone support tool, OHC's inbox must be native. It needs to feel like an assistant (Work Triage) grouping tasks with messages, running AI drafts (Customer & Relationship Assistant), and maintaining multi-tenant strict isolation via PostgreSQL RLS. Since Chatwoot is being 100% RETIRED, we must build a native Rust multi-tenant chat engine that achieves feature parity with Chatwoot's omnichannel capabilities.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ PARTICIPANT : includes
      MESSAGE ||--o{ ATTACHMENT : has
  ```

  ### Core Entities (Native Rust)
  - `Tenant`: The business workspace.
  - `Inbox`: A routing bucket for conversations (e.g., "Support", "Sales", or channel-specific).
  - `ChannelAdapter`: Stores credentials and webhooks for specific platforms (WhatsApp, IG, Web Widget).
  - `Contact`: The external user/customer.
  - `Conversation`: A thread of messages between a Contact and the Tenant (via an Inbox).
  - `Message`: Individual chat bubble (text, image, agent-drafted).

  ### System Flow
  - **Ingestion**: Webhooks from external providers (e.g., Meta) hit our Rust API, which maps them to an Inbox and Contact, creating a Conversation/Message.
  - **Real-Time**: WebSockets notify the OHC Frontend of new messages instantly.
  - **AI Integration**: On new message creation, a background job is enqueued (via Postgres `SKIP LOCKED`) for the `Customer & Relationship Assistant` to generate a draft reply and suggest next actions (e.g., creating a quote).

  ### Mobile UX Flow (375px first)
  1. **Work Triage Tab**: Owner sees an unread conversation badge.
  2. **Unified Inbox List**: A clean, touch-friendly list (44x44px targets) showing recent conversations, with channel icons (e.g., IG, WhatsApp) and unread dots.
  3. **Conversation Thread**: Native-feeling chat screen. Customer messages on the left, owner messages on the right. AI drafts appear inline with a distinct visual state (e.g., translucent glass tint) and a single "Approve & Send" button.
  4. **Context Drawer**: Swiping left or tapping a header info icon reveals the customer's lifetime value, past orders, and notes.

  ### Key Design Decisions
  - **Native Rust**: High performance, type-safe API, and easy integration with OHC's existing Rust ecosystem and multi-tenant database constraints.
  - **Polymorphic Channels**: Abstracting the channel logic ensures the core Conversation/Message engine doesn't care if a message is an email or an IG DM.
  - **Strict Multi-Tenancy**: All queries MUST scope by `tenant_id` and rely on PostgreSQL RLS.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational native Rust omnichannel chat system for OHC.
  **CUJ (Critical User Journey)**:
  1. As an owner (Maya), I open my Work Triage feed on my phone and see a new Instagram DM from a customer asking about vegan cakes.
  2. I open the conversation. The UI shows the DM alongside an AI-drafted reply ("Yes, we do! Here is our vegan menu...").
  3. I tap "Approve & Send", and the message is dispatched to the customer.

  **Requirements**:
  - Implement the core database schema for `inboxes`, `contacts`, `conversations`, and `messages` with multi-tenant row-level security.
  - Build the Rust API endpoints to list conversations, fetch messages, and send messages.
  - Create a placeholder `ChannelAdapter` trait/interface to allow future implementations of IG/WhatsApp.
  - Implement a basic WebSocket broadcasting mechanism for real-time UI updates.
  - Add a Flutter frontend view (Mobile-first, 375px) matching the OHC Premium Token design system.
  - Add Playwright E2E tests verifying an owner can see and reply to a conversation.
  - Do NOT use any third-party Chatwoot services.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
