issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance, native Rust omnichannel customer support and chat engine. Owners (like Maya, Carlos, Priya) currently rely on disparate tools for Instagram DMs, web chat, and WhatsApp. Relying on an external third-party service (Chatwoot) introduces latency, breaks our Zero-Trust multi-tenant isolation, and prevents deep integration with OHC AI agents. We need a unified inbox architecture natively in Rust that scales efficiently, supports row-level tenant isolation in PostgreSQL, and seamlessly integrates with OHC's backend and agent orchestration.

  ## Research Report
  - **Codebase Audit:** OHC currently integrates with external services but mandates a shift to a `onehumancorp/mono` native Rust microservices approach.
  - **Chatwoot Source Code Audit:** Chatwoot's core architecture (Ruby on Rails) uses models like `Conversation`, `Message`, `Inbox`, `Channel`, and `Contact`. It heavily relies on WebSockets (ActionCable) for real-time messaging, webhook integrations for channel adapters, and sidekiq for background jobs.
  - **Competitor Systems Audit:** Systems like Shopify Inbox and Stripe Customer Chat use edge-cached routing and highly optimized websocket connections to handle thousands of concurrent tenant connections.
  - **Gap:** OHC lacks a native Rust multi-tenant inbox system capable of real-time WebSocket communication, multi-channel aggregation (Email, SMS, Web Widget, IG), and AI agent interception before manual routing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ Conversation : contains
      Inbox ||--o{ ChannelAdapter : uses
      Conversation ||--o{ Message : has
      Conversation }|--|| Contact : belongs_to
      Message }o--|| AgentBot : handled_by
  ```
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Rust_WebSocket
      participant ChannelAdapter
      participant AI_Agent
      participant Owner_Mobile

      Customer->>ChannelAdapter: Sends Message (Web/IG/WhatsApp)
      ChannelAdapter->>OHC_Rust_WebSocket: Ingest & Normalize
      OHC_Rust_WebSocket->>AI_Agent: Triage & Draft Reply
      AI_Agent-->>OHC_Rust_WebSocket: Draft Ready
      OHC_Rust_WebSocket->>Owner_Mobile: Push Notification (New Message + Draft)
      Owner_Mobile->>OHC_Rust_WebSocket: Approve Draft
      OHC_Rust_WebSocket->>ChannelAdapter: Send Message to Customer
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox View:** A sticky bottom navigation bar with a badge for unread messages. List view shows contacts with unread indicators and AI draft badges.
  2. **Conversation Thread:**
     - Translucent glass app bar with Contact Name and Channel Icon (e.g., IG, Web).
     - Standard chat bubbles (left for customer, right for owner/agent).
     - If AI generated a draft, it appears in a glowing "AI Suggestion" card just above the text input, with "Approve" (1-tap send) or "Edit" buttons.
  3. **Interaction:** Text input expands dynamically. A "+" icon allows attaching quotes, deposit links, or calendar slots directly into the chat.

  ### AI Agent Integration Points
  - **Triage & Auto-Reply:** Upon message ingestion, the `Work Triage` agent evaluates intent. If simple (e.g., "vegan cakes?"), `Customer & Relationship Assistant` drafts a reply.
  - **Workflow Trigger:** Chat commands or recognized intents can trigger `Operations Assistant` (e.g., customer asks to reschedule).
  - **Memory Injection:** AI injects summarized past interactions (tenant-scoped memory) into the owner's chat view for context.

  ### Key Design Decisions
  - **Native Rust WebSockets:** Use `tokio-tungstenite` or Axum WebSockets for high-throughput, low-latency connections.
  - **Strict Multi-Tenancy:** Every table (`conversations`, `messages`, `inboxes`) strictly requires `tenant_id` with Row-Level Security (RLS) in PostgreSQL.
  - **Channel Abstraction:** Create a generic `ChannelAdapter` trait in Rust so adding new channels (SMS, WhatsApp, IG) is modular and requires no core engine changes.

  ## Implementation Prompt
  Implement a native Rust multi-tenant Omnichannel Inbox feature in the backend (Axum/Tonic).
  - **CUJ:** The owner logs into the mobile app, sees a new incoming web-chat message, reads the AI-generated draft, taps "Approve", and the message is sent back to the customer via WebSockets.
  - **Acceptance Criteria:**
    - Create the database migrations for `inboxes`, `conversations`, `messages`, and `contacts` with RLS.
    - Implement Axum WebSocket handlers for real-time bidirectional messaging.
    - Implement the `ChannelAdapter` trait and a default Web Widget channel.
    - Create the UI components for the Unified Inbox and Conversation view in the Tauri app, adhering to 375px mobile constraints and translucent glass styling.
    - Write full E2E Playwright tests covering the owner receiving and replying to a message.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []