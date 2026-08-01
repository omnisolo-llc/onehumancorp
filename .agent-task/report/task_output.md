issue_title: "[Platform] Native Rust Omnichannel Chat System Architecture (Chatwoot Retirement)"
issue_description: |
  ## Problem Statement
  OHC requires a native omnichannel chat system integrated seamlessly. The product mandate explicitly states: "Chatwoot as an external third-party service, dependency, or integration is 100% RETIRED. OHC implements its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust."

  ## Research Report
  Chatwoot's architecture relies heavily on separate `Conversation`, `Message`, `Contact`, and `Inbox` models.
  To replicate this natively in Rust, we need similar data models. OHC's target backend is Rust, relying heavily on PostgreSQL with Row-Level Security for multi-tenancy.

  ## Design Doc
  - **Data Model & Invariants**:
    - `Inbox`: Channels (Email, SMS, Web Widget) linked to an Account/Tenant.
    - `Contact`: Represents end-users (customers).
    - `Conversation`: Thread linking a `Contact` to an `Inbox`.
    - `Message`: Individual messages within a `Conversation`.
    - Every table must have `tenant_id` for PostgreSQL RLS.
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
    ```
  - **Mobile UX Flow**: A unified inbox view at 375px viewport. Contacts listed with latest messages.
  - **AI Agent Integration**: Agents can observe the `Conversation` feed and draft `Message` responses automatically for the business owner to approve.

  ## Implementation Prompt
  - Create the PostgreSQL migration scripts for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring multi-tenant `tenant_id` is present with RLS policies.
  - Implement the Rust core domain models in `src/server/ohc/domain/chat/`.
  - Create standard CRUD repository operations in Rust using `sqlx` (or the equivalent ORM used in OHC).
  - Implement gRPC/REST handlers for creating conversations and messages.
  - Write unit tests covering domain models, data access, and API handlers. 100% test coverage required.
  - Write E2E Playwright tests simulating a user sending a message through a web widget and an agent seeing it.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, core-platform]
assignees: []
