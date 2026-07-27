issue_title: "Architecture Design: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as an external dependency to build a unified, high-performance, multi-tenant work assistant. Relying on an external third-party chat service breaks our Zero-Trust architecture, limits multi-tenant data isolation enforcement within our primary PostgreSQL database, and restricts our AI agents' ability to intercept, modify, and draft responses synchronously within the core event loop. OHC owners need an embedded, instant, and unified inbox experience across DMs, Web Chat, and Email that is deeply integrated with the AI operations layer, without managing third-party accounts or paying external SLA fees.

  ## Research Report
  Chatwoot's architecture relies on Ruby on Rails, Sidekiq, Redis, and PostgreSQL. It uses `Conversations`, `Messages`, `Contacts`, `Inboxes`, and `Channel` adapters (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Api`). Real-time communication is handled via ActionCable (WebSockets).
  To achieve parity and superior performance in OHC:
  - We can replace Rails with our Rust (Axum) API server.
  - ActionCable WebSockets can be replaced with Axum WebSockets and Redis Pub/Sub for cross-node event propagation.
  - Sidekiq workers can be replaced with our PostgreSQL `SKIP LOCKED` job queue and Rust background workers.
  - The domain model must be ported to strict multi-tenant schema invariants: `tenant_id` on every table (Conversations, Messages, Inboxes, Contacts).

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CONVERSATION : receives
    CONTACT ||--o{ CONVERSATION : initiates
    CONVERSATION ||--o{ MESSAGE : contains
    INBOX ||--|{ CHANNEL_ADAPTER : configured_via

    TENANT {
      uuid id PK
    }
    INBOX {
      uuid id PK
      uuid tenant_id FK
      string name
    }
    CONTACT {
      uuid id PK
      uuid tenant_id FK
      string identifier
      string name
    }
    CONVERSATION {
      uuid id PK
      uuid tenant_id FK
      uuid inbox_id FK
      uuid contact_id FK
      string status
    }
    MESSAGE {
      uuid id PK
      uuid tenant_id FK
      uuid conversation_id FK
      string content
      boolean is_agent
    }
  ```

  ### AI Agent Integration
  - **Operations Assistant** subscribes to `conversation.created` and `message.created` events.
  - When a message arrives, the Rust API pushes a job to the PostgreSQL queue.
  - The AI worker dequeues the job, retrieves tenant memory and contact history, and drafts a reply.
  - It inserts a `MESSAGE` with `status: draft`, which streams via WebSocket to the owner's UI for approval.

  ### Mobile UX Flow (375px First)
  1. **Inbox View**: Unified list of active conversations. Translucent glass app bar. Unread indicators.
  2. **Thread View**: Native-feeling chat bubbles. Bottom input bar with an "AI Draft" shimmering button.
  3. **Agent Handoff**: A translucent overlay when the AI is drafting a response. One-tap "Send" or "Edit" for the AI draft. 44x44px touch targets for all actions.

  ## Implementation Prompt
  **Goal:** Implement the core domain logic, gRPC/REST APIs, and Axum WebSocket endpoints for the native Rust omnichannel chat engine.
  **CUJ:** As an OHC Owner (Maya), I receive an Instagram DM inquiry. The system routes it to my unified inbox, creates a Contact and Conversation, and the AI drafts a reply. I see this instantly in my 375px mobile view, review the draft, and tap "Approve" to send the reply back.
  **Acceptance Criteria:**
  - Create database migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages` (enforcing `tenant_id`).
  - Implement Rust Axum WebSocket handler for real-time `message.created` streaming.
  - Build UI components in Tauri/Flutter for the unified inbox with macOS translucent glass styling and UniFi modular layouts.
  - 100% Unit and Playwright E2E test coverage for the full chat lifecycle.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
