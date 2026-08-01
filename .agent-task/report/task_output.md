issue_title: "[Platform] Native Rust Omnichannel Chat Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems (like Chatwoot) for omnichannel customer communication. This creates data silos, increases latency, complicates multi-tenant data isolation, and breaks the core promise of a unified owner-assistant. SMB owners like Maya and Carlos need all their customer interactions (Instagram DMs, WhatsApp, SMS, Web Chat) seamlessly integrated into their primary work feed without switching context.

  ## Research Report
  - **Chatwoot Audit**: Analyzed `chatwoot/chatwoot` source. Their architecture uses an `Inbox` model connected to `Channel` models (e.g., `Channel::WebWidget`, `Channel::TwilioSms`). `Conversation` acts as the unifying thread for `Message` objects.
  - **Competitors**: Zendesk, Intercom, and Shopify Inbox all utilize a unified thread model but lack the native AI-agent intervention OHC promises.
  - **The Gap**: OHC lacks a native, high-performance, Rust-based omnichannel chat engine that strictly enforces our multi-tenant row-level security and seamlessly hands off to our AI Agent departments.

  ## Design Doc
  ### Data Model (Rust Structs & Traits)
  - `Inbox`: Represents a collection channel for a specific tenant.
  - `Conversation`: A continuous thread between a `Contact` and the `Inbox`.
  - `Message`: Individual message payloads (text, attachments, agent-drafted replies).
  - `Contact`: Represents the customer end of a conversation.
  - `ChannelAdapter` (Trait): Defines methods for sending/receiving messages (e.g., `TwilioAdapter`, `MetaAdapter`).

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : manages
    INBOX ||--o{ CONVERSATION : contains
    CONTACT ||--o{ CONVERSATION : participates_in
    CONVERSATION ||--o{ MESSAGE : contains
    TENANT ||--o{ CONTACT : owns

    TENANT {
      uuid id PK
    }
    INBOX {
      uuid id PK
      uuid tenant_id FK
      string name
      string channel_type
    }
    CONTACT {
      uuid id PK
      uuid tenant_id FK
      string name
      string identifier
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
      text content
      string message_type
    }
  ```

  ### Architecture Flow Diagram
  ```mermaid
  sequenceDiagram
    participant Webhook as External Webhook (Meta/Twilio)
    participant API as OHC API Gateway
    participant Handler as Rust Channel Adapter
    participant DB as Postgres DB (RLS Enabled)
    participant WS as WebSocket Service
    participant AI as AI Customer Assistant (Background Job)
    participant Flutter as Flutter Frontend

    Webhook->>API: Incoming Message Payload
    API->>Handler: Normalize to Message Struct
    Handler->>DB: Insert Message (tenant validated)
    Handler->>WS: Push new message to relevant tenant clients
    WS->>Flutter: Update Unified Inbox UI
    Handler->>DB: Enqueue AI Agent drafting job (SKIP LOCKED)
    DB-->>AI: Dequeue job
    AI->>DB: Read Conversation Context
    AI->>DB: Insert Draft Message / Action Intent
    AI->>WS: Push update (draft status)
    WS->>Flutter: Show draft to owner (Maya)
  ```

  ### Architecture
  - **Ingestion**: Webhooks from providers (Meta, Twilio) hit Rust handlers.
  - **Processing**: Rust handlers normalize payloads into `Message` structs and append to `Conversation`.
  - **Real-time**: WebSockets push updates to the Flutter frontend for live chat.
  - **AI Agent Integration**: A background job queue (PostgreSQL `SKIP LOCKED`) triggers the Customer Assistant AI to draft replies or triage intent for new messages.

  ### Mobile UX Flow
  - The OHC app features a unified "Inbox" tab accessible easily on a 375px wide screen without horizontal scroll.
  - Conversations show the source icon (Instagram, Web, SMS).
  - Agent-drafted replies appear with a distinct visual indicator (e.g., a "Draft" translucent glass tag) allowing the owner to approve/edit before sending.
  - Touch targets are large (>44px) for easy tapping while on the go.

  ## Implementation Prompt
  Implement the core native Rust backend for the OHC Omnichannel Chat System.
  1. Define the core data models (`Inbox`, `Conversation`, `Message`, `Contact`) in Rust with strict `tenant_id` validation.
  2. Implement a mockable `ChannelAdapter` trait and a basic `WebWidget` implementation.
  3. Create the API endpoints (gRPC/REST) for the frontend to fetch conversations and send messages.
  4. Ensure robust integration with the existing background job queue for asynchronous AI processing.
  5. The system must support the 'Maya' persona seamlessly responding to an Instagram DM via the unified interface.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
