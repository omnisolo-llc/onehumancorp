issue_title: "[Architect] Implement Native Rust Omnichannel Inbox (Legacy Chat Replacement)"
issue_description: |
  # Problem Statement
  Small business owners need a unified inbox to manage customer interactions across WhatsApp, Instagram DMs, Email, and Web Chat. Previously, OHC relied on an external third-party chat integration. This is now 100% RETIRED. We must implement a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside `onehumancorp/mono`. Relying on external vendors causes data silos, high latency for our AI agents (who need real-time stream access to messages to auto-draft replies), and breaks our Zero Trust isolation model.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Legacy Source Audit:** The prior external system used a Rails monolith with complex models for Inbox, Conversation, Message, and Channels. It used WebSockets for real-time updates and heavily relied on background jobs for webhooks and agent routing. We have reviewed its architecture and are moving to a native model.
  - **OHC Native Rust Approach:** We need to replicate the core conceptual entities (Conversations, Messages, Inboxes, Contacts) natively in our Rust backend, utilizing PostgreSQL with strict Row Level Security (RLS) for multi-tenancy.
  - **AI Integration (The Ambassador):** Unlike external systems where AI is an afterthought or plugin, OHC's native chat engine must have AI built-in. Every incoming message triggers an event on our internal message bus (NATS or Redis Pub/Sub), allowing The Ambassador agent to immediately read the message, query the customer's history, and draft a reply *before* the owner even opens the app.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhook] -->|Ingest| B(Rust API Server Webhook Endpoint)
      B --> C{Channel Adapter}
      C --> D[Create/Update Conversation & Message in DB]
      D --> E[Redis Event Bus]
      E --> F[The Ambassador Agent Auto-Drafter]
      F -->|Draft Reply| D
      E --> G[WebSocket Server]
      G -->|Push Update| H[Flutter Mobile Client 375px]
      H -->|Owner Approves Draft| I[Rust API Message Send Endpoint]
      I --> J[Channel Adapter]
      J -->|Send| K[External Service]
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean list of active conversations, sorted by last activity. Unread messages and "Draft Ready" indicators are prominent.
  - **Conversation Thread View:** Standard chat interface. If The Ambassador drafted a reply, it appears in a translucent glass card at the bottom above the keyboard with two buttons: "Send" (primary) and "Edit" (secondary).
  - **Zero-Jargon:** No terms like "Channels," "Inboxes," or "Macros" on the main screen. Just "Messages".

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the message creation event. It reads the Conversation context, looks up the Contact in the CRM, queries the knowledge base/inventory, and inserts a new Message with a draft status.

  ### Key Design Decisions
  - **Rust Native:** Complete removal of any external legacy chat dependency. All models live in the OHC Postgres DB.
  - **Row Level Security (RLS):** Every table MUST have `tenant_id` and RLS enabled to guarantee data isolation.
  - **Event-Driven AI:** The AI agent acts as just another participant in the system, listening to the event bus rather than polling.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens the "Messages" tab on their phone and sees a unified list of customer chats. They can tap into a chat, see a perfectly AI-drafted reply to a customer's WhatsApp message, and hit "Send" in one tap.
  **CUJ & Acceptance Criteria:**
  1. Define and run migrations for the native chat data models (inboxes, contacts, conversations, messages) with strict `tenant_id` RLS. Let the implementer determine the exact schema.
  2. Implement Rust API endpoints for CRUD operations on these entities. Let the implementer determine the exact API paths and function signatures.
  3. Implement a webhook ingestion endpoint that can receive a real external payload, identify the correct tenant and inbox, and create a message record.
  4. Ensure a new message emits an event that the real AI service (The Ambassador) can intercept to create a draft message.
  5. Provide Playwright E2E tests: A test user logs in, navigates to the Messages screen, sees a real conversation populated from the test-mode adapter, and can send a new message. No mock data is allowed in the UI or E2E tests.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
