issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  Our core personas (Maya, Carlos, Priya, Leo, Fatima) operate across multiple communication channels (Instagram DMs, WhatsApp, SMS, web chat) to manage their business operations. Currently, OneHumanCorp (OHC) lacks a unified, high-performance omnichannel communication architecture. This gap results in fragmented customer context, delayed responses, and limits the ability of the AI assistants (e.g., Customer & Relationship Assistant) to seamlessly coordinate work triage and draft replies across all channels in real-time.

  ## Research Report
  - **Shopify Inbox & Ping**: Uses a unified inbox for web chat, email, and social, but relies heavily on third-party apps for advanced routing.
  - **Zendesk / Intercom**: Powerful but far too complex and technical for a non-technical owner like Maya or Carlos, requiring significant configuration and admin overhead.
  - **Tencent Workbuddy / WeCom**: Integrates deeply with WeChat but lacks multi-harness AI orchestration out-of-the-box.
  - **Finding**: OHC needs a native, unified communication engine that abstracts channel complexity. It must provide a single, zero-configuration inbox for the owner and expose normalized events for the AI agent departments to process asynchronously.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ CONVERSATION : owns
    CONVERSATION ||--o{ MESSAGE : contains
    CONVERSATION ||--o{ CHANNEL_ADAPTER : routes
    MESSAGE ||--o{ EVENT_DELIVERY_ENVELOPE : tracks
    TENANT {
      uuid tenant_id PK
      string name
    }
    CONVERSATION {
      uuid id PK
      uuid tenant_id FK
      string status
      uuid customer_id
    }
    MESSAGE {
      uuid id PK
      uuid conversation_id FK
      string content
      string sender_type
    }
    CHANNEL_ADAPTER {
      uuid id PK
      string channel_type
      jsonb config
    }
  ```

  ### Mobile UX Flow
  - **375px Viewport Optimization**:
    - **Home Screen**: A unified "Inbox" card displaying unread messages across all channels, prioritized by urgency (e.g., booking requests vs. general inquiries).
    - **Conversation View**: Translucent Glass header with customer context (tags, past orders). Messages appear in a continuous, native-feeling chat stream.
    - **Action Drawer**: Swipe up to reveal AI-suggested draft replies, quick quoting tools, and payment link generators.

  ### AI Agent Integration Points
  - **Event Delivery**: Inbound messages are normalized into `EventDeliveryEnvelope` and published to the Redis-backed background job queue.
  - **Multi-Harness Orchestration**: The Customer Support Department (powered by the Universal Provider Facade) consumes the events, retrieves tenant context via Redlock, and streams draft replies back to the conversation.

  ### Key Design Decisions
  - **Native Rust WebSocket / SSE Engine**: To achieve strict latency targets and handle high-throughput typing indicators and read receipts.
  - **Zero Trust & Security**: SPIFFE/SPIRE identity applied at the Channel Adapter boundary. Strict RLS on all Postgres tables using `tenant_id`.
  - **Open Source Leverage**: Use `tokio` for async runtime, `axum` for HTTP/WebSocket routing, and `sqlx` for Postgres interactions.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Objective**: Implement the Native Rust Omnichannel Chat System for OHC.
  **Acceptance Criteria**:
  1. Implement the core native Rust WebSocket and SSE engine for real-time messaging.
  2. Implement the `ChannelAdapter` system supporting at least Web Chat and WhatsApp.
  3. Ensure all PostgreSQL tables (Conversation, Message) enforce RLS using `tenant_id`.
  4. Ensure the system works seamlessly on a 375px mobile viewport without horizontal scrolling, using OHC Translucent Glass materials.
  5. Do NOT prescribe specific database schemas or API endpoints; design the details as needed to fulfill the core capabilities.

  ## Priority
  P0 (critical)

  ## Estimated Scope
  Large
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
