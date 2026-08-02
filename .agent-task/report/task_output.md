issue_title: "Design and Implement Native Rust Omnichannel Chat Engine"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) relies on external tools or lacks a fully integrated, high-performance omnichannel inbox native to its ecosystem. As mandated, the legacy third-party chat service is 100% RETIRED. OHC owners like Maya (baker managing IG DMs) and Carlos (handyman handling SMS and website leads) need a unified, zero-latency inbox to coordinate with customers seamlessly. The lack of a native system forces context switching and creates delays in AI agent triage.

  ## Research Report
  - **Competitor Benchmarking**: We audited the architecture of leading platforms like Shopify Inbox, and Wix Inbox.
  - **Native Omnichannel Source Code Audit**: We analyzed external systems omnichannel data models (Conversations, Messages, Contacts, Inboxes, Channel Adapters), WebSocket real-time messaging, and webhook processing for platforms like WhatsApp, Instagram, and SMS.
  - **Finding**: OHC must improve upon native data models and WebSocket event patterns using Rust and gRPC to guarantee high-performance, row-level tenant isolation, and strict Zero-Trust security.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : engages_in
      Conversation ||--o{ Message : contains
      ChannelAdapter ||--|{ Inbox : routes_to
  ```
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_WebHook
      participant Rust_Chat_Engine
      participant AI_Triage_Agent
      participant Owner_Mobile
      Customer->>OHC_WebHook: IG DM / SMS
      OHC_WebHook->>Rust_Chat_Engine: Parse & normalize payload
      Rust_Chat_Engine->>Rust_Chat_Engine: Upsert Contact & Conversation
      Rust_Chat_Engine->>AI_Triage_Agent: Trigger async job for auto-reply / triage
      Rust_Chat_Engine->>Owner_Mobile: WebSocket push (New Message)
      AI_Triage_Agent->>Rust_Chat_Engine: Insert draft reply
      Rust_Chat_Engine->>Owner_Mobile: WebSocket push (Draft Ready)
  ```

  ### Mobile UX Flow (375px First)
  - **Screen 1: Unified Inbox (Home)**: A clean, UniFi-style list of active conversations. Each card shows the customer name, last message preview, channel icon (IG, SMS, Web), and an AI triage badge (e.g., "Needs Deposit", "Urgent").
  - **Screen 2: Conversation View**: WhatsApp-style chat interface. Translucent glass sticky header with customer details. AI-suggested draft replies appear just above the input field. The input field uses the native mobile keyboard.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Listens to `message.created` events on the event bus to categorize urgency and suggest the next best action.
  - **Customer & Relationship Agent**: Automatically generates context-aware draft replies based on the tenant's memory and past interactions, inserting them directly into the conversation state for owner approval.

  ### Key Design Decisions
  - **Language & Stack**: Built in Rust inside `onehumancorp/mono` for maximum performance and memory safety.
  - **Real-time Protocol**: WebSockets handled by Rust (e.g., `tokio` + `axum` or `tungstenite`) with Redis Pub/Sub for horizontal scaling across nodes.
  - **Multi-tenancy**: Strict PostgreSQL row-level security (`tenant_id`) enforced on all tables (Inbox, Conversation, Message, Contact).

  ## Implementation Prompt
  **Goal**: Implement the core data model, gRPC service, and WebSocket infrastructure for the Native Rust Omnichannel Chat Engine.
  **Persona Context**: Maya needs to see her Instagram DMs and website inquiries in a single list on her iPhone, with zero lag, so she can quickly approve AI-drafted replies while baking.
  **Acceptance Criteria**:
  1. Define the PostgreSQL schema for Inboxes, Contacts, Conversations, and Messages with strict `tenant_id` isolation.
  2. Implement the Rust backend services to handle CRUD operations for these entities.
  3. Implement a WebSocket endpoint that successfully broadcasts `message.created` and `conversation.updated` events to connected clients.
  4. Build a basic Mobile-First (375px) Flutter view demonstrating the real-time Unified Inbox with mock-free, real database-backed data.
  5. 100% Unit and E2E Test Coverage. No mocked database calls.

  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
