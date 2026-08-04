issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) needs a robust, high-performance, and multi-tenant omnichannel chat system to serve our core owner/operator personas (e.g., Maya the baker, Carlos the handyman). Our platform currently lacks the native infrastructure to handle unified customer messaging (Instagram DMs, WhatsApp, SMS, Web Chat) securely and reliably. We are retiring the external Chatwoot dependency to ensure tight multi-tenant isolation, Zero-Trust security, and native Rust performance. We must build a matching native Rust messaging engine that provides 100% feature parity with Chatwoot's omnichannel capabilities but integrated directly into OHC.

  ## Research Report
  - **Competitive Analysis:** Audited the `chatwoot/chatwoot` source repository. Chatwoot relies on a complex Ruby on Rails backend, heavy background workers (Sidekiq), and PostgreSQL/Redis for state and pub/sub.
  - **Key Capabilities Needed:**
    - **Inboxes/Channels:** Unified interface for different channel types (Web, Email, API, WhatsApp, Instagram, etc.).
    - **Conversations:** Core entity linking Contacts, Messages, Inboxes, and Agents/Assignees.
    - **Messages:** Multi-type messages (text, attachments) with rich metadata.
    - **Contacts:** Customer profiles linked across multiple channels.
    - **Real-time Engine:** WebSocket based real-time event streaming.
  - **OHC Architecture Fit:** A native Rust implementation using our existing gRPC/REST API layers and PostgreSQL with Row-Level Security (RLS) for tenant isolation (`tenant_id`).

  ## Design Doc
  - **Architecture Overview:**
    - A new set of native Rust domain models (`Conversation`, `Message`, `Inbox`, `Contact`, `ChannelAdapter`) located in `src/server/ohc/domain/chat/`.
    - PostgreSQL tables with strict `tenant_id` based RLS for all chat entities.
    - WebSocket service (`ChatGateway`) for real-time bidirectional communication.
    - Background jobs (using PostgreSQL `SKIP LOCKED`) for webhooks, external channel syncing, and AI agent coordination.

  - **Architecture Diagram:**
    ```mermaid
    graph TD;
      Client[Mobile/Web App] --> API[OHC API Gateway];
      API --> REST[REST API];
      API --> WS[WebSocket Chat Gateway];

      REST --> ChatService[Chat Service];
      WS --> ChatService;

      ChatService --> DB[(PostgreSQL with RLS)];
      ChatService --> Queue[(PostgreSQL Job Queue)];

      Queue --> BackgroundWorker[Background Job Worker];
      BackgroundWorker --> ThirdParty[3rd Party Channels (WhatsApp/IG)];
      BackgroundWorker --> AIAgent[AI Triage / Assistant Agents];
    ```

  - **Data Model & Invariants:**
    - `conversations`: Links `tenant_id`, `inbox_id`, `contact_id`, `status` (open/resolved/snoozed), `assignee_id`.
    - `messages`: Links `tenant_id`, `conversation_id`, `sender_type`, `content`, `content_type`, `status`.
    - `inboxes`: Links `tenant_id`, `channel_type`, `name`, `config`.
    - `contacts`: Links `tenant_id`, `name`, `email`, `phone_number`, `identifier`.

    ```mermaid
    erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Tenant ||--o{ Conversation : "owns"
      Tenant ||--o{ Message : "owns"

      Inbox ||--o{ Conversation : "contains"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "has"
    ```

  - **Mobile UX Flow (375px first):**
    - A unified "Inbox" tab replacing the standard feed.
    - Swipe-to-resolve/snooze interactions.
    - Real-time typing indicators and message delivery statuses (Sent, Delivered, Read).
    - Native keyboard handling and attachment uploads (images, PDFs) via MinIO/GCS.

  - **AI Agent Integration:**
    - Work Triage Agent: Listens to new `Conversation` events, categorizes intent, and auto-assigns or drafts replies.
    - Customer Assistant Agent: Suggests replies based on tenant context (e.g., business hours, inventory).

  ## Implementation Prompt
  Implement the core database schema, Rust domain models, and basic gRPC/REST APIs for the new native Rust Omnichannel Chat System.

  - Define PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` and Row-Level Security (RLS) are applied correctly.
  - Create the corresponding Rust structs and Diesel/SQLx ORM mappings in `src/server/ohc/domain/chat/`.
  - Implement CRUD APIs (Create Inbox, Create Contact, Start Conversation, Send Message).
  - Write unit tests ensuring 100% coverage and proper tenant isolation (tenant A cannot read tenant B's messages).
  - Ensure `bazel test //...` passes completely.
  - Focus purely on the backend logic and data models for this initial phase.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
