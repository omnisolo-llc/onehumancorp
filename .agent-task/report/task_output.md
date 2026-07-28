issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chat system as an external dependency. We need a native Rust omnichannel customer support & chat engine built directly into `onehumancorp/mono` to achieve 100% feature parity with Chat system, ensuring tight multi-tenant isolation, native performance, and a single cohesive work context for the owner. The system must unify messages from Instagram, email, WhatsApp, and web chat into a single prioritized feed.

  ## Research Report
  - **Context:** The task mandate specifies a complete retirement of Chat system as an external service. OHC must implement its own high-performance, multi-tenant chat engine.
  - **Chat system Architecture Audit:** Chat system relies heavily on `Conversation`, `Message`, `Inbox`, `Contact`, `ContactInbox`, and various channel models (e.g. `Channel::WebWidget`, `Channel::Email`, etc.).
  - **Requirements:**
    - **Multi-Tenancy:** Row-level tenant isolation (PostgreSQL RLS with `tenant_id`).
    - **Models:** Core structures for Conversations, Messages, Inboxes, Contacts, and Attachments.
    - **Performance:** Native Rust microservices and crates for high throughput and low latency.
    - **Integration:** API endpoints (REST/gRPC) and real-time capabilities (WebSockets) for seamless frontend integration.

  ## Design Doc
  - **Architecture Diagram (Mental Model / Mermaid equivalent):**
    - `Frontend (Flutter/React)` <-> `Rust WebSocket/API Gateway` <-> `Chat Core (Rust Services)` <-> `PostgreSQL (RLS)`
  - **Mobile UX Flow (375px first):**
    - The chat interface must be optimized for mobile devices with responsive layouts.
    - Conversations list with unread indicators.
    - Unified messaging view supporting text, attachments, and structured rich messages (e.g., product cards, payment links).
    - Quick reply buttons and integrated AI draft suggestions.
  - **AI Agent Integration:**
    - Seamlessly integrate with the existing AI Customer & Relationship Assistant to draft replies and analyze sentiment.
    - AI drafts appear inline within the conversation context for owner review before sending.
  - **Key Design Decisions:**
    - **Language:** Rust for backend services to ensure memory safety, performance, and concurrency.
    - **Database:** PostgreSQL with Row-Level Security for robust multi-tenant data isolation.
    - **Real-time:** WebSockets for live message delivery and typing indicators.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the foundational data models, API endpoints, and a basic WebSocket server in Rust for the new OHC Omnichannel Chat System.
  1. Define the core database schemas (Conversations, Messages, Inboxes, Contacts) incorporating `tenant_id` for RLS.
  2. Create the corresponding Rust structs and Diesel/SeaORM models.
  3. Implement fundamental API routes for creating and retrieving conversations and messages.
  4. Scaffold a basic WebSocket handler for real-time message broadcasting within a specific tenant context.
  5. Ensure comprehensive unit test coverage and E2E tests verifying message flow from API to WebSocket.
  - **Acceptance Criteria:**
    - The system can accept a message via API and broadcast it to connected WebSocket clients for the correct tenant.
    - Database operations respect tenant boundaries.
    - Mobile-responsive frontend components can connect and display real-time updates.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
