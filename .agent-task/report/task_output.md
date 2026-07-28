issue_title: "Native Rust Omnichannel Inbox & Chat Engine"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Implement Native Rust Omnichannel Inbox & Chat Engine

  **Problem Statement**:
  Currently, OneHumanCorp (OHC) is retiring its dependency on the external third-party messaging service to reduce external coupling and improve native multi-tenant performance. Our non-technical owner/operator personas, such as Maya (baker using Instagram DMs) and Carlos (handyman using SMS/WhatsApp), require a unified, lightning-fast inbox to view and respond to all customer inquiries. Without a native omnichannel engine, OHC cannot seamlessly integrate AI-assisted drafts and background operations with real-time customer messaging. We need to build a native Rust replacement that provides 100% feature parity with the core messaging architecture.

  **Research Report**:
  An extensive audit of the chat engine architecture was conducted.
  - The core data model revolves around `Accounts` (Tenants), `Inboxes`, `Channels` (Web Widget, API, Email, Facebook Page, Twitter, Twilio SMS, WhatsApp, Line, Telegram), `Conversations`, `Messages`, and `Contacts`.
  - Real-time messaging is handled via WebSockets broadcasting event payloads.
  - OHC will replicate this model natively in Rust, leveraging our existing PostgreSQL infrastructure with strict multi-tenant Row-Level Security (RLS) and Redis for pub/sub WebSocket coordination.

  **Design Doc**:

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      MESSAGE }o--|| CONTACT : sent_by
  ```

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to the Redis pub/sub feed for `message.created` events.

  **Implementation Prompt**:
  To the Implementer Agent:
  Your task is to implement the core backend Rust services and database schema for the Native Omnichannel Inbox.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
