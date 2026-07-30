issue_title: "Implement Native Rust Multi-Tenant Omnichannel Chat Engine"
issue_description: |
  # Implement Native Rust Multi-Tenant Omnichannel Chat Engine

  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native, high-performance, multi-tenant omnichannel chat engine to centralize communications (Instagram, WhatsApp, Email, Web Widget) for owners. Relying on an external third-party service like Chatwoot is completely retired. The absence of a unified, native system forces owners to context switch between apps, losing valuable customer context and preventing OHC's AI agents from seamlessly assisting with work triage and customer relationships. Maya the baker and Carlos the handyman need all their messages routed into one single OHC inbox that works perfectly on their 375px mobile screens without technical setup.

  ## Research Report
  - **Chatwoot Source Audit**: Evaluated Chatwoot's architecture (`app/models/inbox.rb`, `app/models/channel/`). It heavily relies on polymorphic associations (Inboxes linked to specific Channel models like `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Instagram`). It uses WebSockets for real-time delivery and Sidekiq for async processing.
  - **Competitor Analysis**: Tools like Shopify Inbox and Stripe also unify communications but are tightly coupled to their commerce engines. OHC needs a similar tightly integrated approach, but built natively in Rust.
  - **Data Model Translation**: The Ruby on Rails models need to be translated into strict, tenant-isolated Rust models with PostgreSQL row-level security.
  - **Gaps to Fill**: Real-time event broadcasting (Rust WebSockets/gRPC streams), webhook ingest for external channels (WhatsApp/IG), and AI agent hooks (Customer & Relationship Assistant drafting replies).

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configures
      CHANNEL_ADAPTER {
          uuid id
          string channel_type
          json credentials
      }
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          string content
          boolean is_ai_draft
          datetime created_at
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Work Triage Dashboard**: Owner opens app, sees unified "Inbox" card with a notification badge indicating new messages.
  2. **Conversation List**: Tapping Inbox opens a vertically scrolling list of active conversations. Each row shows the customer name, channel icon (IG, Web, WA), and the latest message snippet. No horizontal scrolling.
  3. **Chat Thread**: Tapping a conversation opens the chat view.
     - **Top bar**: Customer name & contact details toggle.
     - **Body**: Message bubbles. AI drafted replies appear in a distinct translucent "Premium OHC Token" glass container with an "Approve & Send" button.
     - **Bottom**: Native mobile keyboard input, attachment button (camera/library).
  4. All touch targets strictly >= 44x44px.

  ### AI Agent Integration Points
  - **Event Trigger**: When a new message arrives via Webhook, the ingestion pipeline queues an AI job via the `SKIP LOCKED` Postgres queue.
  - **Work Triage AI**: Categorizes the message (e.g., inquiry, complaint, spam).
  - **Customer Assistant AI**: Analyzes conversation history (Tenant-scoped memory) and drafts a reply.
  - **Storage**: Draft is saved as a `MESSAGE` with `is_ai_draft = true` and broadcast to the frontend via WebSocket. The owner sees it instantly and can approve/edit.

  ## Implementation Prompt
  "Implement the backend Rust core for the Native Omnichannel Chat Engine in `src/server/ohc/chat/`. You must define the PostgreSQL schemas for `inboxes`, `channels`, `conversations`, `contacts`, and `messages`, ensuring rigorous row-level security by `tenant_id`. Implement a robust webhook ingestion endpoint to receive incoming messages and a WebSocket broadcasting service to push updates to the Flutter frontend. Add integration points for the AI Customer Assistant to auto-draft replies. The UI must follow the macOS Translucent Glass aesthetic and fit perfectly on a 375px mobile screen. All code must achieve 100% test coverage and include full Playwright E2E tests for the new Chat CUJ."

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, core-architecture]
assignees: []
