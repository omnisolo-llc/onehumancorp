issue_title: "Native Rust Omnichannel Inbox to Replace Chatwoot"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement**
  OHC relies on external chat tools like Chatwoot or fragmented inboxes to handle customer communications. For our non-technical owners (like Maya the baker or Carlos the handyman), jumping between Instagram DMs, WhatsApp, Email, and SMS creates missed leads and lost context. They need a single, unified "Work Triage" feed natively built into OHC that unifies all conversations into one interface, allowing our AI agents to draft replies and manage context automatically. Chatwoot as an external dependency is being 100% retired in favor of a high-performance, native Rust omnichannel inbox tailored specifically for OHC's multi-tenant architecture and AI capabilities.

  **Research Report**

  *Chatwoot Source Code Audit:*
  Chatwoot's architecture relies heavily on Ruby on Rails with PostgreSQL and Redis. The core data models revolve around:
  - `Account` (Tenant isolation)
  - `Inbox` (A specific channel instance, e.g., Maya's Instagram)
  - `Channel` (The type of integration: WebWidget, API, Email, Facebook, Twitter, WhatsApp, SMS, Line, Telegram)
  - `Contact` (The customer/user interacting via the channel)
  - `ContactInbox` (Mapping between a Contact and a specific Inbox)
  - `Conversation` (A thread between an owner/agent and a Contact within an Inbox)
  - `Message` (Individual items within a Conversation)

  *Competitor Systems Audit:*
  Systems like Intercom, Shopify Inbox, and Zendesk use a similar hub-and-spoke model where various external API webhooks (Meta, Twilio) normalize into a standard `Message` schema. High-performance systems use WebSockets for real-time delivery and background job queues (like Rust's `faktory` or Postgres `SKIP LOCKED` patterns) to handle incoming webhook spikes.

  *Gap Identification:*
  OHC currently lacks a native Rust omnichannel data model and real-time messaging pipeline. We need a unified system where all incoming messages from connected integrations (Instagram, WhatsApp, SMS, Web Chat) are instantly normalized and stored in our database, with multi-tenant row-level security (RLS), and broadcasted to the owner's mobile app via WebSockets. Furthermore, our AI agents need to seamlessly hook into this pipeline to auto-draft replies.

  **Design Doc**

  *Architecture Diagram*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      TENANT ||--o{ CONTACT : has
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant MetaWebhook
      participant RustAPI (OHC)
      participant AI_Agent
      participant Owner_Mobile

      Customer->>MetaWebhook: Sends IG DM
      MetaWebhook->>RustAPI (OHC): POST /webhooks/meta
      RustAPI (OHC)->>RustAPI (OHC): Normalize to `Message`
      RustAPI (OHC)->>RustAPI (OHC): Save to DB (Postgres)
      RustAPI (OHC)->>AI_Agent: Trigger Auto-Draft Job
      AI_Agent->>RustAPI (OHC): Suggest Reply
      RustAPI (OHC)->>Owner_Mobile: WebSocket Event (New Message + Draft)
      Owner_Mobile->>RustAPI (OHC): Approve Draft
      RustAPI (OHC)->>MetaWebhook: Send IG DM
  ```

  *Mobile UX Flow (375px)*
  1. **Work Triage Feed:** A unified list of all active conversations. Unread messages are bold. A small icon indicates the source (IG, WA, Web).
  2. **Conversation View:** Standard chat bubble interface. A translucent floating action button shows the AI agent's suggested reply.
  3. **One-Tap Reply:** The owner taps the suggestion to send immediately or taps the text box to edit. The UI feels like iMessage or WhatsApp, using native keyboards.
  4. **Context Panel:** Swiping left reveals the customer's profile, past orders, and custom notes.

  *AI Agent Integration Points*
  - **Work Triage:** AI agent evaluates incoming messages and assigns priority/tags.
  - **Customer Assistant:** AI agent listens to the `message.created` event, reads the conversation history, and prepares a draft `Message` with `status: draft`.

  *Key Design Decisions*
  - Use PostgreSQL with Row-Level Security (RLS) bound to `tenant_id` for absolute data isolation.
  - Normalize all incoming provider payloads into a standard OHC `Message` entity.
  - WebSockets (via Axum) for real-time delivery to the Flutter frontend.
  - Background job queue (Postgres `SKIP LOCKED`) for processing webhooks and AI tasks to ensure high availability and responsiveness.

  **Implementation Prompt**
  Implement the native Rust omnichannel chat backend.
  - Define the core domain entities (`Inbox`, `ChannelAdapter`, `Contact`, `Conversation`, `Message`) ensuring strict multi-tenant (`tenant_id`) boundaries.
  - Implement the service layer to handle conversation creation, message threading, and state transitions (open, resolved, snoozed).
  - Provide the real-time event broadcasting mechanism so the Flutter frontend can subscribe to updates via WebSockets.
  - Ensure all database queries leverage multi-tenant scopes.
  - Write comprehensive unit tests for the domain logic and ensure `bazel test //...` passes completely.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
