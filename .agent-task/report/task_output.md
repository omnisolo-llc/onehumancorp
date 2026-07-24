issue_title: "Native Rust Omnichannel Chat System Replication (Chatwoot Retirement)"
issue_description: |
  ## Problem Statement
  Small business owners and operators (Maya the baker, Carlos the handyman, Fatima the food cart owner) manage communications across Instagram DMs, SMS, email, and WhatsApp. Currently, relying on third-party integrations like Chatwoot fractures the experience, complicates multi-tenant isolation, and creates integration latency that degrades AI response capabilities. We need to fully retire the Chatwoot external dependency and build a native, high-performance Rust omnichannel chat system inside `onehumancorp/mono`. This will allow our AI agents (like The Ambassador) to proactively resolve customer issues seamlessly.

  ## Research Report
  **Findings & Chatwoot Source Audit:**
  - Audited `https://github.com/chatwoot/chatwoot`. Key legacy components identified:
    - **Models:** `Conversation`, `Message`, `Inbox`, `Contact`, `Channel`.
    - **Architecture:** Ruby on Rails, heavy PostgreSQL reliance for JSONB unstructured metadata, Sidekiq for background workers.
    - **Channels:** Email, API, Widget, Twitter, Facebook, Line, WhatsApp, SMS.
  - **OHC Native Rust Opportunity:**
    - Replicate the core schema (`inboxes`, `conversations`, `messages`, `contacts`) directly in our PostgreSQL schema with our strict row-level security `tenant_id` multi-tenancy requirements.
    - Replace Sidekiq with our existing `SKIP LOCKED` Postgres queue or NATS JetStream.
    - Expose WebSocket and gRPC channels via our existing Tokio/Tonic stack.
    - Eliminate third-party API keys and latency for internal unified inbox reads/writes.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG, WA, Email] --> B[Rust Omnichannel Gateway]
      B --> C{Tenant Identity Resolver}
      C --> D[Native Rust Inbox Service]
      D --> E[PostgreSQL Unified Graph]
      D --> F[Redis Pub/Sub WebSocket Notifier]
      F --> G[OHC Mobile App 375px]
      E -.-> H[The Ambassador AI Agent]
      H -.->|Drafts Replies| D
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Feed (Mobile):** The owner sees a single feed. A card says "Instagram DM from @sarah."
  - **Interaction:** Tapping opens a glassmorphism detailed view. The AI Ambassador has pre-drafted a reply based on Sarah's past cake orders and current inventory.
  - **Action:** Owner taps "Approve" (large 44x44px target) to dispatch the response.

  ### AI Agent Integration Points
  - The Native Rust Inbox service emits events to the Agent Job Queue when a new message arrives.
  - "The Ambassador" reads the conversation thread and the unified contact graph (orders, bookings) and writes a `DRAFT` status message back to the conversation.

  ### Key Design Decisions
  - **100% Rust Native:** No external Chatwoot Ruby services.
  - **Strong Multi-tenancy:** All tables must enforce `tenant_id` at the RLS level.
  - **AI-First:** The database schema must distinguish between `AI_DRAFT`, `HUMAN_SENT`, and `AI_SENT` message states.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya receives an Instagram DM. She opens OHC and immediately sees the pre-drafted reply inside the native OHC interface. She taps send without knowing she is using a complex omnichannel system.
  **CUJ & Acceptance Criteria:**
  - Create the required database schemas for Inboxes, Conversations, Messages, and Contacts in the Rust backend.
  - Expose internal APIs (gRPC or REST) for the frontend to render the unified inbox.
  - Ensure the AI agent can subscribe to new messages and write drafts.
  - Implement Playwright E2E tests verifying an owner can view a conversation, see the AI draft, and tap "Approve" to update the message status.
  - The UI MUST NOT contain mock data. Ensure E2E tests seed real conversations via the backend.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
