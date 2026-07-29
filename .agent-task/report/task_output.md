issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  The OHC backend currently delegates chat functionalities (omnichannel inbox, WhatsApp, web widgets, etc.) to a legacy external chat dependency. The architecture mandate requires that this dependency be completely retired and replaced with a native Rust implementation inside `onehumancorp/mono`. This change is critical for achieving zero-trust tenant isolation, real-time message routing (via WebSockets), and high-performance message processing without external network boundaries or third-party SLA constraints.

  ## Research Report
  - **Codebase Audit**: Scanned `src/server/integrations/chat/README.md` which confirms the mandate to implement a native Rust omnichannel chat system replacing the external chat service.
  - **Source Benchmark**: Benchmarked against the legacy open-source chat service data models. Key models include `Account`, `User`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel`, and integrations for API, Web Widget, and WhatsApp.
  - **Competitive Advantage**: A native Rust implementation eliminates cross-service latency, removes a major infrastructure dependency, allows deep native integration with OHC's Multi-Tenant Row Level Security (RLS) PostgreSQL database, and simplifies the deployment topology for independent owner/operators.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      Client[Web/Mobile Client] -->|WebSocket/REST| API[OHC Rust API Gateway];
      API --> Auth[SPIFFE/SPIRE Zero-Trust Auth];
      Auth --> ChatSvc[Native Rust Chat Service];

      ChatSvc --> InboxDB[(Tenant-Isolated DB / RLS)];
      ChatSvc --> MsgQueue[Postgres/Redis Job Queue];

      Meta[WhatsApp Webhooks] --> WebhookHandler[Webhook Ingress];
      WebhookHandler --> ChatSvc;

      Widget[Web Widget Chat] -->|WebSocket| ChatSvc;

      MsgQueue --> AI[OHC Agent Triage];
      AI --> ChatSvc;
    ```
  - **Mobile UX Flow (375px)**:
    - **Unified Inbox View**: Clean, Apple-style list of recent conversations. Unread indicators prominently displayed.
    - **Conversation View**: Translucent glass sticky header showing customer name. Messages rendered natively. Keyboard overlay avoids breaking the scroll view.
    - **Actions**: Native touch targets (44x44px minimum) for assigning conversations, resolving tickets, and invoking AI reply suggestions.
  - **AI Agent Integration**:
    - The Native Chat Service pushes new unread messages to the OHC AI Job Queue.
    - The AI Triage Agent analyzes the message and optionally drafts a response or flags urgency.
  - **Key Design Decisions**:
    - **Database Schema**: Implement `conversations`, `messages`, `inboxes`, `contacts` tables with strictly enforced `tenant_id` RLS.
    - **Real-Time Delivery**: Utilize native Rust async WebSockets (e.g., via Axum/Tokio) for instant message delivery to the Flutter frontend.
    - **Extensibility**: Create a modular `ChannelAdapter` trait in Rust to support WhatsApp, Web Widget, Email, and SMS cleanly.

  ## Implementation Prompt
  **Goal:** Implement the foundational Native Rust Omnichannel Chat models and API endpoints.

  **Instructions for Implementer:**
  1. Define the core database schema migrations for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure every table includes a `tenant_id` column and enables PostgreSQL Row Level Security (RLS).
  2. Implement the Rust data models (structs) corresponding to these tables in `src/server/integrations/chat/models.rs` (or appropriate module).
  3. Create REST API endpoints in Rust (e.g., using Axum) for:
     - Creating/listing Inboxes.
     - Fetching Conversations for an Inbox.
     - Sending and receiving Messages in a Conversation.
  4. Ensure all endpoints are protected by the existing Zero-Trust/SPIFFE auth framework and validate `tenant_id` context.
  5. **Verification**: Write comprehensive unit tests (100% coverage) for the new models and endpoints. Write at least one end-to-end Playwright test simulating a user logging in and viewing their inbox messages.

  **Acceptance Criteria:**
  - Database migrations for chat models are present and correct.
  - Rust API endpoints for basic CRUD operations on Inboxes, Conversations, and Messages are fully functional.
  - Row Level Security is demonstrably enforced in tests.
  - All tests (`bazel test //...`) pass successfully.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
