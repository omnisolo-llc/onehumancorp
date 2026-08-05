issue_title: "[Native Chat] Implement Rust-Native Omnichannel Models & Schema"
issue_description: |
  **Problem Statement**
  OHC requires a native Rust omnichannel chat system to replace external dependencies (like legacy-chat-system) in order to provide an integrated, high-performance, and deeply observable multi-tenant experience. We must design and document the core data architecture based on a detailed source-code benchmarking against legacy-chat-system's core abstractions, but mapped to OHC's Rust/PostgreSQL environment using row-level security.

  **Research Report**
  Benchmarking `https://github.com/legacy-chat-system/legacy-chat-system` source code yields these core model patterns:
  1. `Account` -> Maps to OHC `Tenant`.
  2. `Contact` -> End user / customer interacting with OHC. Needs strong `tenant_id` isolation.
  3. `Inbox` -> The configured entry point (e.g., WhatsApp, WebWidget, Email). Links a Channel to a Tenant.
  4. `ContactInbox` -> The linking table mapping a Contact to an Inbox (including `source_id` from the channel).
  5. `Conversation` -> Groups messages for a specific `ContactInbox` and `Inbox`. Status tracking (open, resolved, snoozed), agent assignment.
  6. `Message` -> The individual chat payload (text, media, system activity). `content_type`, `message_type` (incoming, outgoing, template), `private` (internal note vs public).

  **Design Doc**
  - Architecture: Rust microservices backed by PostgreSQL with strict RLS on `tenant_id`.
  - Data Entities (Mermaid):
    ```mermaid
    erDiagram
        Tenant ||--o{ Inbox : configures
        Tenant ||--o{ Contact : owns
        Inbox ||--o{ Conversation : contains
        Contact ||--o{ ContactInbox : has
        Inbox ||--o{ ContactInbox : has
        ContactInbox ||--o{ Conversation : has
        Conversation ||--o{ Message : contains
    ```
  - Mobile UX: 375px native Chat Inbox feed -> Conversation view -> Message bubbles. Needs robust real-time updates (WebSocket) and optimistic UI.
  - AI Integration: AI Agents monitor `Conversation` state, read `Messages`, and can insert drafted replies (`Message.message_type = draft` or `bot`).

  **Implementation Prompt**
  Implement the PostgreSQL schemas and Rust `diesel` / `sqlx` model structs for the core omnichannel entities (`Inbox`, `Contact`, `ContactInbox`, `Conversation`, `Message`) as analyzed. Ensure every table has `tenant_id` and strict Row Level Security (RLS) is applied. Write unit tests for CRUD operations on these models under different tenants. Focus on the core schema and data models first; the WebSocket and adapter layers will follow in subsequent tasks.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
