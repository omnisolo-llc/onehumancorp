issue_title: "Native Rust Omnichannel Chat & Support Architecture"
issue_description: |
  ## Problem Statement
  OHC requires a deeply integrated, high-performance omnichannel chat system. Historically, we relied on Chat system as an external service, which created friction in multi-tenant isolation, data synchronization, and latency. For owners like Maya (baker) and Carlos (handyman), missing an Instagram DM or a web chat directly translates to lost revenue. We need a native Rust solution within OHC that guarantees zero-trust multi-tenancy, sub-50ms latency, and seamless offline-tolerant mobile UX.

  ## Research Report
  - **Chat system Source Code Audit**: An audit of `chat_system/chat_system` reveals a robust but conventional Rails architecture. Core entities include `Account` (Tenant), `User`, `Contact`, `Inbox`, `Conversation`, `Message`, and `Channel` variants (e.g., Email, WhatsApp, Line, WebWidget).
  - **WebSocket Architecture**: Chat system uses ActionCable for real-time messaging. For OHC, a native Rust implementation using `tokio-tungstenite` or `axum` WebSockets will provide significantly higher throughput and lower memory footprint per connection.
  - **Data Model Translation**:
    - Chat system's `Account` maps to OHC's `Tenant`.
    - `Inbox` groups channels and routing rules.
    - `Conversation` links a `Contact`, an `Inbox`, and `Messages`.
  - **Competitor Insights**: Shopify Ping and Wix Inbox excel at tying conversations directly to orders/carts. Our system must embed operational context (quotes, bookings) directly into the message stream.

  ## Design Doc
  ### High-Level Architecture
  - **Microservice**: `ohc-chat-engine` (Rust, Axum, Tokio).
  - **Database**: PostgreSQL (tenant-isolated via RLS). Tables: `inboxes`, `channels`, `contacts`, `conversations`, `messages`, `attachments`.
  - **Real-time Layer**: Axum WebSockets with Redis Pub/Sub for multi-node message broadcasting.
  - **Storage**: MinIO/GCS for attachments (images, voice notes).

  ### Mermaid Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      USER ||--o{ MESSAGE : sends
      MESSAGE ||--o| ATTACHMENT : includes
  ```

  ### Mobile UX Flow (375px First)
  - **Home/Triage**: A unified feed showing unread messages, pending quotes, and alerts.
  - **Conversation View**: Clean, Apple iMessage-like UI. Translucent top/bottom bars. Inline action buttons for "Generate Quote" or "Request Deposit".
  - **Offline Capability**: Flutter local SQLite cache. Messages sent offline are queued and sync automatically upon reconnection.

  ### AI Agent Integration
  - **Work Triage Agent**: Subscribes to new messages, categorizes intent (e.g., inquiry, complaint), and drafts suggested replies.
  - **Sales Agent**: Automatically parses dimension/preference requests from DMs (e.g., "vegan cake") and prepares structured quotes inline.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  **To the Implementer:**
  Implement the native Rust omnichannel chat system (`ohc-chat-engine`).
  - Set up the Axum WebSocket server and Redis Pub/Sub integration.
  - Create the Postgres schema (RLS-enabled) for `inboxes`, `conversations`, and `messages`.
  - Implement the Flutter UI for the unified inbox and conversation view, strictly adhering to the 375px mobile-first and translucent glass design system.
  - Integrate the Work Triage AI agent to auto-draft replies for incoming messages.
  - **Acceptance Criteria**: A real owner (e.g., Maya) can receive a web-widget message, see it in the unified OHC inbox on mobile, view an AI-drafted reply, and send it—all with sub-50ms perceived UI latency.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
