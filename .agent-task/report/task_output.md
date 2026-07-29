issue_title: "Implement Native Multi-Tenant Omnichannel Support & Real-Time Chat Engine in Rust"
issue_description: |
  ### Problem Statement
  Our primary business personas (Priya the boutique owner, Maya the custom baker, and Carlos the handyman) rely on unified communication to handle customer inquiries, capture incoming demand, coordinate services, and collect deposits. The previous third-party omnichannel integration (CW) has been fully retired and deprecated. To maintain absolute data privacy, multi-tenant isolation, and high performance, OHC requires a fully native, robust support & chat engine built directly in Rust.

  This native engine must unburden non-technical operators by consolidating messaging channels (Web Chat, DMs, Webhooks) into a single, assistant-guided commands center.

  ### Discovered Architectural & Scaling Gaps
  A comprehensive benchmarking of leading support platforms (including the retired CW service architecture) highlights several key requirements for a production-grade system:
  1. **Omnichannel Data Models**: A unified messaging layer must represent conversations, contacts, message attachments, and channel-specific configurations seamlessly.
  2. **Multi-Tenant Row-Level Security (RLS)**: Under PostgreSQL, every query must be locked to the active tenant's context using the `tenant_id` session configuration.
  3. **High-Performance Ingress**: Standard HTTP webhooks from external providers can experience sudden load spikes. The endpoint must parse and enqueue payloads rapidly.
  4. **Secure, Real-Time Delivery**: Clients need instant updates. A secure WebSocket delivery gateway using single-use signed tickets is required to prevent unauthorized connection hijacking.

  ### Proposed System Architecture (System Design)

  #### 1. Data Schema & Tenant Isolation
  Every entity is securely isolated at the database layer using PostgreSQL Row-Level Security:

  ```mermaid
  erDiagram
      chat_inboxes {
          uuid id PK
          uuid tenant_id
          text name
          timestamptz created_at
      }
      chat_channels {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          text channel_type
          jsonb config
      }
      chat_contacts {
          uuid id PK
          uuid tenant_id
          text name
          text email
          text phone
      }
      chat_conversations {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id
          text status
      }
      chat_messages {
          uuid id PK
          uuid tenant_id
          uuid conversation_id FK
          text sender_type
          uuid sender_id
          text content
      }

      chat_inboxes ||--o{ chat_channels : "has"
      chat_inboxes ||--o{ chat_conversations : "contains"
      chat_contacts ||--o{ chat_conversations : "initiates"
      chat_conversations ||--o{ chat_messages : "receives"
  ```

  #### 2. Secure Real-Time Delivery Protocol
  To prevent unauthorized access to WebSocket channels:
  1. The client requests a temporary connection ticket via an authenticated REST endpoint (`POST /api/chat/ws-ticket`).
  2. The server generates a cryptographically signed, single-use ticket (e.g., HMAC-SHA256 containing `tenant_id`, `user_id`, and a short expiration epoch) and stores it in Redis with a 30-second TTL.
  3. The client establishes a WebSocket connection by passing the ticket as a query parameter: `ws://api.ohc.corp/ws?ticket=<ticket_id>`.
  4. The WebSocket server validates the ticket signature, checks Redis for single-use consumption, authorizes the channel, and subscribes the socket connection to NATS streams.

  ### Implementation Prompt for Engineering Swarm

  **Outcome Description**:
  Implement a production-grade, native multi-tenant Omnichannel Support & Real-Time Chat Engine in Rust under `src/server/services/chat/`. Provide Axum-based API controllers for webhook ingest, conversation management, secure WebSocket ticket dispensing, and a real-time WebSocket connection handler.

  **Acceptance Criteria & CUJ**:
  1. **Multi-Tenant Isolation**: All chat service DB writes and reads must enforce tenant boundaries via PostgreSQL session configuration `app.current_tenant_id`.
  2. **WebSocket Authentication**: Deny connections lacking a valid, single-use signed ticket. Return JSON error responses for invalid tickets.
  3. **Visual Excellence (Grandmother Test)**: The frontend component must work seamlessly on a 375px mobile screen without horizontal scroll, adopting a translucent glass macOS aesthetic with clean card layouts.
  4. **Practically Tested**: Achieve 100% unit test coverage for the service layer and WebSocket authorization module. Provide comprehensive Playwright integration tests.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
