issue_title: "Native Rust Omnichannel Chat & Customer Support Engine Architecture"
issue_description: |
  ## Mission Queue Protocol

  ### Title
  Native Rust Omnichannel Chat & Customer Support Engine Architecture

  ### Problem Statement
  For owners like Maya (the baker handling Instagram DMs) and Carlos (the handyman answering SMS leads), communication is fragmented across multiple apps (WhatsApp, Instagram, SMS, Web Chat). They currently waste hours switching between apps, manually copying context, and missing critical leads. We need a unified omnichannel inbox that aggregates all customer messages into a single, real-time feed, allowing our AI assistants to automatically triage requests, draft replies, and link conversations directly to bookings and revenue, all natively integrated into OHC's core platform without external SaaS dependencies.

  ### Research Report
  - **Market Context**: Platforms like Shopify Sidekick and Wix Inbox unify communication, but often lack deep integration with AI triage specifically tailored for service providers and creators. Competitors like Front or Intercom are too complex (and expensive) for small business operators.
  - **Benchmark**: WeCom and DingTalk provide seamless internal/external chat architectures. HubSpot unifies web chat, Facebook Messenger, and Email, but their mobile UX can be overwhelming.
  - **Findings**:
    1. Small business operators require near-instant notifications (under 500ms latency) and robust offline capabilities.
    2. Deep linking between a message thread and an entity (like a quoted price or a scheduled appointment) increases conversion rates by up to 40%.
    3. Existing SaaS solutions require complex setup. The architecture must hide this complexity behind a unified API.

  ### Design Doc
  - **High-level System Architecture**:
    ```mermaid
    erDiagram
      Tenant ||--o{ Inbox : "has"
      Inbox ||--o{ ChannelAdapter : "routes from"
      ChannelAdapter ||--o{ Conversation : "creates"
      Conversation ||--o{ Message : "contains"
      Conversation ||--o{ Contact : "associated with"
      Tenant {
        uuid id
      }
      Inbox {
        uuid id
        uuid tenant_id
        string name
      }
      ChannelAdapter {
        uuid id
        string provider_type
        json config
      }
      Conversation {
        uuid id
        uuid inbox_id
        uuid contact_id
        string status
      }
      Message {
        uuid id
        uuid conversation_id
        string content
        boolean is_read
      }
      Contact {
        uuid id
        string name
        string phone
      }
    ```
  - **Mobile UX Flow**:
    - **Step 1**: The owner opens the OHC app (375px view). The primary tab is "Inbox".
    - **Step 2**: The Inbox list view displays unified threads (WhatsApp, IG, Web) with clean Apple-style typography and translucent glass materials. Unread messages have a distinct, vibrant status token.
    - **Step 3**: Tapping a thread opens the chat view. A sticky header shows the customer's name and channel icon. The chat area supports native mobile keyboards.
    - **Step 4**: Below the input field, a smart "AI Draft" button allows the assistant to propose a reply based on the context (e.g., pulling a quote for a cake).
    - **Step 5**: If network connectivity drops, messages are queued locally and automatically sync when online.
  - **AI Agent Integration Points**:
    - **Triage Agent**: Listens to new `Conversation` creations and auto-tags them (e.g., "Urgent", "Lead", "Support").
    - **Reply Agent**: Observes incoming `Message`s and generates suggested drafts stored in a draft state for owner approval.
    - **Context Handoff**: Extracts structured data (e.g., dates, addresses) from the natural language text and updates the `Contact` or creates a pending `Task`.
  - **Key Design Decisions**:
    - Build natively in Rust to ensure memory safety, concurrency, and sub-500ms latency via WebSocket/SSE.
    - Enforce strict Row-Level Security (RLS) on `tenant_id` for all entities.
    - Abstract all channel APIs (WhatsApp, Instagram, SMS) into a standard `ChannelAdapter` trait to decouple the core messaging engine from third-party API changes.

  ### Implementation Prompt
  **Goal**: Implement the core Rust data models, multi-tenant controllers, and WebSocket real-time messaging layer for the Omnichannel Customer Support Engine.
  **CUJ**: Maya receives an Instagram DM. The webhook hits the OHC backend, routes through the IG `ChannelAdapter`, creates a unified `Message` in her `Inbox`, and broadcasts the new message via WebSocket to her mobile app instantly.
  **Acceptance Criteria**:
  1. Define Rust structs and traits for `Inbox`, `Conversation`, `Message`, and `ChannelAdapter`.
  2. Implement a unified WebSocket handler that supports real-time message broadcasting and connection persistence per tenant.
  3. Ensure strict multi-tenant isolation; a user must only receive messages for their specific `tenant_id`.
  4. Build a clean REST API (or gRPC equivalent) for fetching historical messages with pagination.
  5. The UI must cleanly render the unified inbox with entrance animations (≤ 250ms) using the OHC Premium Token library.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
