issue_title: "Design Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Replace Chatwoot with Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot as an external third-party service for omnichannel customer support and chat functionality. The mandate dictates that Chatwoot must be **100% RETIRED**. OHC must implement its own high-performance, multi-tenant omnichannel chat engine natively in Rust within the `onehumancorp/mono` repository to ensure deep integration, unified data modeling, superior performance, and strict Zero-Trust tenant isolation.

  Relying on an external service creates friction for the owner/operator personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) as their data is split between systems, creating latency and preventing seamless AI agent interactions across their entire business context. A native solution is required to truly deliver the "One Assistant" promise.

  ## Research Report
  ### The Chatwoot Architecture Audit
  I performed a clone and audit of the open-source Chatwoot repository (`https://github.com/chatwoot/chatwoot`). The key architectural components needed for replication in OHC are:
  - **Models/Entities**:
    - `Account` (maps to OHC `Tenant`)
    - `User` (Owner/Agent) & `Contact` (Customer)
    - `Inbox` (Channel aggregator)
    - `Channel` variants (Web Widget, API, Email, Facebook, Twitter, WhatsApp, SMS, Line, Telegram, etc.)
    - `Conversation` & `Message`
    - `AgentBot` (maps to OHC AI Agents)
    - Additional features: `CannedResponse`, `Macro`, `Note`, `AutomationRule`, `Label`, `Campaign`, `SlaPolicy` (though some of these are naturally handled by OHC's AI agents).
  - **Communication**:
    - WebSockets (ActionCable in Chatwoot) for real-time updates.
    - Webhooks for external integrations.
  - **AI Integration**:
    - AgentBots in Chatwoot act as middle-men. In OHC, AI agents will be first-class citizens directly reading from and writing to the native chat data models.

  ### Competitive Analysis
  - **Shopify Inbox**: Deeply integrated into the commerce platform. Chat context is inherently tied to orders and products. OHC must achieve this level of integration natively.
  - **Stripe**: Handles complex multi-tenant data with strict isolation. OHC's Rust backend must emulate Stripe's rigorous data modeling and API design.
  - **Chatwoot (Ruby on Rails)**: While feature-rich, the RoR architecture is heavy and external. Moving to a native Rust microservice/crate structure in OHC will drastically improve performance, reduce memory footprint, and allow native integration with OHC's existing Rust/Go/K8s ecosystem.

  ## Design Doc
  ### High-Level Architecture
  The native chat system will be built as a set of Rust crates within the OHC monorepo, communicating via gRPC internally and exposing REST/WebSockets externally.

  ```mermaid
  graph TD
      Client(Mobile/Web PWA) --> API_Gateway(OHC API Gateway)
      Widget(Customer Web Widget) --> API_Gateway
      Webhook(External Channels: IG, WhatsApp) --> API_Gateway

      API_Gateway --> |REST/WebSockets| Chat_Service(Rust Chat Service)
      API_Gateway --> |gRPC| Core_Service(OHC Core Service)

      Chat_Service --> |gRPC| Core_Service
      Chat_Service --> |Read/Write| DB[(PostgreSQL)]
      Chat_Service --> |Pub/Sub, Locks| Cache[(Redis)]

      Chat_Service --> |gRPC / Event Queue| AI_Agents(OHC AI Agents)
      AI_Agents --> |gRPC| Chat_Service
  ```

  ### Data Model & Invariants
  - **Tenant Isolation**: Every table MUST include `tenant_id`. PostgreSQL Row Level Security (RLS) MUST be enabled and enforced.
  - **Entities**:
    - `ohc_chat_inboxes`: Aggregates channels for a tenant.
    - `ohc_chat_channels`: Represents a specific connection (e.g., "Instagram DM", "Web Widget").
    - `ohc_chat_contacts`: The external customer interacting via a channel.
    - `ohc_chat_conversations`: A thread between a contact and the business (handled by owner or AI).
    - `ohc_chat_messages`: Individual messages within a conversation. Supports text, attachments, templates.
  - **Real-time**:
    - Rust-based WebSocket server (e.g., using `tokio-tungstenite` or `axum` WebSockets) pushing events (`message.created`, `conversation.updated`) to connected clients.
    - Redis Pub/Sub for horizontal scaling of WebSocket nodes.

  ### Mobile-First UX Flow (375px)
  - **Unified Inbox View**: The primary screen on mobile shows a consolidated list of active conversations (Work Triage).
  - **Conversation View**: Native chat UI styling. "Reply as AI" or "Draft Reply" prominently featured.
  - **Context Panel**: Tapping a contact's avatar slides over a panel showing their order history, preferences, and upcoming bookings (pulling from OHC Core).

  ### AI Agent Integration
  - OHC AI Agents will subscribe to `message.created` events via the internal event queue.
  - Agents can automatically draft replies, classify intent, update conversation status, or trigger workflows (e.g., creating an order) based on chat content.
  - Agent actions are stored in the database and visible in the UI with a clear "AI Draft" or "AI Action" visual indicator (using the premium Translucent Glass design tokens).

  ## Implementation Prompt
  **Target Persona**: Maya (Home Baker) & Carlos (Field Service)
  **Objective**: Implement the foundational Rust data models and gRPC API for the native OHC chat system to replace Chatwoot, focusing on the core Inbox, Conversation, and Message entities.
  **CUJ**:
  1. Maya opens OHC on her phone.
  2. She receives a new message via a simulated Instagram DM webhook.
  3. The system creates a Contact, Conversation, and Message natively.
  4. The UI updates in real-time (via WebSocket) to show the new message in her unified inbox.
  5. Maya types a reply and sends it.

  **Acceptance Criteria**:
  - Chatwoot integration code is completely removed.
  - Native Rust crates for chat models and APIs are implemented.
  - PostgreSQL schema includes tables for inboxes, channels, contacts, conversations, and messages with strict `tenant_id` RLS.
  - Basic WebSocket infrastructure for real-time updates is functional.
  - AI Agent event hooks are defined.
  - Comprehensive unit tests (100% coverage) and Playwright E2E tests verifying the CUJ.
  - The UI reflects the new native chat system without performance degradation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
