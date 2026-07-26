issue_title: "Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat (Chatwoot Replacement)

  ## Problem Statement
  OHC requires a high-performance, real-time, multi-tenant omnichannel customer support & chat engine. The current system relied on Chatwoot as an external third-party service/dependency, which violates the strict architectural mandate (Chatwoot is 100% RETIRED). We need to build a native Rust implementation inside `onehumancorp/mono` that achieves feature parity with Chatwoot's omnichannel messaging capabilities without relying on any external services. This will ensure seamless integration with our Zero-Trust architecture (SPIFFE/SPIRE) and multi-tenant PostgreSQL (Row-Level Security) models.

  ## Research Report
  - **Chatwoot Codebase Audit**: An analysis of `https://github.com/chatwoot/chatwoot` reveals a robust data model including `Conversation`, `Message`, `Inbox`, `Contact`, `ChannelAdapter`, `Webhook`, `AgentBot`, and `SlaPolicy`. The real-time messaging heavily uses WebSockets.
  - **Competitor Systems**: Shopify Inbox, Zendesk, and Intercom all provide unified omnichannel inboxes (email, SMS, web widget, social media DMs). OHC must similarly unify these streams into a single timeline for the owner (Work Triage capability).
  - **OHC Architecture Fit**: Building this natively in Rust leverages our existing high-performance Agent infrastructure, Redis for cross-agent coordination/pub-sub, and PostgreSQL for strongly-typed, tenant-isolated data storage.

  ## Design Doc
  ### Data Model & Multi-Tenancy
  - **Core Entities**:
    - `Inbox`: Represents a specific communication channel (e.g., "Web Widget", "Instagram DM", "Email Support").
    - `Contact`: Represents the customer/lead interacting with an inbox.
    - `Conversation`: A thread of messages between a Contact and an Inbox, managed by Agents (human or AI).
    - `Message`: Individual messages within a Conversation, with attachments, rich media, and AI-drafted previews.
    - `ChannelAdapter`: Interfaces for integrating external platforms (WhatsApp, SMS, etc.).
  - **Multi-Tenant Isolation**: All tables MUST include a `tenant_id` column and enforce `ENABLE ROW LEVEL SECURITY`. Foreign keys must always include `tenant_id` to prevent cross-tenant leakage.
  - **Real-time Sync**: Use Redis Pub/Sub (`ohc:pubsub:chat:{tenant_id}:{inbox_id}:{conversation_id}`) to push real-time events (new message, typing indicator, read receipt) to connected WebSocket clients.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Tenant ||--o{ Conversation : "owns"
      Inbox ||--o{ Conversation : "receives"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string channel_type
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          boolean is_ai_draft
      }
  ```

  ### Mobile UX Flow
  - **375px First**: The Inbox view must prioritize a unified message stream on mobile.
  - **Swipe Actions**: Quick triage actions (archive, resolve, assign to AI agent) accessible via swipe gestures.
  - **Smart Replies**: AI-drafted replies (Customer Assistant) should appear seamlessly above the native keyboard, requiring a single tap for owner approval.
  - **Context Panel**: Tapping a contact's avatar reveals past orders, lifetime value, and custom notes in a slide-over panel.

  ### AI Agent Integration
  - **Work Triage Agent**: Automatically categorizes incoming messages, prioritizes them based on SLA or sentiment, and drafts responses.
  - **Customer Assistant**: Listens to the `Conversation` stream and injects drafted messages for owner review, utilizing tenant-scoped memory to recall past interactions.
  - **Handoff Protocol**: Clear observable states indicating whether a conversation is handled by an AI AgentBot, waiting for human action, or resolved.

  ## Implementation Prompt
  **Goal**: Implement the core data models, Rust gRPC/REST service layer, and real-time WebSocket infrastructure for the new Native Rust Omnichannel Chat system, achieving parity with Chatwoot's fundamental `Inbox`, `Conversation`, and `Message` entities.

  **CUJ (Critical User Journey)**:
  1. Owner logs into OHC and opens the Unified Inbox.
  2. A new customer (Contact) sends a message via the Web Widget (Inbox).
  3. The message appears in real-time in the Owner's Inbox view via WebSocket.
  4. The AI Customer Assistant drafts a suggested reply based on context.
  5. The Owner taps to approve the reply, which is sent back to the customer instantly.

  **Acceptance Criteria**:
  - Rust microservice with Protobuf definitions for `Inbox`, `Conversation`, `Message`, and `Contact`.
  - PostgreSQL schema with strict Row-Level Security (RLS) enforcing `tenant_id` isolation.
  - WebSocket gateway utilizing Redis Pub/Sub for real-time bidirectional message delivery.
  - Unit test coverage MUST be 100% for all new Rust code.
  - End-to-End Playwright test demonstrating the CUJ with a mocked Web Widget client connecting to the real WebSocket backend.
  - No external dependencies on Chatwoot or similar services.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
