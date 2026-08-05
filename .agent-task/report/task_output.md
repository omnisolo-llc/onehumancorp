issue_title: "Native Rust Omnichannel Chat System Replication"
issue_description: |
  # Native Rust Omnichannel Chat System Replication

  ## Problem Statement
  OneHumanCorp currently has a major gap in its omnichannel chat architecture. Chatwoot has been fully retired as an external dependency per the OHC Engineering Standards, but we do not yet have a native Rust microservice that can replicate Chatwoot's omnichannel chat logic (multitenant inboxes, conversation routing, macros, SLA policies, and agent interfaces). Maya, Carlos, and Priya need an integrated "Customer & Relationship Assistant" within the OHC ecosystem to draft replies for chat, Instagram, and web inquiries without routing data to third-party services.

  ## Research Report
  Based on an audit of the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`), the core entities to replicate are:
  - `accounts`: Tenant boundaries.
  - `inboxes`: Channel entry points (web, email, API, FB/IG).
  - `conversations`: Grouped message threads tied to contacts and assignees.
  - `messages`: Immutable message payloads with rich media support.
  - `contacts`: Unified customer identities across channels.
  - `users` / `account_users`: Agent and admin identities with roles.

  The Chatwoot architecture uses PostgreSQL for persistence and Redis/Sidekiq for background jobs (webhooks, email processing) and ActionCable for WebSockets. To replicate this in OHC, we will leverage:
  - Rust + PostgreSQL with strict row-level security (RLS) on `tenant_id`.
  - Tokio + WebSockets for real-time `ActionCable` style updates.
  - OHC's native job queue (`SKIP LOCKED` pattern on Postgres).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : has
      ACCOUNT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      ACCOUNT_USER ||--o{ CONVERSATION : assigned_to
  ```

  ### Mobile UX Flow
  - The OHC Assistant-First Shell will have an "Inbox" tab.
  - Tapping Inbox shows unified conversations sorted by SLA and urgency.
  - Tapping a conversation opens a standard chat view, allowing replies from the AI Assistant or human owner.
  - 375px viewport optimized: Sticky bottom text input, easy thumb-reach for predefined macros or AI drafts.

  ### AI Agent Integration
  - **Work Triage Agent**: Monitors new `CONVERSATION` records and assigns priorities/tags.
  - **Customer Assistant**: Listens to new `MESSAGE` events, queries the Knowledge Assistant, and drafts a pending reply.

  ## Implementation Prompt
  Implementer Agent:
  Your task is to build the initial Rust foundation for the OHC Omnichannel Chat System, replacing Chatwoot.
  1. Define the PostgreSQL migrations for `ohc_inboxes`, `ohc_conversations`, `ohc_messages`, and `ohc_contacts`. All must include `tenant_id` for RLS.
  2. Implement the Rust data models and CRUD repositories for these entities.
  3. Create a REST/gRPC API service layer for creating messages and fetching conversation histories.
  4. Ensure 100% test coverage and verify using the running Docker Compose stack.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
