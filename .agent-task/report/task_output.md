issue_title: "[Architectural Design] Native Rust Omnichannel Chat System (CW Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing CW with a high-performance, multi-tenant native Rust omnichannel chat system inside `onehumancorp/mono`. Small business owners like Carlos (handyman) or Maya (baker) receive messages across fragmented channels (Instagram DMs, WhatsApp, SMS, email, web chat). Without a unified inbox, they miss leads, drop context, and lose revenue. A native implementation allows OHC to deeply integrate its AI "Teammates" (e.g., The Ambassador Agent) to proactively draft and manage responses directly against the OHC backend, bypassing third-party platform limitations and latency.

  ## Research Report & Gap Analysis
  - **CW Source Code Audit:**
    - CW's core models (`Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*`) dictate how messages flow from disparate adapters into a unified view.
    - CW relies on Ruby on Rails, Sidekiq for background jobs, ActionCable for WebSockets, and Postgres for storage.
    - The `Inbox` model represents a channel-specific pipeline into a unified `Conversation`.
    - `Message` handles content, types (incoming, outgoing, template), and associations with `Sender` (User vs. Contact).
  - **OHC Implementation Gap:**
    - OHC needs to replicate this architecture in Rust using axum, tokio, tonic, and sqlx.
    - We must build the core entity schemas: `inboxes`, `conversations`, `messages`, `contacts`, and `channel_adapters`.
    - Real-time updates to the 375px mobile UI must use a native WebSocket or gRPC streaming layer instead of ActionCable.
    - The "Ambassador Agent" must act as a hook into the `messages` insertion pipeline to automatically draft suggested replies (status: `draft`) for the owner to review via the mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Webhook: IG/WhatsApp] -->|HTTP POST| B(Axum Channel Ingest API)
      B --> C{Rust Channel Adapter Layer}
      C -->|Normalize| D(Core Message Service)
      D --> E[(PostgreSQL - tenant isolated)]
      E -.->|Inboxes, Contacts, Convos, Messages| D
      D --> F(AI Ambassador Agent Hook)
      F -->|Drafts reply| D
      D --> G(WebSocket/gRPC Stream)
      G -->|Push| H[Mobile Flutter UI 375px]
      H -->|Approve Draft| D
      D --> C
      C -->|HTTP/API| I[External API: IG/WhatsApp]
  ```

  ### Data Model & Invariants
  1.  **`channels` / `inboxes`**: Represents a connection to an external provider (e.g., WhatsApp, Email, Web Widget). Must enforce `tenant_id` for multi-tenancy.
  2.  **`contacts`**: External customers communicating with the business. Unified by identity resolution.
  3.  **`conversations`**: Groups messages between a `contact` and an `inbox`. Tracks `status` (open, resolved, snoozed).
  4.  **`messages`**: Individual communication units. Has `message_type` (incoming, outgoing, draft) and `content`. AI-generated drafts are stored as `message_type=draft`.

  ### Mobile UX Flow (375px First)
  1.  **Unified Feed View:** A vertical list of `conversations`. Unread messages or pending AI drafts have high-contrast visual indicators.
  2.  **Conversation View:** A chat interface. If an AI draft exists, it is displayed prominently above the keyboard as an "Approval Card" with a "Send" button and an "Edit" button.
  3.  **Action:** Tapping "Send" updates the message state to `outgoing` and triggers the outbound channel adapter.

  ### AI Agent Integration Points
  -   The `MessageService` emits a tokio asynchronous event upon saving a new incoming `message`.
  -   The `Ambassador Agent` consumes this event, performs RAG over the tenant's data, and calls the LLM.
  -   The agent inserts a new `message` record with `status: draft` into the same `conversation`.
  -   The UI receives the new draft via WebSocket and updates the "Action Required" feed.

  ## Implementation Prompt
  **User-Facing Outcome**: As a small business owner, when a customer sends a message on Instagram, I receive a native push notification. Opening the app shows the conversation with a pre-written AI response tailored to their history. I can tap "Approve" to send it immediately.

  **CUJ & Acceptance Criteria**:
  1.  **Database Migration**: Create `inboxes`, `contacts`, `conversations`, and `messages` tables in Postgres with strict `tenant_id` row-level security.
  2.  **Rust Service Layer**: Implement `ConversationService` and `MessageService` in the `src/server/` crate with CRUD operations and tenant isolation.
  3.  **Mock Channel Adapter**: Implement a dummy webhook endpoint to simulate an incoming WhatsApp message.
  4.  **AI Hook**: Implement a basic event hook where a new incoming message triggers a dummy background task that inserts a `draft` response after 1 second.
  5.  **API Endpoints**: Expose REST/gRPC endpoints for the UI to list conversations and approve/send drafts.
  6.  **Tests**: Write 100% unit tests for the service layer and at least one E2E Playwright test that simulates receiving a webhook and approving a draft in the UI.

  **Note**: Do NOT prescribe specific sqlx macro usages or exact API routes—the implementer agent must design the lowest-level code details based on this structure.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
