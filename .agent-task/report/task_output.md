issue_title: "[Native Rust Chat] Architect High-Performance Omnichannel Unified Inbox (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC is actively retiring its dependency on the external Chatwoot Ruby on Rails system to achieve tighter latency, strictly enforced row-level multi-tenant isolation, Zero-Trust compliance (SPIFFE/SPIRE), and native integration with the OHC Agent Triage system. We require a high-performance, omnichannel Unified Inbox engine implemented natively in Rust to serve as the unified communication layer for all our core business personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun). This system must ingest messages from WhatsApp, Meta Webhooks, and web widgets, presenting a cohesive inbox to business owners and automated agents.

  ## Research Report
  - **Chatwoot Source Code Audit:** Analyzed the data models of Chatwoot (`inboxes`, `conversations`, `messages`, `contacts`). Chatwoot utilizes polymorphic associations and complex JSONB columns. We will simplify and strongly type these models for our Rust implementation.
  - **Competitor Systems:** We studied Zendesk, Intercom, and Shopify Inbox. A key differentiator is maintaining ultra-low latency for web widgets via WebSockets, ensuring seamless AI-agent handoffs without dropped context, and strong multi-tenant database isolation.
  - **Gaps Identified:** The current `/src/server/integrations/chat` module is incomplete. We lack the core data structures (Inbox, Conversation, Message, Contact), the database repository layer (with RLS), the service layer for handling channel integrations (WhatsApp, Web Widget), and WebSocket real-time capabilities.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type "whatsapp | web_widget | instagram"
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          string identifier
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open | resolved | snoozed"
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type "incoming | outgoing"
          string status "sent | delivered | read"
      }
  ```

  ### Core Components
  1. **Data Model & Invariants:** Define strongly typed Rust structs for `Inbox`, `Conversation`, `Message`, and `Contact`. Enforce multi-tenancy at the ORM/SQL level by mandating `tenant_id` on every query.
  2. **Service Layer (Channel Adapters):** Implement traits for different channels (e.g., `ChannelAdapter`) to handle the specifics of WhatsApp vs. Web Widget.
  3. **Real-time WebSockets:** Implement an `axum` or `actix-web` WebSocket handler that broadcasts message updates to connected clients based on `tenant_id` and `conversation_id`.
  4. **AI Agent Triage Integration:** Expose internal gRPC/Rust traits allowing the OHC Agent Triage system to subscribe to new conversations, draft replies, and hand off to human operators.

  ### Mobile UX Flow (375px)
  - The UI will feature an "Unified Inbox" icon.
  - Tapping opens a list of active conversations, grouped by urgency and AI-triage status.
  - Conversation view shows standard chat bubbles. Unassigned conversations will clearly show "AI Agent Handling" or "Needs Owner Attention".
  - Responses can be typed directly, or the user can approve/edit an AI-drafted reply.

  ## Implementation Prompt
  Implement the core native Rust Chat engine in `src/server/integrations/chat`.
  1. Define the SQL schema (PostgreSQL) and Rust structs for `inboxes`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` is present on all models for RLS.
  2. Build the repository layer to perform CRUD operations on these entities.
  3. Implement the core `ChatService` that handles creating conversations and sending/receiving messages.
  4. Create comprehensive Unit and Integration tests verifying tenant isolation (Tenant A cannot see Tenant B's messages).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
