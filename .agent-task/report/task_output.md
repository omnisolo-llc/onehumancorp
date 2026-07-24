issue_title: "[Architecture] Native Rust Omnichannel Inbox & Chatwoot Replacement"
issue_description: |
  ## Problem Statement
  Currently, OmniSolo (OHC) integrates with or previously considered external customer support tools like Chatwoot. However, relying on third-party services creates friction for small-business owners (like Maya the Baker or Carlos the Handyman), who need an integrated, zero-configuration assistant that operates across all customer channels (Instagram, WhatsApp, SMS, Web Chat) seamlessly out of the box. An external tool also breaks strict multi-tenancy, introduces latency, and fractures the "One Assistant" vision. We need a native, multi-tenant, high-performance omnichannel inbox that handles all customer communications directly within OHC.

  ## Research Report
  Based on an audit of Chatwoot's source code and industry standards (Shopify Inbox, Meta Business Suite):
  - **Data Model**: Chatwoot relies on accounts, inboxes, channels, conversations, and messages. This maps perfectly to OHC's multi-tenant (`tenant_id`) approach.
  - **Channels**: Chatwoot abstracts channels (API, Email, Facebook, Instagram, Line, SMS, Telegram, Web Widget, WhatsApp) behind channel adapters. We need equivalent Rust traits and services to adapt incoming webhooks into standard `Message` entities.
  - **Real-time Engine**: Chatwoot uses ActionCable for WebSockets. OHC will leverage native Rust (e.g., Tokio, Axum/Tonic WebSockets) for a significantly more efficient and concurrent real-time messaging pipeline, connected to our Redis Pub/Sub backplane.
  - **Agent Automation**: Chatwoot uses "automation rules" and "macros." In OHC, this will be handled directly by our AI Assistant Capabilities (Operations, Customer Assistant), observing the message stream and acting natively.

  ## Design Doc
  ### High-Level Architecture (Rust)
  ```mermaid
  graph TD
    Client[Mobile/Web Client - 375px PWA] -->|WebSocket/REST| RustGateway[Rust API Gateway]
    Webhooks[External Webhooks: IG, WA, SMS] -->|HTTP| ChannelAdapters[Rust Channel Adapters]

    RustGateway --> CoreInbox[Omnichannel Inbox Service]
    ChannelAdapters --> CoreInbox

    CoreInbox --> Postgres[(PostgreSQL Multi-Tenant DB)]
    CoreInbox --> Redis[Redis Pub/Sub & Caching]

    Redis --> AIAgents[AI Assistant Job Queue]
    AIAgents -->|Drafts Replies| CoreInbox
  ```

  ### Core Entities & Multi-Tenancy Invariants
  1. `Tenant` (Owner Workspace)
  2. `Inbox` (Container for grouped channels)
  3. `ChannelAdapter` (Configuration for IG, WA, Email, etc.)
  4. `Contact` (Customer profile)
  5. `Conversation` (Thread of messages)
  6. `Message` (The actual payload, sender type: Customer vs. AI/Agent)
  *All database tables must include `tenant_id` and enforce PostgreSQL RLS (`ENABLE ROW LEVEL SECURITY`).*

  ### Mobile UX Flow (375px First)
  - **Triage View**: A unified feed where the owner sees all incoming customer messages (marked with channel icons like IG, WA).
  - **Conversation View**: Tap a conversation to view the thread. AI-generated draft replies appear as translucent "Glass" suggestion chips.
  - **Action Layer**: Under the chat, simple action buttons: "Approve AI Reply", "Create Booking", "Send Payment Link". No complex "ticket status" dropdowns unless the user expands an "Advanced" sheet.

  ### AI Agent Integration
  - **Trigger**: New incoming `Message` triggers a Redis event.
  - **Action**: The `Customer Assistant` reads the `Conversation` history and `Contact` context, then inserts a draft `Message` (status: `pending_approval`).
  - **UI Reflection**: The drafted message surfaces to the owner in the Mobile UI. The owner taps "Approve" (or edits it), which dispatches the message via the specific `ChannelAdapter`.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the core database schema and the foundational Rust services for the Native Omnichannel Inbox.
  1. Define the SQL migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring all tables have a `tenant_id` and RLS enabled.
  2. Implement the Rust data models and a basic `CoreInbox` service that can receive a standard message payload and save it to the database.
  3. Create a unified mobile-first UI component (React/Flutter) for the "Triage View" that renders a list of active conversations with mock-free, real database data. Use macOS-style Translucent Glass materials.
  4. Add a Playwright E2E test verifying an owner can view an incoming message in their unified triage view.

  Do not worry about specific external channel API integrations (WhatsApp, IG) yet; focus on the internal pipeline and the generic Channel Adapter interface.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
