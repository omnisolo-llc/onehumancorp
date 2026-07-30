issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Title: Native Rust Omnichannel Chat System & Universal Inbox

  ## Problem Statement
  Small business owners (Maya, Carlos, Priya) communicate with their customers across numerous fragmented channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email). Previously, OHC relied on an external third-party service (Chatwoot) for omnichannel capabilities. This external dependency introduces latency, complicates multi-tenant data isolation, creates a disjointed user experience, and limits our AI agents' ability to deeply integrate with real-time conversations. Owners need a native, high-performance, embedded Universal Inbox that consolidates all customer communications seamlessly into their mobile feed without relying on external SaaS products.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses a Ruby on Rails backend with PostgreSQL and Redis. Its architecture centers around `Account` (Tenant), `Inbox`, `Conversation`, `Message`, and various `Channel::*` models (API, Email, Facebook Page, Instagram, SMS, Twilio, WhatsApp, Web Widget). It handles real-time updates via WebSockets and relies heavily on background workers for channel syncing.
  - **Shopify Inbox:** Provides a unified inbox but is tightly coupled with Shopify's ecosystem. It supports Apple Business Chat and Instagram but lacks open, extensible channel adapters.
  - **OHC Opportunity:** By building a native Rust omnichannel engine, we can achieve significantly higher performance (lower memory footprint, faster WebSocket broadcasting via Tokio/Tungstenite or Axum), deeper integration with OHC's Zero-Trust multi-tenancy model (SPIFFE/SPIRE, row-level security), and seamless native integration with OHC's "Ambassador" AI Agent for autonomous drafting.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Mobile App / Web UI] <-->|WebSocket/REST| Gateway(API Gateway)
      Gateway <--> Router[Rust Omnichannel Service]

      subgraph Rust Omnichannel Microservice
          Router --> Auth[Auth & Tenant Middleware]
          Auth --> WSM[WebSocket Manager]
          Auth --> REST[REST Controllers]

          REST --> Core(Core Engine)
          WSM --> Core

          Core --> DB[(PostgreSQL - RLS Enabled)]
          Core --> Redis[(Redis Pub/Sub)]

          subgraph Channel Adapters
              Core --> IG[Instagram Adapter]
              Core --> WA[WhatsApp Adapter]
              Core --> SMS[SMS/Twilio Adapter]
              Core --> Web[Web Widget Adapter]
          end
      end

      IG <--> MetaAPI[Meta Graph API]
      WA <--> WhatsAppAPI[WhatsApp Cloud API]
      SMS <--> Twilio[Twilio API]

      Core -.-> EventMesh[Event Mesh / Kafka]
      EventMesh -.-> AIAgent[The Ambassador Agent]
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  - **Unified Inbox Feed (Mobile Home):**
    - The first screen presents a clean list of active conversations, using Apple-style translucent materials.
    - Each conversation card displays the customer's avatar, channel icon (e.g., small Instagram logo overlay), customer name, snippet of the last message, and an unread indicator.
  - **Conversation View:**
    - Tapping a card opens the chat. The top bar shows the customer name and channel.
    - Chat bubbles are rendered with clear distinction between inbound (customer) and outbound (owner/agent).
    - If the Ambassador AI drafted a response, a highlighted "Drafted by AI" bubble appears at the bottom with a primary "Approve & Send" button and a secondary "Edit" button.
  - **Message Composition:**
    - Bottom input field uses native mobile keyboards. Includes an attachment icon (for photos/quotes).
  - **Zero-Jargon:** No mention of "channels", "inboxes", or "webhooks". Just "Messages".

  ### AI Agent Integration Points
  - **The Ambassador Agent:** Subscribes to the Event Mesh for `message.created` events. When an inbound message arrives, it retrieves customer context from the core database and automatically drafts a response payload. This draft is sent back via internal API as a `Message` with status `draft`.
  - **The Manager Agent:** Monitors conversations for keywords implying booking or purchasing (e.g., "Can I book for Tuesday?") and silently annotates the conversation context with available slots or inventory, visible only to the owner.

  ### Key Design Decisions
  - **Native Rust Implementation:** Replacing Ruby on Rails (Chatwoot) with Rust (Axum/Tokio) for the core service ensures maximum concurrency for WebSocket connections and minimizes latency.
  - **Strict Multi-Tenancy:** PostgreSQL schemas must implement Row-Level Security (RLS) keyed by `tenant_id` for every table (`inboxes`, `conversations`, `messages`, `contacts`).
  - **Adapter Pattern for Channels:** Each external channel (Instagram, WhatsApp) will implement a strict Rust Trait (`ChannelProvider`) to normalize incoming/outgoing payloads into a unified `Message` struct.
  - **Proactive AI Drafting:** The system is optimized for an "Approve to Send" workflow rather than just "Read and Type", deeply embedding the AI into the message lifecycle.

  ## Implementation Prompt
  **Objective:** Implement the core Native Rust Omnichannel Chat Service backend and corresponding Flutter mobile UI components to replace Chatwoot.
  **Target User:** Maya (Baker), who needs to see her Instagram DMs and WhatsApp messages in one feed and approve AI-drafted replies on her iPhone.
  **Acceptance Criteria:**
  1. Initialize a new Rust crate/service within the monorepo for the omnichannel engine.
  2. Implement the PostgreSQL database schema (with `tenant_id` RLS) for `Contacts`, `Conversations`, and `Messages`.
  3. Implement the `ChannelProvider` trait and create a robust Mock/Local adapter for testing.
  4. Implement the WebSocket endpoints for real-time message broadcasting to the Flutter frontend.
  5. Build the 375px-optimized Flutter UI for the Unified Inbox list and Conversation view.
  6. E2E Playwright verification: A message ingested via the Mock adapter appears in real-time on the frontend, and an owner reply is successfully routed back through the adapter.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
