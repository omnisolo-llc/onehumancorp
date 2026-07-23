issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp previously relied on Chatwoot as a third-party dependency for handling omnichannel customer support. Chatwoot has been 100% retired to consolidate operations, reduce external dependencies, improve multi-tenant isolation, and tightly integrate AI orchestration. We need a native Rust omnichannel chat system within `onehumancorp/mono` that achieves feature parity with Chatwoot (Inboxes, Conversations, Messages, Channels) but operates with Zero-Trust multi-tenant data boundaries and seamless AI agent assistance. Non-technical owners (like Maya the baker and Carlos the handyman) need a unified mobile-first inbox where all customer DMs, texts, and emails are triaged and drafted automatically by the AI assistant without any complex configuration.

  ## Research Report
  - **Codebase & Docs Audit**: OHC currently lacks native models for `Inbox`, `Conversation`, `Message`, and `ChannelAdapter`. Chatwoot relies heavily on Ruby on Rails ActiveRecord models.
  - **Chatwoot Source Code Benchmarking**:
    - The repository (`https://github.com/chatwoot/chatwoot`) uses a normalized structure:
      - `Inbox`: Configuration for a communication channel.
      - `Conversation`: A threaded discussion within an Inbox, linking an `Account` (Tenant) and a `Contact`.
      - `Message`: Immutable log of communication, threaded via `Conversation`.
      - `Channel`: Specific models for Facebook, Twitter, WhatsApp, Email, Web Widget, etc.
    - Chatwoot uses WebSockets (ActionCable) for real-time messaging.
  - **Competitor Insights**: Shopify Ping and Apple Business Chat utilize edge-cached, extremely low-latency messaging. We will build a high-performance gRPC + Rust microservice with a Flutter Web UI utilizing WebSockets and Server-Sent Events (SSE).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : holds
      MESSAGE }o--|| AI_DRAFT : optionally_has

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string provider_type
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: A unified, translucent-glass list view containing grouped messages. Each row shows the customer avatar, channel icon (e.g., IG, SMS), last message snippet, and an AI indicator if an auto-reply is drafted.
  2. **Conversation Thread**: Tapping a row opens a full-screen thread. The header contains sticky customer context.
  3. **AI Drafting Action Bar**: At the bottom of the screen, instead of just a raw text input, a prominent "Review AI Draft" button appears for new inquiries, alongside quick-action tokens.
  4. **Swiping/Gestures**: Swipe left to resolve/archive, swipe right to assign to AI for follow-up.

  ### AI Agent Integration Points
  - **Work Triage**: Whenever a new `Message` is created via a webhook channel adapter, the system triggers the `Work Triage Agent` to categorize the intent and link existing customer history.
  - **Customer & Relationship Assistant**: Drafts context-aware replies stored as `AI_DRAFT` entities associated with the `Message` thread. The owner taps one button to approve or edit the draft.
  - **Operations Assistant**: Extracts dates and booking requests from messages to create calendar events automatically.

  ### Key Design Decisions
  1. **Zero-Trust Multi-Tenancy**: Every database table will require `tenant_id` with Row-Level Security (RLS) enabled in PostgreSQL. Application-level gRPC interceptors will enforce tenant validation.
  2. **Rust-Native & gRPC First**: Implemented in Rust for memory safety and concurrency, exposing gRPC for internal AI jobs and a REST/WebSocket translation layer for the Flutter UI.
  3. **No External Chatwoot Hooks**: All channel adapters (Instagram, WhatsApp, Web Widget) will terminate directly in the new OHC Rust infrastructure, ensuring complete control over the payload and AI pipeline.

  ## Implementation Prompt
  **Goal:** Implement the backend Rust primitives for the OHC Omnichannel Chat Engine.
  **CUJ:** A small business owner opens the OHC mobile app (375px width) and sees a unified inbox of messages from Instagram, SMS, and Web Widget. They tap a conversation and read an AI-drafted reply.
  **Instructions:**
  1. Define the Protocol Buffer definitions (`chat.proto`) for `Inbox`, `Conversation`, `Message`, and `Contact`. Ensure `tenant_id` is included for RLS.
  2. Implement the gRPC server in Rust (`src/server/chat`) handling CRUD for Inboxes and Conversations.
  3. Create a Postgres database schema (DDL) with strict RLS policies bound to the tenant ID.
  4. Implement a mock `ChannelAdapter` for receiving dummy webhooks and converting them into `Message` records.
  5. **Acceptance Criteria**: `bazel test //...` passes 100%. The Rust service must compile and run. Unit tests must cover the chat creation flow and enforce tenant isolation. Do not implement complex UI in this task; focus on the Rust backend and proto definitions.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
