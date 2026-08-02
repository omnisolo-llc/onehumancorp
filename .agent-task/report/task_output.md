issue_title: "Architecture Design: Native Rust Multi-Tenant Omnichannel Chat Engine"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as an external, third-party dependency for handling customer interactions. The current external dependency on Chatwoot breaks our multi-tenant isolation, adds network latency, and violates our Zero-Trust architecture goals. We need a native, high-performance omnichannel chat system built in Rust that seamlessly integrates with our existing Go + Bazel backend and PostgreSQL database, preserving strict multi-tenancy and providing low-latency real-time WebSocket messaging for our owner/operator personas (e.g., Maya, Carlos, Priya).

  ## Research Report & Findings
  An audit of the Chatwoot architecture (https://github.com/chatwoot/chatwoot) reveals the following core structural requirements for a modern omnichannel chat system:
  1. **Omnichannel Data Models**: Needs abstract representations for `Inbox`, `Conversation`, `Message`, and `Contact`.
  2. **Channel Adapters**: Need modular adapters for Email, Web Widget, WhatsApp, Instagram DMs, etc. Chatwoot handles this via STI (Single Table Inheritance) or polymorphic associations.
  3. **WebSocket Real-time Messaging**: Chatwoot relies on ActionCable for real-time pub/sub. For OHC, a native Rust implementation using Tokio and WebSockets (e.g., using `tungstenite` or `axum`) connected to Redis Pub/Sub will offer much higher throughput and lower memory footprint.
  4. **Agent & Routing Logic**: Requires SLA policies, macro execution, and intelligent routing of incoming messages.

  **Competitor Analysis (Shopify / Wix / GoDaddy)**:
  - Shopify Inbox provides a deeply integrated, first-party native chat experience where order details are visible in the chat pane.
  - Wix Inbox unifies messages, but often feels disjointed from the core store.
  - OHC's competitive advantage lies in integrating this chat directly with our AI Job Queue (PostgreSQL SKIP LOCKED) and AI Assistant Capabilities (e.g., Customer & Relationship Assistant) to auto-draft replies.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    A[Client Web/Mobile] -->|WebSocket| B(Rust Chat Service)
    C[Instagram / Webhooks] -->|HTTP| B
    B -->|Pub/Sub| D[(Redis)]
    B -->|Read/Write| E[(PostgreSQL)]
    B -->|gRPC| F[Go Core Backend / AI Queue]
  ```

  ### Mobile UX Flow (375px First)
  - **Screen 1**: Unified Inbox View. A simple list of active conversations, clearly badged with source (Instagram, Web, etc.). Touch targets > 44px.
  - **Screen 2**: Chat Detail View. Transparent glass header. Auto-drafted AI reply pre-filled in the text area for the owner (Maya/Carlos) to approve with a single tap.
  - **Flow**: User receives push notification -> Taps to open Chat Detail -> Taps "Send AI Draft" -> Message marked complete and routed back via Rust service.

  ### AI Agent Integration
  - When a new `Message` is created in Postgres by the Rust Chat Service, an asynchronous job is enqueued in the AI Job Queue.
  - The Customer Assistant AI dequeues the job, analyzes context, and drafts a reply, saving it to the `Message` table as a `draft` status, which triggers a WebSocket event back to the UI.

  ### Key Design Decisions
  - **Rust + Axum + Tokio**: For maximum concurrency and memory safety in handling thousands of concurrent WebSocket connections.
  - **PostgreSQL RLS**: Row-Level Security on `tenant_id` must be enforced on all chat tables (Inboxes, Conversations, Messages).
  - **Stateless Chat Nodes**: The Rust chat service must be stateless. All pub/sub routing goes through Redis to allow horizontal scaling in Kubernetes.

  ## Implementation Prompt
  **Goal**: Implement the core data models and Rust WebSocket service for OHC's native omnichannel chat system.
  **CUJ**: A home baker (Maya) receives an Instagram DM. The webhook hits our Rust service, creates a `Message` in a `Conversation`, and pushes the event over WebSocket to Maya's 375px mobile app, where she sees the message instantly.
  **Acceptance Criteria**:
  1. Define Rust structs/schema for `Inbox`, `Conversation`, and `Message` with `tenant_id`.
  2. Implement a WebSocket endpoint in Rust (using Axum) that authenticates via SPIFFE/SPIRE (or current token system) and subscribes to Redis.
  3. Ensure all DB operations use tenant-scoped RLS.
  4. Write automated integration tests proving multi-tenant isolation (Tenant A cannot see Tenant B's messages over WebSocket).

  ## Priority & Scope
  **Priority**: P0
  **Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust]
assignees: []