issue_title: "Implement Native Rust Omnichannel Inbox & Chat Engine"
issue_description: |
  ## Problem Statement
  Currently, Maya (the home baker) and Carlos (the field service owner) are struggling to manage customer communications across different platforms (Instagram DMs, WhatsApp, SMS, Web Chat). They often miss messages, lose context, or cannot respond quickly enough when they are out making deliveries or performing services. They need a single, unified inbox that runs reliably on their mobile devices (even on slow networks), where AI can automatically draft replies or handle routine inquiries, freeing them up to focus on their actual business. Relying on an external third-party service like Chatwoot introduces latency, breaks offline capabilities, and fragments the unified owner experience.

  ## Research Report
  - **Market Context**: Platforms like Shopify Inbox, WeCom, and Wix Inbox offer integrated chat solutions, but they often lack deep operational integration (e.g., turning a chat message directly into a service booking or an inventory-aware quote).
  - **Chatwoot Source Audit**: An audit of `chatwoot/chatwoot` source code reveals a robust omnichannel architecture using core models: `Account` (Tenant), `User` (Agent/Owner), `Inbox`, `Channel::*` (adapters for Facebook, Twitter, WhatsApp, etc.), `Conversation`, `Message`, and `Contact`. It relies on WebSocket for real-time updates and background queues for processing webhooks.
  - **Conclusion**: We must build a Native Rust Omnichannel Chat system within OHC. This ensures 100% data locality, allows row-level security per tenant in PostgreSQL, and enables zero-latency AI agent context sharing.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Unified Inbox Screen**: A single feed of conversations across all channels. Unread messages are bolded. Urgent AI-flagged messages pin to the top.
  2. **Conversation Thread Screen**: Chat bubbles. Native mobile keyboard support. A "Generate Draft" AI action button floats above the input field.
  3. **Action Context Sheet**: Swiping left on a message opens a quick-action sheet to "Create Quote", "Book Service", or "Mark as Paid", deeply integrating chat with operations.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Listens to the `conversation.created` and `message.created` internal events. It pulls `Contact` history, reads the `Message` payload, and generates a draft reply stored as an `AgentDraft` pending owner approval.
  - **Work Triage**: Analyzes sentiment and intent of incoming messages. If a message implies a booking request, it routes a signal to the Operations Assistant.

  ### Key Design Decisions
  - **Row-Level Security**: Every table (`inboxes`, `conversations`, `messages`, `contacts`) must have a `tenant_id` enforced by PostgreSQL RLS.
  - **Native Rust**: Implement the backend in Rust (`src/server/ohc/domain/inbox`) leveraging Axum for REST/WebSockets and Tokio for async event processing.
  - **Zero Trust**: Webhooks from external providers (Meta, Twilio) must be cryptographically verified before hitting the internal job queue.

  ## Implementation Prompt
  **Goal**: Implement the core data models and service layer for the native Rust omnichannel inbox.
  **CUJ**: Maya receives an Instagram DM. The webhook is ingested, a `Conversation` and `Message` are created in the database under her `tenant_id`, and a real-time WebSocket event is emitted to her mobile app. She views the unified inbox, sees the message, and can read it without leaving the OHC app.
  **Acceptance Criteria**:
  1. Define Rust structs and Diesel/SQLx schemas for `Inbox`, `ChannelAdapter`, `Conversation`, `Message`, and `Contact`.
  2. Enforce `tenant_id` isolation.
  3. Implement the REST endpoints to list conversations and fetch messages for a given inbox.
  4. Write comprehensive unit tests and a Playwright E2E test verifying a user can see a mocked (via test adapter) incoming message in their unified feed.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
