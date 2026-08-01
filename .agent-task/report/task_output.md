issue_title: "[Native Rust Chatwoot Replacement] Core Data Model & Multitenant Channels"
issue_description: |
  **Mission Queue Protocol Brief**

  **Problem Statement**
  The OHC platform must provide a powerful, multi-channel inbox for small-business owners (like Maya the baker and Carlos the handyman) to consolidate WhatsApp, Instagram DMs, email, and web chat into a single feed. Currently, the architecture mandate dictates retiring external dependencies like Chatwoot and building a native Rust-based omnichannel chat engine that is fully multi-tenant, hyper-performant, and deeply integrated into OHC's edge and AI ecosystems. Small business owners cannot deal with disconnected silos; they need an "assistant-first" inbox where agents can draft responses, extract CRM data, and schedule bookings straight from the chat thread.

  **Research Report**
  I cloned the Chatwoot source code and analyzed their Ruby on Rails architecture. Chatwoot centers around a core `Account` (which translates to our `Tenant`), an `Inbox`, `Conversation`, `Message`, and `Contact`. They use polymorphic associations (`channelable`) to link an `Inbox` to specific channel implementations (e.g., `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Email`).
  Competitors like Shopify Inbox, GoDaddy Conversations, and Wix Inbox similarly aggregate channels but fail to deeply inject AI into the middle of the workflow. OHC will exceed them by treating "AI Agents" as first-class participants in the `Conversation` model, rather than external bots.

  **Design Doc**

  *Architecture Overview:*
  We will implement this native chat engine in Rust (within `src/server/ohc/chat` or similar), using PostgreSQL with row-level security (RLS) for tenant isolation.

  *Key Entities (Rust / Postgres):*
  - **Inbox**: Belongs to a `Tenant`. Represents a collection point (e.g., "Main Support", "Sales").
  - **ChannelAdapter**: Polymorphic link (e.g., `ChannelWebWidget`, `ChannelWhatsapp`). Contains config like tokens.
  - **Contact**: The external customer. Belongs to a `Tenant`.
  - **Conversation**: Links a `Contact` to an `Inbox`. Tracks status (open, resolved, snoozed, bot-handoff).
  - **Message**: The actual payload. Links to `Conversation`. Includes `sender_type` (Contact, User/Owner, Agent/AI) and `message_type` (incoming, outgoing, internal_note).

  *Mobile UX Flow (375px first):*
  1. The owner opens the OHC app and taps the "Inbox" tab in the bottom nav.
  2. The screen displays a unified list of active `Conversation`s, badged by channel (WhatsApp icon, Web icon).
  3. Tapping a conversation opens the thread. The AI assistant's drafted reply (if applicable) is pinned at the bottom, just above the native keyboard, waiting for a single tap to "Approve & Send".
  4. Swiping left on a conversation reveals quick actions: "Mark Resolved", "Assign to Agent".

  *AI Agent Integration Points:*
  - New incoming `Message`s trigger a PostgreSQL `SKIP LOCKED` job queue event.
  - The "Customer & Relationship Assistant" worker picks up the event, reads the `Conversation` history (fetching from the DB with RLS context), and inserts an `internal_note` or a drafted `outgoing` message.
  - Handoff mechanics: Conversations have a state `bot_active` vs `human_active`.

  **Implementation Prompt**
  Implement the core database schema (PostgreSQL migrations) and the foundational Rust data models (SeaORM or SQLx, depending on OHC's stack) for the native Omnichannel Chat system.
  - Create tables for `inboxes`, `channel_web_widgets`, `contacts`, `conversations`, and `messages`.
  - Ensure every table has a `tenant_id` column.
  - Implement Row-Level Security (RLS) policies on all new tables to enforce `tenant_id` isolation.
  - Create the Rust entity structs and the gRPC/REST API stubs for creating an inbox and sending a message.
  - Acceptance Criteria: A new tenant can be created, a Web Widget inbox provisioned, a contact created, and a conversation started with 2 messages via integration tests. The schema must cleanly support polymorphic channels.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
