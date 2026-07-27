issue_title: "Architectural Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OHC currently relies on external Chat system for omnichannel customer support. We need to implement our own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. This new system must handle multi-tenant isolation, real-time WebSockets, agent routing, and channel adapters (Instagram, WhatsApp, Email, Web Widget) to serve our primary non-technical owner personas like Maya and Carlos directly from their mobile devices.

  ## Research Report
  Based on an audit of the source code and competitor tools:
  - **Data Models**: Relies on `Accounts` (Tenants), `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `Channel` models.
  - **Controllers & Channels**: Use ActionCable for real-time WebSocket messaging. Events like `message.created` and `conversation.updated` are pushed down to clients.
  - **Agent Routing & SLA**: Round-robin assignment, SLA policies, and macros are used to automate replies.
  - **AI Integration**: AI agents must be able to hook into `message.created` events to auto-draft replies based on context.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ Conversation : contains
      Conversation ||--o{ Message : has
      Conversation ||--|| Contact : belongs_to
      Inbox ||--o{ ChannelAdapter : uses
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View**: A clean, unified inbox showing conversations from all channels. Read/unread states indicated clearly with OHC Premium Tokens.
  - **Chat View**: Chat bubbles with a native-feeling composer. AI-drafted responses appear above the composer with an "Approve & Send" or "Edit" button.
  - **Touch Targets**: All message action buttons and channel toggles are strictly >= 44x44px.
  - **Offline/Flaky Network**: Optimistic UI updates. Messages are marked as "Sending..." and saved locally until confirmed by the Rust backend.

  ### AI Agent Integration Points
  - Agents subscribe to the Redis pub/sub topic for new messages.
  - **Customer Assistant Agent** reads conversation history, fetches customer context (deposits, past orders), and writes an `ai_draft` to the conversation, which is synced to the frontend via WebSockets.

  ### Key Design Decisions
  - **Rust + WebSockets**: The new backend will use `axum` + `tokio-tungstenite` for high-performance WebSockets.
  - **Multi-Tenant Isolation**: Row-Level Security (RLS) in PostgreSQL is enforced by injecting `tenant_id` at the database session level for every request.
  - **Redis Pub/Sub**: Real-time events will be broadcast across instances using Redis to support horizontal scaling.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the foundational backend for the native Rust omnichannel chat system.
  1. Setup the basic Rust project structure for the chat microservice if it doesn't exist, or integrate into the existing Rust backend.
  2. Implement the `Conversation`, `Message`, and `Inbox` entities with strict multi-tenant isolation.
  3. Create the WebSocket server using `axum` that accepts connections and can broadcast `message.created` events.
  4. Ensure all database writes are protected by `tenant_id` and test for cross-tenant data leakage.
  5. Provide a basic Flutter frontend component (375px width optimized) that connects to this WebSocket and displays a placeholder inbox.
  6. **Acceptance Criteria**: A user (like Maya) can open the mobile view, see a hardcoded or newly created conversation, send a message, and see it echoed back via WebSocket without any external dependencies.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust]
assignees: []
