issue_title: "Architecture & Implementation: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing legacy external chat system as an external dependency. We need a native, high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust. This system must handle real-time messaging, multi-channel integrations (Web Widget, Email, Instagram, WhatsApp, etc.), and multi-tenant data isolation. For our target personas (like Maya who handles Instagram DMs, or Carlos who uses a web widget for service requests), having a unified inbox that is fast and reliable is crucial.

  ## Research Report
  Based on an audit of the legacy external chat system source code, their architecture heavily relies on polymorphic associations (e.g., Channels), multi-tenant isolation (Account ID on every model), and real-time pub/sub via WebSockets.

  To build this in Rust, we need to replicate these core models:
  - **Account/Tenant**: The root of multi-tenancy.
  - **Inbox**: A unified view for messages, associated with a specific Channel.
  - **Channel**: Polymorphic-like trait or enum representing different message sources (WebWidget, API, Email, SMS, WhatsApp).
  - **Conversation**: A thread of messages between a Contact and an Assignee/Bot.
  - **Message**: The individual message unit.
  - **Contact**: The customer.

  ## Design Doc

  ### Architecture Diagram (Mental Model)
  - **Frontend (Flutter PWA/Mobile)**: Connects via WebSocket for real-time updates and REST for historical data.
  - **API Gateway (Rust/Axum or Tonic)**: Routes requests and verifies SPIFFE/OIDC tokens.
  - **WebSocket Server (Rust/Tokio-Tungstenite or Axum WS)**: Manages active connections, subscribes to Redis channels.
  - **Chat Service (Rust)**: Handles business logic, creating conversations, processing incoming webhooks from channels.
  - **Database (PostgreSQL)**: Stores all entities. Uses Row-Level Security (RLS) based on `tenant_id`.
  - **Pub/Sub (Redis)**: Broadcasts message events across multiple server instances.

  ### Mobile UX Flow (375px first)
  - **Unified Inbox Screen**: A list of active conversations. Each row shows the contact avatar, last message snippet, time, and a channel icon (e.g., Instagram, Web).
  - **Conversation Screen**:
    - Header: Contact name and status.
    - Body: Scrollable message bubbles. Contact messages on the left, Owner/Agent messages on the right.
    - Footer: Native keyboard-friendly input area with attachment button and send button.

  ### AI Agent Integration Points
  - **Agent_Bot**: A special assignee type. When a new conversation starts, if an Agent_Bot is assigned to the Inbox, it can auto-reply.
  - **Drafting**: AI can suggest replies in the Conversation screen based on conversation history and tenant knowledge base.

  ## Implementation Prompt
  **Goal:** Implement the core database schema and Rust backend structures for the Omnichannel Chat System.

  **Tasks:**
  1. Define the PostgreSQL schema (using SQL migrations) for `inboxes`, `channels` (can be separate tables per type or a single table with a type column and JSONB config), `conversations`, `messages`, and `contacts`. Ensure `tenant_id` is present and RLS is configured.
  2. Create Rust structs representing these entities.
  3. Implement a basic Rust service layer (e.g., `ChatService`) with methods to:
     - Create an inbox and associated channel.
     - Create a contact.
     - Start a conversation.
     - Send a message in a conversation.
  4. Ensure unit tests cover the service logic and multi-tenant isolation.

  **Acceptance Criteria:**
  - Database migrations are present and run successfully.
  - Rust models accurately reflect the schema and include necessary relations.
  - The `ChatService` can create and retrieve chat entities, strictly scoped by `tenant_id`.
  - Unit tests achieve 100% coverage for the new code.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
