issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The system currently relies on either legacy external integrations or disjointed basic endpoints for chat capabilities. Small business owners (like Carlos the handyman or Maya the baker) need a unified, high-performance, and deeply integrated omnichannel support inbox (SMS, Web Chat, Email, IG/WhatsApp DMs). Relying on external dependencies like Chatwoot breaks the unified Multi-Tenant Architecture, introduces network latency, and complicates compliance and Zero-Trust isolation. The mandate explicitly requires the 100% retirement of Chatwoot as an external service in favor of a native Rust implementation.

  **Research Report**
  A deep dive into Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) reveals the core data models driving an omnichannel inbox. These must be replicated natively in Rust to achieve feature parity while adhering to OHC's architecture.

  - *Data Models Replicated*: `Account`, `Inbox` (Channel agnostic routing), `Contact` (Customer profiles), `Conversation` (Thread grouping), and `Message` (Individual interactions).
  - *Multi-Tenancy*: Every data access layer must be scoped to an `account_id` (OHC `tenant_id`) ensuring strict tenant isolation.
  - *Extensibility*: A polymorphic `channel` design is critical for integrating with Twilio (SMS), Sendgrid (Email), and Meta (IG/WA).

  **Design Doc**
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        Tenant ||--o{ Inbox : configures
        Tenant ||--o{ Contact : owns
        Inbox ||--o{ Conversation : hosts
        Contact ||--o{ Conversation : participates
        Conversation ||--o{ Message : contains
        Tenant ||--o{ Agent : employs
        Agent ||--o{ Conversation : handles
    ```
  - **Mobile UX Flow (375px first)**:
    - *Home Screen*: A clean, unified "Inbox" view prioritizing unassigned and open conversations.
    - *Conversation View*: A translucent, Apple-style messaging interface with quick-action buttons for AI-assisted drafting (Gemini/MiniMax integration).
    - *Customer Profile Slide-over*: Swiping left in a conversation reveals the `Contact` details, previous orders, and custom attributes.
  - **AI Agent Integration Points**:
    - `Customer Assistant Agent`: Listens for new `Message` events on a queue and drafts suggested replies based on the tenant's knowledge base and previous conversation context.
    - `Triage Agent`: Automatically categorizes new `Conversation` instances based on NLP analysis of the initial message and assigns priority/tags.
  - **Key Design Decisions**:
    - Use Rust's `axum` for HTTP API routing and `tonic` for internal gRPC communications between the chat microservice and other OHC components.
    - Persist chat data in PostgreSQL with strict Row Level Security (RLS) policies tied to the `tenant_id`.
    - Implement real-time updates via WebSockets, integrated with the existing `hub.rs` / `msgbus.rs` architecture for pub/sub.

  **Implementation Prompt**
  Build the core backend infrastructure for the native Rust omnichannel chat system.

  *Tasks:*
  1.  **Database Schema**: Define the SQL migrations (e.g., in `src/server/migrations/`) to create the `inboxes`, `contacts`, `conversations`, and `messages` tables. Ensure every table includes a `tenant_id` column and appropriate foreign key constraints. Set up RLS policies.
  2.  **Rust Data Models**: Create the corresponding Rust structs (using `sqlx` or `diesel` as per repo standards) representing these entities in `src/server/models/chat.rs` (or equivalent).
  3.  **API Endpoints**: Implement the CRUD operations for Inboxes and Contacts, and the core messaging endpoints (create conversation, send message, list messages) in `src/server/api/chat.rs`.
  4.  **Real-time Bus**: Integrate message creation with the internal pub/sub system (`msgbus.rs`) to emit events when a new message is received or sent.
  5.  **Multi-tenant Middleware**: Ensure all endpoints enforce tenant isolation using the existing authentication and authorization middleware.

  *Acceptance Criteria:*
  - All database tables are created successfully via migrations.
  - Rust models accurately reflect the schema and support multi-tenant queries.
  - REST/gRPC endpoints function correctly (tested via curl or Playwright).
  - Unit tests achieve 100% coverage for the new code.
  - No external Chatwoot dependencies are introduced.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
