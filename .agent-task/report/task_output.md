issue_title: "[Research] OHC Native Rust Chat Engine Architecture"
issue_description: |
  # Native Rust Chat Engine Architecture Research Report

  ## 1. Executive Summary & Problem Statement
  **Problem Statement:** OneHumanCorp is currently dependent on a third-party omnichannel chat service (Chatwoot), which violates the platform's multi-tenancy requirements, scales poorly with our Zero-Trust architecture (SPIFFE/SPIRE), and compromises the core platform values of embedded AI workflows and owner data isolation. To align with OHC's product architecture, Chatwoot must be fully retired, and its functionality must be natively replicated in Rust within the `onehumancorp/mono` codebase.

  **Business Persona Gap:** Users like Maya (Baker handling Instagram DMs), Carlos (Handyman messaging SMS leads), and Nora (Agency dealing with Email + Slack requests) need a single, instantly responsive, secure, and AI-assisted inbox. A third-party provider slows down response times, fractures data context for AI agents, and creates a disconnected UI experience that fails our standard for a simple, unified owner control center.

  ## 2. Research Report
  ### Benchmarking Chatwoot
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals several core components that must be rebuilt natively:
  1.  **Channel Adapters:** Interfaces for Web Widget, API, Email, Facebook/Instagram, Twitter, Twilio/SMS, LINE, WhatsApp, and Telegram.
  2.  **Conversational Data Model:** `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Agent`, `Team`, `Label`.
  3.  **Real-time Engine:** WebSocket connections pushing incremental updates (new messages, typing indicators, presence).
  4.  **Automation Engine:** Macros, SLA policies, and Webhook dispatching.

  ### Competitive Analysis
  *   **Shopify Inbox:** deeply integrated with orders and products; AI suggests replies based on shop data.
  *   **Stripe / Wix:** Highly unified context but weaker multi-channel support.
  *   **OHC Advantage:** By building this natively in Rust, our AI assistants (Customer & Relationship Assistant, Work Triage) can access the data instantaneously without API hopping. This allows our "Work Triage" to intercept messages *before* the human sees them, categorize them, and draft replies instantaneously.

  ## 3. Design Doc: High-Level Architectural Design

  ### 3.1. Architecture Diagram
  ```mermaid
  graph TD
      Client[Client Web/Mobile App] --> |WebSocket / gRPC| API[Rust Chat API Gateway]
      API --> Auth[SPIFFE/SPIRE Identity & Auth]
      Auth --> Router[Message Router & Dispatcher]

      Router --> |CRUD| DB[(PostgreSQL - RLS Tenant Isolated)]
      Router --> |Cache / PubSub| Redis[(Redis - PubSub)]

      Router --> ChannelAdapters[Channel Adapters Node]
      ChannelAdapters --> |API| WebWidget[Web Widget]
      ChannelAdapters --> |API| Email[Email Provider]
      ChannelAdapters --> |API| Social[WhatsApp / IG / SMS]

      Router --> AILayer[OHC AI Work Triage]
      AILayer --> Agent[Customer Assistant Agent]
  ```

  ### 3.2. Mobile UX Flow (375px First)
  *   **View 1: Unified Work Feed (Home):** Messages appear in the main feed alongside tasks and bookings, not hidden in a separate "Inbox" tab.
  *   **View 2: Conversation Detail:** A clean, translucent chat interface. The input field defaults to "Draft with AI" or quick replies derived from context (e.g., "Send Payment Link").
  *   **View 3: Customer Context Drawer:** Swiping left reveals the customer's purchase history, active tasks, and tags—critical for personas like Priya (Boutique) and Nora (Agency).

  ### 3.3. Key Design Decisions & Why
  *   **Native Rust:** Guarantees memory safety, extreme concurrency for WebSockets, and low-latency throughput for real-time messaging.
  *   **Tenant-Isolated PostgreSQL (RLS):** Every row in the chat database must have `tenant_id` and strict Row Level Security to prevent cross-account data leaks.
  *   **AI-First Ingestion:** Messages do not just go to a database; they are first published to a topic where the AI Work Triage agent assesses them for intent (e.g., "Is this a new order?").

  ## 4. Implementation Prompt
  **Target Implementer:** Backend / Full-Stack Agent

  **Objective:** Implement the foundational Native Rust Chat Engine to replace Chatwoot.

  **Requirements:**
  1.  **Data Models & Database:** Design the core entities (`Inbox`, `Conversation`, `Message`, `Contact`) adhering to strict multi-tenant isolation via `tenant_id` and PostgreSQL RLS.
  2.  **API Surface:** Define gRPC and REST (via OpenAPI) endpoints for sending/receiving messages and fetching conversation history.
  3.  **Real-Time Subsystem:** Implement a WebSocket handling mechanism (e.g., using `tokio-tungstenite` or similar async networking in Rust) backed by Redis Pub/Sub for cross-node delivery.
  4.  **Channel Adapter Trait:** Define an extensible Rust trait/interface for integrating various external channels (starting with a simple "Web API" channel).
  5.  **Integration:** Ensure the API gateway securely authenticates requests via SPIFFE/SPIRE.
  6.  **Verification:** Write comprehensive Rust unit tests (100% coverage requirement) and add Playwright E2E tests validating the basic send/receive flow in the mobile-responsive UI.

  **Acceptance Criteria:** A user can send a message via the API, the message is stored correctly with tenant isolation, and the message is broadcasted via WebSocket to connected clients.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
