issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels (WhatsApp, Instagram, Web Widget, Email). Managing these manually leads to missed messages and lost sales. Currently, there is an architectural mandate to completely retire the external dependency on Chatwoot and replace it with a high-performance, native Rust omnichannel chat system inside the OHC monorepo. This native engine must enforce strict multi-tenant isolation, deeply integrate with our AI work assistants, and operate flawlessly on a 375px mobile screen.

  # Research Report
  **Findings & Source Code Benchmarking (Chatwoot):**
  - Cloned and audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`).
  - **Core Data Models:** Identified crucial entities such as `Conversation` (tracks `account_id`, `inbox_id`, `status`, `assignee_id`, `contact_id`) and `Message` (tracks `content_type`, `message_type`, `sender_id`). These models must be ported to Rust structs and PostgreSQL schemas.
  - **Channel Adapters:** Chatwoot relies heavily on polymorphic channels (`app/models/channel/whatsapp.rb`, `web_widget.rb`, `instagram.rb`). Our Rust system will implement dedicated crates for each channel adapter.
  - **Real-Time Engine:** Chatwoot uses ActionCable. OHC's implementation will leverage an asynchronous Rust WebSocket server connected to our Redis (Valkey) event mesh for lightning-fast, horizontal broadcast of `message.created` and `conversation.updated` events.
  - **OHC Integration Opportunity:** Natively integrating this system allows "The Ambassador" (Customer Success Agent) to intercept incoming messages, query the tenant's unified product catalog and customer history, and proactively draft a context-aware response, fundamentally changing the owner's workflow from read-reply to read-approve.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram / WhatsApp / Web Widget] -->|Webhooks/WS| B(Rust Omnichannel Gateway)
      B --> C{Identity Resolution Engine}
      C -->|Match/Create Contact| D[(Unified Tenant DB RLS)]
      C --> E[Redis / Valkey Event Mesh]
      E --> F[WebSocket Broadcast Hub]
      F --> G[Mobile Client 375px]
      E --> H[The Ambassador Agent]
      H -->|Context Query & Draft Reply| D
      D -.->|Update| F
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top card displays unified pending actions, e.g., "1 New Message from Sarah (WhatsApp)".
  - **Interaction:** Tapping the card opens a unified conversation view. The top half shows customer context (e.g., Sarah's last booking/purchase). The bottom half shows an AI-drafted reply ("Hi Sarah! Are you looking to rebook for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** macOS-style Translucent Glass materials, clear hierarchy, native keyboard integration when editing.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by incoming messages published to the event mesh. Uses RAG against tenant-scoped context to draft highly personalized replies and stages them in the `ActionRequiredQueue`.
  - **The Manager:** If the inquiry requires calendar/inventory checks, The Ambassador coordinates with The Manager seamlessly via internal agent protocols.

  ### Key Design Decisions
  - **Strict Multi-Tenancy:** Row-Level Security (RLS) via `tenant_id` must be enforced on every new table (`inboxes`, `conversations`, `messages`, `contacts`).
  - **High Performance Async I/O:** The Webhook Gateway and WebSocket hub will be built using Rust's async ecosystem (e.g., Axum/Tokio) to handle high concurrency with low overhead.
  - **Proactive Drafting:** Move from reactive messaging to proactive approval. The owner should rarely type a full message from scratch.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, when a customer texts me on WhatsApp, I open the OHC app and see a fully drafted, context-aware reply waiting for my approval. I tap "Send Draft" and the conversation is resolved in 2 seconds.

  **CUJ & Acceptance Criteria:**
  1. Build the Rust data models, migrations, and repository layer for `Inbox`, `Conversation`, `Message`, and `Contact` in `src/server/domain/omnichannel` (or similar), strictly enforcing `tenant_id` isolation.
  2. Implement the `Rust Omnichannel Gateway` to ingest simulated webhooks and store incoming messages.
  3. Implement the WebSocket endpoint that broadcasts new messages to the mobile frontend.
  4. Integrate with The Ambassador Agent: on new message ingestion, trigger a background worker that drafts a reply and updates the conversation state.
  5. Provide complete Playwright E2E tests: A user logs in, receives a mocked incoming webhook message, views the drafted reply on a 375px viewport, taps "Approve", and the system records the outgoing dispatch.
  6. Ensure 100% unit test coverage for all new Rust modules. `bazel test //...` must remain green.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
