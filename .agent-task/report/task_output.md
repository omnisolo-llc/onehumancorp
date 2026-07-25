issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Replace Chatwoot with a Native Rust Omnichannel Chat System

  ## Problem Statement
  Currently, OneHumanCorp (OHC) uses Chatwoot as its core component for managing omnichannel communications. However, Chatwoot introduces an external Ruby-on-Rails/PostgreSQL/Redis dependency which violates the project goal of consolidating around a single monolithic architecture backed by Go/Rust and Bazel. We need a fully native implementation of Chatwoot's functionality in Rust that natively integrates into our existing multi-tenant PostgreSQL/Kubernetes stack without adding external components. The OHC Assistant needs deep, synchronous access to chat channels (web widget, Whatsapp, SMS) to enable the "Work Triage", "Customer & Relationship Assistant" and "Operations Assistant" workflows seamlessly.

  ## Research Report
  I performed an audit of the `chatwoot/chatwoot` GitHub repository to understand the core functionality. Here are the core data models and concepts that need replicating:
  *   **Inboxes & Channels:** Represents the intake mechanism (Email, SMS, API, WebWidget, Whatsapp). Channels hold provider-specific configuration. Inboxes bind channels to the account/tenant.
  *   **Conversations:** The thread of messages between a customer and the business.
  *   **Messages:** Individual items within a conversation (incoming, outgoing, private notes). Handled internally by a background queue (`SendReplyJob`).
  *   **Contacts & ContactInboxes:** Represents the external user and links them to the specific inbox.
  *   **Webhooks & WebSocket Realtime messaging:** Used for real-time widget updates.

  Competitor Analysis (Zendesk, Intercom, Chatwoot): All use a pub/sub mechanism to push events to the web client. OHC's architecture uses Postgres for persistence and Redis for distributed locks/pubsub. We must implement a fast Rust-based WebSocket server that communicates with our gRPC backend.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client(Web Widget / Mobile App) -->|WebSocket / HTTPS| EdgeRouter[Edge Router / NGINX]
      EdgeRouter --> RustChat[Rust Chat Service]
      RustChat -->|gRPC| CoreGo[Go Core Services]
      RustChat -->|PubSub| Redis
      RustChat -->|Read/Write| Postgres[Multi-tenant Postgres]
      CoreGo --> Postgres
  ```

  ### Core Entities (Rust implementation of Chatwoot models)
  *   `Tenant`: (Existing in OHC)
  *   `Channel`: Extensible trait/struct for different channel types (WebWidget, WhatsApp, SMS).
  *   `Inbox`: Ties a `Channel` to a `Tenant`.
  *   `Contact`: Information about the customer.
  *   `Conversation`: Belongs to `Inbox` and `Contact`.
  *   `Message`: Belongs to `Conversation`. Has a `type` (incoming, outgoing, bot).

  ### Mobile UX Flow (375px)
  *   **Web Widget:** A floating action button (FAB) in the bottom right. Tapping opens a translucent-glass styled chat pane taking up 100% of the mobile viewport. Follows UniFi clean aesthetic.
  *   **Owner Inbox View:** A list of conversations. Tapping one opens a detailed view with messages and context (customer details, recent orders). All inputs use native mobile keyboards.

  ### AI Agent Integration Points
  *   When a new `Message` is created (incoming), a PostgreSQL `SKIP LOCKED` job is queued.
  *   The "Work Triage" agent picks up the job, analyzes the message, and can either draft a reply (saving a `Message` with `type=draft`) or automatically reply if confidence is high.
  *   The "Operations Assistant" can inject private `Message` entries into the conversation thread to notify the owner of actions taken (e.g., "Created a booking request based on this message").

  ## Implementation Prompt
  Implement a native Rust microservice (or crate within the monorepo) that handles omnichannel chat, fully replicating Chatwoot's core messaging, conversation, and inbox models.

  The implementation must:
  1. Use Rust with a high-performance web framework (e.g., Axum) for WebSocket and REST endpoints.
  2. Implement strict row-level multi-tenancy using PostgreSQL `tenant_id`.
  3. Support a "Web Widget" channel out of the box, with a Vue/React/WebComponent frontend that matches the OHC premium translucent-glass design system.
  4. Provide gRPC interfaces so the existing Go backend and AI agents can seamlessly read conversations and inject messages.
  5. Include full unit test coverage and at least 5 Playwright E2E tests validating the Critical User Journey: "A visitor opens the web widget, sends a message, and the owner sees it in their unified feed."

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []