issue_title: "[Native Rust Chatwoot Replacement] Core Data Model & Architectures for OHC Omnichannel Inbox"
issue_description: |
  # Native Rust Chatwoot Replacement Architecture & Design

  ## Problem Statement
  OneHumanCorp (OHC) is replacing the third-party Chatwoot service with a natively built, high-performance, multi-tenant Rust omnichannel chat system inside `onehumancorp/mono`. We need to design the core data models, channels, and inbox architecture to match Chatwoot's functionality but optimized for OHC's target personas (Maya, Carlos, Priya, Leo, Fatima) and our multi-tenant scaling needs.

  ## Research Report & Gap Analysis
  Based on a source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`), the core entities required for an omnichannel inbox are:
  - **Account/Tenant**: The highest level of isolation.
  - **Inbox**: A container for conversations coming from specific channels (e.g., "Support", "Sales", "Instagram DMs").
  - **Channel Adapters**: Interfaces to external networks (WhatsApp, Instagram, Web Widget, Email).
  - **Conversation**: A thread of messages between a Contact and Agents/Bots.
  - **Message**: Individual payloads (text, attachments, templates).
  - **Contact**: The external customer.
  - **Agent/Bot**: The internal operator or AI assistant handling the conversation.

  **OHC Specific Requirements:**
  - **Strict Row-Level Security (RLS)**: Every entity MUST have a `tenant_id` for PostgreSQL RLS.
  - **Mobile-First UX (375px)**: The UI for managing this inbox must be fully functional on a 375px mobile screen. Complex configuration (like setting up a new channel) must be agent-assisted (e.g., "Agent Feed" action cards) rather than complex web forms.
  - **AI Agent Integration**: The inbox must seamlessly integrate with OHC's AI agents. Agents should be able to draft responses, auto-reply based on context, and escalate to humans.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ AGENT : owns

      INBOX ||--o{ CHANNEL : configured_with
      INBOX ||--o{ CONVERSATION : contains

      CONTACT ||--o{ CONVERSATION : participates_in

      CONVERSATION ||--o{ MESSAGE : has

      MESSAGE ||--o| ATTACHMENT : includes
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Agent Feed**: The user (e.g., Maya) opens the OHC app. Instead of a complex inbox view, she sees an "Action Card" in her feed: "New Instagram DM from Contact X. Drafted response: 'Yes, we have vegan cakes!'. [Approve & Send]".
  2. **Inbox View**: If she navigates to the Inbox, it's a simple, touch-friendly list of active conversations.
  3. **Conversation View**: Large touch targets for predefined quick replies, a native keyboard integration, and clear AI-draft suggestions inline.

  ### Key Design Decisions
  - **Rust Native Services**: Implement the messaging core (WebSockets, Webhooks, Data persistence) using Rust (Axum + Tokio) for high concurrency and low latency.
  - **PostgreSQL Persistence**: Use PostgreSQL with strict RLS for tenant isolation.
  - **Event-Driven AI**: All incoming messages publish events to the AI Job Queue (Redis/Pg `SKIP LOCKED`). AI agents consume these events, classify intent, and draft responses *before* the human user even opens the app.

  ## Implementation Prompt
  **Objective**: Implement the foundational PostgreSQL schema and Rust entity structs for the native omnichannel inbox.

  **Tasks for Implementer**:
  1. Create database migrations for `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Ensure every table includes a `tenant_id` and has RLS enabled.
  2. Define the corresponding Rust structs and SeaORM/SQLx models in the backend service.
  3. Implement basic CRUD APIs (gRPC or REST) for Inboxes and Conversations, ensuring tenant isolation is enforced at the query level.
  4. Build a simple E2E Playwright test simulating a user (e.g., Maya) logging in, creating an Inbox, and viewing an empty conversation list.

  **Acceptance Criteria**:
  - Migrations run successfully and enforce RLS.
  - Rust models compile and accurately represent the schema.
  - E2E test passes on the real docker-compose stack.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
