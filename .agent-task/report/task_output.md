issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Title: Native Rust Omnichannel Chat System

  ## Problem Statement
  OHC needs an integrated, omnichannel customer support and chat engine to serve small-business personas like Maya (baker), Carlos (handyman), and Priya (boutique owner). Previously, OHC relied on an external Chatwoot dependency, which violated our Zero-Trust architecture, created data silos, and hampered unified multi-tenant AI capabilities. A non-technical owner needs one unified inbox on their 375px mobile screen that seamlessly merges Instagram DMs, WhatsApp, SMS, Web Chat, and Email into actionable tasks and AI drafts, without managing third-party tools.

  ## Research Report
  - **Chatwoot Source Code Audit**: Benchmarked `https://github.com/chatwoot/chatwoot`. Chatwoot uses a robust domain model centered around `Account` (Tenant), `Inbox`, `Conversation`, `Message`, and `Contact`. It uses Channel adapters (`Channel::WebWidget`, `Channel::Whatsapp`, `Channel::TwitterProfile`, `Channel::Email`, etc.) to normalize external payloads into unified `Message` records.
  - **Competitor Analysis**: Shopify Inbox, Meta Business Suite, and Wix Inbox all provide first-party, tightly integrated communication hubs. They maintain strict tenant isolation and low-latency real-time updates without forcing users into separate admin portals.
  - **OHC Technical Gap**: We are missing a native Rust messaging core that implements Chatwoot's unified channel model but uses OHC's internal multi-tenant PostgreSQL/RLS, Redis Redlock for AI concurrency, and gRPC for high-performance agent processing.

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o| CHANNEL_ADAPTER : configured_with

      TENANT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string channel_type
          jsonb config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier
          string name
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          int message_type
          string sender_type
      }
  ```

  ### 2. UI Wireframes & 375px Mobile UX Flow
  - **The Work Feed (Unified Inbox)**: The home screen immediately shows urgent messages in a simple list.
  - **Conversation Screen (Mobile-First)**: A 375px-optimized chat view with macOS Translucent Glass styling. Top bar shows Contact name & channel icon (e.g., Instagram). Bottom has a native mobile keyboard input and a one-tap "Generate AI Draft" button.
  - **Translucent Glass Material**: Message bubbles use slight transparency and blur filters, matching the clean UniFi modular dashboard style. No complex agent routing controls visible to the owner.

  ### 3. AI Agent Integration Points
  - **Work Triage Agent**: Hooks into `Message` creation (via PostgreSQL SKIP LOCKED queue). Automatically classifies urgency and flags the conversation.
  - **Customer & Relationship Agent**: Reads the `Conversation` history and tenant-scoped `Contact` memory to pre-draft context-aware replies (e.g., checking Maya's baking schedule before replying about a cake order).
  - **Distributed Locks**: Uses Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) to ensure two AI agents do not draft or send replies simultaneously.

  ### 4. Key Design Decisions
  - **Complete Chatwoot Replacement**: All logic is built natively in Rust. No third-party Chatwoot services.
  - **Strict Row-Level Security (RLS)**: Every table (`inboxes`, `conversations`, `messages`, `contacts`) includes a mandatory `tenant_id` and utilizes PostgreSQL RLS to guarantee tenant isolation.
  - **Normalized Channels**: All inbound messages, regardless of source (WhatsApp, IG, Web), are normalized into a standard `Message` struct in Rust.
  - **Mobile-Parity & Offline**: The Flutter frontend stores recent conversations locally. Critical writes (like sending a message) are queued and retry gracefully on flaky networks.

  ## Implementation Prompt
  **Persona**: Maya (Baker)
  **Objective**: Build the core Rust gRPC microservice and PostgreSQL database migrations for the new Omnichannel Inbox, replacing Chatwoot.
  **Acceptance Criteria**:
  1. Implement Rust structs and PostgreSQL migrations for `Inbox`, `Contact`, `Conversation`, and `Message`, with strict `tenant_id` RLS policies.
  2. Implement a unified gRPC API for the Flutter mobile app to fetch conversations, send messages, and receive real-time updates.
  3. Create a placeholder Channel Adapter trait in Rust for processing inbound external webhooks (e.g., from WhatsApp/IG).
  4. Ensure 100% unit test coverage for the Rust service layer and database models.
  5. The Flutter frontend must render a 375px-friendly Conversation view with placeholder UI for AI drafted replies.
  *Note: Do not prescribe the exact database schema fields or exact gRPC methods. The implementer should design those based on the Chatwoot audit principles and this brief.*

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
