issue_title: "Build Native Rust Omnichannel Customer Identity & Chat Engine (Replacing Chatwoot)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual owner responses.
  Furthermore, OHC previously relied on Chatwoot as an external service. Chatwoot has been 100% RETIRED as a dependency. We must now implement its core capabilities (Inbox, Conversation, Message, Omnichannel adapters) natively in Rust inside `onehumancorp/mono` so that The Ambassador agent can natively query the graph and draft responses.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat but relies heavily on manual responses or basic rigid auto-replies. Does not proactively draft contextual responses.
  - **Wix Inbox:** Good aggregation but limited AI features (mostly "improving tone").
  - **Chatwoot Source Code Audit:** Chatwoot uses a robust polymorphic `Channel` model (e.g. `Channel::Email`, `Channel::Whatsapp`, `Channel::Instagram`), which belongs to an `Inbox`. `Inbox` has many `Conversations`, which have many `Messages`. We need to replicate this core relational structure in PostgreSQL (with tenant-level RLS) and the native Rust backend.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / SMS] -->|Webhook| B(Omnichannel Gateway - Rust Axum)
      B --> C{Identity Resolution Engine}
      C -->|Lookup/Create| D[(PostgreSQL: Contacts & Inboxes)]
      C --> E[Conversation Manager]
      E --> F[(PostgreSQL: Conversations & Messages)]
      F --> G[Redis Pub/Sub Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|RAG against tenant catalog| I[Draft Reply]
      I --> J[Action Required Queue]
      J --> K[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow
  - **Home Feed (Mobile 375px):** Card shows "1 New Message from Sarah (Insta DM)".
  - **Action Card:** Tapping the card opens a unified view showing customer context (e.g. past orders) on top, and an AI-drafted reply on the bottom.
  - **Actions:** "Approve", "Edit", "Discard".
  - **Design:** Glassmorphism cards, blurred backgrounds, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered via Redis Pub/Sub when a new message is inserted. Uses RAG against the tenant's product catalog and past orders to draft a personalized reply, saving it to the `Action Required` queue rather than sending immediately.

  ### Key Design Decisions
  - **100% Native Rust:** Implement the Chatwoot `Inbox`, `Channel`, `Conversation`, and `Message` models in Rust using sqlx and PostgreSQL.
  - **Multi-Tenant RLS:** All tables must include `tenant_id` and enforce Row-Level Security.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, when a customer DMs me on Instagram, I open the OHC app to find a pre-written, perfectly accurate response already drafted based on their past history. I tap one button to approve it.
  **CUJ & Acceptance Criteria:**
  1. Implement PostgreSQL schema migrations for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with `tenant_id` and RLS.
  2. Build a Rust Axum webhook endpoint (`/api/v1/webhooks/omnichannel`) that accepts incoming messages and routes them to the appropriate channel handler.
  3. The handler must resolve the contact identity, create/find a conversation, and insert the message.
  4. Trigger a Redis Pub/Sub event upon message insertion.
  5. Provide Playwright E2E tests: A user logs in, sees a drafted message card (simulated by a webhook payload) on the mobile feed, taps "Approve", and the system records the approved message.

  # Priority: P0 (Critical - Chatwoot Replacement)
  # Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
