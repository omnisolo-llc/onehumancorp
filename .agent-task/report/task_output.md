issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System Implementation

  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from the legacy external dependency for omnichannel customer support and chat functionality. The goal is to build a high-performance, multi-tenant, omnichannel chat engine natively in Rust inside `onehumancorp/mono`. This new system must support WhatsApp Business, Web Widget integrations, and other channels while strictly adhering to tenant isolation (Row Level Security via `tenant_id`). It must also integrate seamlessly with the AI Agent Triage to categorize incoming messages and automate responses, serving non-technical owners like Maya (baker) and Carlos (handyman) effectively.

  ## Research Report
  - **Codebase Audit:** The current `src/server/integrations/chat/` directory is essentially empty (only a `README.md` exists) and acts as a placeholder.
  - **Source Benchmarking:**
    - The legacy architecture relies on concepts like `Inbox`, `Conversation`, `Message`, `Contact`, and specific channel models (`Channel::Whatsapp`, `Channel::WebWidget`).
    - It uses WebSockets heavily for real-time widget updates.
    - OHC needs to replicate this architecture but natively in Rust, using `tonic` for gRPC/REST APIs, `sqlx` or `diesel` for PostgreSQL with RLS, and a native WebSocket implementation.
  - **Competitor Systems Analysis:** Modern helpdesk solutions prioritize speed, agent handoff, and unified inboxes. Native implementations benefit from lower latency and tight integration with core business objects (e.g., matching a WhatsApp message directly to an order).

  ## Design Doc

  ### Architecture Overview
  - **Core Entities (Rust Structs & DB Tables):**
    - `Inbox`: Represents a collection channel (e.g., Maya's Instagram DM inbox).
    - `Conversation`: A thread between a contact and the business.
    - `Message`: Individual messages within a conversation.
    - `Contact`: The customer interacting with the business.
    - `ChannelAdapter`: Interfaces for specific platforms (WhatsApp, Web Widget, Instagram).
  - **API & Transport:**
    - REST/gRPC endpoints for mobile and web clients to fetch inboxes and messages.
    - WebSocket server (e.g., `axum` or `warp` based) for real-time web widget and mobile app updates.
  - **AI Agent Integration:**
    - Background job queue (PostgreSQL `SKIP LOCKED` pattern) triggering the AI Customer & Relationship Assistant to draft replies for new messages.
  - **Multi-Tenancy:**
    - All tables MUST include `tenant_id`. Database connections enforce RLS based on the active session's tenant ID.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string message_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }

      INBOX ||--o{ CONVERSATION : "contains"
      CONVERSATION ||--o{ MESSAGE : "has"
      CONTACT ||--o{ CONVERSATION : "participates in"
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen:**
    - List of active conversations, sorted by recency and priority.
    - Visual indicators (tokens) for unread messages, channel source (WhatsApp icon, Web icon), and AI-drafted reply readiness.
    - Floating Action Button (FAB) or quick actions to compose a new message.
  - **Conversation View:**
    - Standard chat bubble layout.
    - "AI Assistant Draft" box visible just above the input field, allowing the owner to "Approve & Send" or edit.
    - Context panel (accessible via swipe or top button) showing customer details and recent orders.

  ### Key Design Decisions
  - **Native Rust over External Service:** Guarantees data locality, simplifies the architecture stack, and allows deep integration with OHC's unique AI triage system.
  - **Unified Data Model:** Abstracting specific channel details into generic `Message` and `Conversation` models to ensure the UI remains consistent regardless of the source.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the core backend infrastructure for the Native Rust Omnichannel Chat System.
  1. Define the PostgreSQL schema (migrations) for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure `tenant_id` is on all tables and RLS policies are created.
  2. Implement the Rust data models and database repositories (using the standard ORM/query builder in the repo) for these entities.
  3. Create the core API endpoints (REST/gRPC depending on the server framework) to:
     - List inboxes.
     - Fetch conversations for an inbox.
     - Fetch messages for a conversation.
     - Send a message (which should also enqueue an event for external channels).
  4. Implement a foundational WebSocket handler for real-time message delivery to the web/mobile clients.
  5. Add unit and integration tests to verify multi-tenant isolation (user A cannot see user B's messages).

  *Acceptance Criteria:*
  - All new tables enforce tenant isolation.
  - Rust models and APIs are implemented and fully unit tested (100% coverage).
  - The WebSocket endpoint can establish a connection and broadcast a test message to an authenticated client.
  - Existing tests on `main` continue to pass without regression.

  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
