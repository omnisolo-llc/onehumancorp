issue_title: "Native Rust Omnichannel Chatwoot Alternative"
issue_description: |
  **Problem Statement**
  The external service Chatwoot is 100% RETIRED from OHC. Currently, the product lacks an omnichannel chat and support inbox. We need to implement a high-performance native Rust Chat system replicating Chatwoot's core capabilities directly within OneHumanCorp to ensure strict multi-tenant data isolation, lower latency, and zero dependency on a third-party support tool for Maya, Carlos, Priya, Leo, Fatima, and Nora.

  **Research Report**
  As part of evaluating the core architecture, we reviewed Chatwoot's source code and system design (https://github.com/chatwoot/chatwoot). Chatwoot utilizes concepts such as multi-channel inboxes, macro automations, SLA policies, WebSocket real-time messaging, and distinct agent roles. We need these capabilities replicated in Rust to provide a native OHC omnichannel support experience.

  **Design Doc**
  - **Architecture Diagram (Mermaid)**:
    ```mermaid
    graph TD
      A[Mobile / Web Client] -->|WebSocket / REST| B[OHC Rust API Gateway]
      B --> C[Chat Service Module]
      C --> D[PostgreSQL: ohc_conversations, ohc_messages]
      C --> E[Redis: Pub/Sub & Presence]
      C --> F[Agent AI Module]
    ```
  - **Mobile UX Flow**: A 375px optimized layout featuring an Inbox list, sliding over to a Conversation view. Supports translucent glass overlays for the message input bar.
  - **AI Agent Integration**: Agent AI module automatically categorizes incoming queries and drafts suggested responses.
  - **Decisions**: A clean separation of `Conversations`, `Messages`, and `Channels` entities within the PostgreSQL database using standard row-level multi-tenancy.

  **Implementation Prompt**
  Implement the backend data models and gRPC/REST APIs in Rust for `Conversations` and `Messages`. Ensure it supports multi-tenant isolation via `tenant_id`. Implement a basic WebSocket hub for real-time delivery. Update the Tauri UI to render the Inbox and Message views using the OHC Premium Token library (macOS translucent style). Provide E2E tests validating a customer sending a message and an owner receiving it on a 375px screen layout.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
