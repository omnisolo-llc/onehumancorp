issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The external dependency on the third-party open-source chat solution for omnichannel customer support has been explicitly retired to align with the core product vision. OHC must own the end-to-end multi-tenant customer communication channel natively within `onehumancorp/mono`. We need a high-performance, strictly isolated Rust implementation of the core omnichannel model (Inboxes, Contacts, Conversations, Messages, and Channels) that powers the Work Triage and Customer & Relationship Assistant capabilities.

  **Research Report**
  An audit of the retired legacy third-party source code reveals its core data modeling around several key domain entities:
  - `Account` (matches OHC's `tenant`)
  - `Inbox` (the aggregation point for channels like Email, SMS, Web Widget)
  - `Contact` (the customer interacting via channels)
  - `Conversation` (the threaded interaction between an Inbox and a Contact)
  - `Message` (the individual events within a conversation, containing text, attachments, or system events).

  To replicate and improve this inside OHC:
  1. We must map these entities to OHC's PostgreSQL database using strict Row-Level Security (RLS) on `tenant_id`.
  2. Implement an event-driven messaging layer in Rust for real-time WebSocket capabilities, likely using existing messaging infrastructure like `msgbus.rs` or `queue.rs`.
  3. Support extensibility for different `Channel` adapters (e.g., Web Widget, Email, Instagram DMs).

  **Design Doc**
  *Architecture Overview:*
  The system will reside within a new or existing module (e.g., `src/server/chat` or embedded in `hub.rs`/`interop`). It will provide a gRPC/REST API layer consumed by the frontend and AI agents.
  - **Data Models:**
    - `Inbox`: Configuration for receiving messages (Channel Type, Settings).
    - `Contact`: Identity and custom attributes of a customer.
    - `Conversation`: Links Inbox, Contact, and assigned Agent/Bot. Tracks status (`open`, `resolved`).
    - `Message`: Represents actual payloads, including private notes or public replies.

  *Architecture Diagram (Mermaid.js):*
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : owns
    TENANT ||--o{ CONTACT : owns
    TENANT ||--o{ CONVERSATION : owns
    TENANT ||--o{ MESSAGE : owns
    INBOX ||--o{ CONVERSATION : has
    CONTACT ||--o{ CONVERSATION : participates_in
    CONVERSATION ||--o{ MESSAGE : contains

    TENANT {
        uuid tenant_id PK
        string name
    }

    INBOX {
        uuid id PK
        uuid tenant_id FK
        string channel_type
        jsonb settings
    }

    CONTACT {
        uuid id PK
        uuid tenant_id FK
        string identifier
        jsonb custom_attributes
    }

    CONVERSATION {
        uuid id PK
        uuid tenant_id FK
        uuid inbox_id FK
        uuid contact_id FK
        string status
    }

    MESSAGE {
        uuid id PK
        uuid tenant_id FK
        uuid conversation_id FK
        string content
        string message_type
    }
  ```

  - **Multi-Tenancy:**
    - Every table will include a `tenant_id` UUID column.
    - RLS policies must strictly enforce `tenant_id = current_setting('app.current_tenant_id')::uuid`.
  - **Real-time Delivery:**
    - New messages will be published to a Redis Pub/Sub channel (via `msgbus.rs`) scoped by `tenant_id`.
    - WebSocket connections from the frontend will subscribe to their tenant's channel for instant updates.
  - **Mobile UX Flow (375px first):**
    - The Triage feed will surface open conversations.
    - Tapping a conversation opens a standard chat view: a header with contact details, scrollable message list, and an input area optimized for mobile keyboards.
    - Quick actions for AI Assistant to draft replies will be sticky above the input.
  - **AI Agent Integration:**
    - When a `Message` is created by a customer, an event is emitted.
    - The Customer & Relationship Assistant agent (via PostgreSQL `SKIP LOCKED` job queue) picks up the event, analyzes context, and drafts a private response or executes automated rules (e.g., auto-reply for out of office).

  **Implementation Prompt**
  Implement the backend core data models and service layer for the native Rust omnichannel chat system.
  1. Create the database migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring all tables have `tenant_id` and strict RLS policies enabled.
  2. Implement the corresponding Rust struct models and CRUD repositories using `sqlx`.
  3. Build a service layer (`ChatService`) with methods to create inboxes, start conversations, and send messages.
  4. Ensure sending a message publishes an event to the message bus for real-time delivery and agent processing.
  5. The acceptance criteria is that a user can create an inbox, a contact can start a conversation, and messages can be sent/retrieved via API endpoints, completely independently of the external third-party service previously used.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
