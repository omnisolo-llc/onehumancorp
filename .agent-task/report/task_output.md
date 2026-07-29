issue_title: "Native Rust Omnichannel Chat Engine"
issue_description: |
  ## Problem Statement
  OneHumanCorp needs a fully native, multi-tenant omnichannel customer support and chat engine.
  Relying on an external service violates our self-contained, high-performance architectural vision and creates data silos.
  Non-technical owners need a unified inbox that aggregates Instagram DMs, SMS, WhatsApp, and Web Chat into a single, real-time feed on their 375px mobile screen.
  They need AI to draft replies, track contexts, and manage follow-ups without leaving the OHC platform.

  ## Research Report
  - **Source Audit:** Cloned and audited open-source repository.
  - **Key Learnings:** Multi-model structure built around `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, and multiple `Channel::*` models.
  - **Real-Time:** WebSockets and Redis for pub/sub.
  - **Normalization:** Webhook payloads from various platforms are parsed and normalized into a standard `Message` entity.
  - **Competitor Benchmarking:** Building it natively in Rust will give us superior performance, stronger type safety, and seamless integration with our AI Job Queue (PostgreSQL `SKIP LOCKED`).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    INBOX ||--o{ CHANNEL_ADAPTER : configures
    INBOX ||--o{ CONVERSATION : contains
    CONVERSATION ||--o{ MESSAGE : contains
    CONVERSATION }o--|| CONTACT : belongs_to
    TENANT ||--o{ CONTACT : manages
  ```

  ### Key Architectural Decisions
  1. **Multi-Tenancy & Zero Trust:** Every entity (`Inbox`, `Conversation`, `Message`, `Contact`) MUST include `tenant_id`. PostgreSQL Row-Level Security (RLS) must enforce strict tenant isolation.
  2. **Rust Channel Adapters:** Define a Rust trait `ChannelProvider` that standardizes `send_message` and `parse_webhook`. Implement an initial native `WebWidget` provider.
  3. **Real-Time WebSockets:** Use `axum::extract::ws` backed by Redis Pub/Sub (`ohc:chat:pubsub:{tenant_id}`) to push real-time message updates to the Flutter frontend.
  4. **AI Agent Integration:** When a new `Message` is inserted, emit a job to the AI Job Queue. The "Customer & Relationship Assistant" will pick up the job, analyze the conversation context, and generate a draft reply in the background, updating the UI via WebSocket when the draft is ready.

  ### Mobile UX Flow (375px Viewport)
  - **Unified Inbox Screen:** Clean, Ubiquiti-style list view. Each row displays the Contact name, a channel badge, preview text, and an unread indicator.
  - **Conversation Screen:** Translucent glass App Bar with the Contact details. Scrollable message history with clear visual distinction between customer messages (left) and owner/agent messages (right).
  - **Interaction:** A sticky bottom text input with a prominent "AI Draft" magic wand button. Employs native mobile keyboards seamlessly without layout shifting. Touch targets are rigorously >= 44x44px.

  ## Implementation Prompt
  "Implement the core multi-tenant Rust Chat Engine based on the native omnichannel design. Create the database schemas for `Inbox`, `Conversation`, `Message`, and `Contact` in PostgreSQL, ensuring `tenant_id` and RLS are strictly applied. Build the REST and `axum` WebSocket endpoints required for reading and sending messages in real-time. Implement the base `ChannelProvider` trait and a `WebWidget` adapter.
  On the frontend (Flutter PWA), build the 375px-optimized Unified Inbox and Conversation screens with macOS-style Translucent Glass aesthetics. Connect the UI to the WebSocket for real-time updates.
  Integrate the Customer Assistant agent so that incoming messages trigger a background job to prepare draft replies. Ensure 100% unit test coverage for the Rust backend and comprehensive Playwright E2E tests verifying the real-time chat flow."
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
