issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  Small business owners are bombarded with messages across various channels (Instagram DMs, WhatsApp, Email, Web Widget). Current generic solutions like Chatwoot just aggregate messages into a "Unified Inbox". This forces the owner to manually piece together context (purchase history, previous interactions) and manually draft replies, which doesn't scale.
  We need to replace the retired external Chatwoot dependency with a Native Rust implementation inside the `onehumancorp/mono` repository. This native system will seamlessly integrate with "The Ambassador" (Customer Success Agent) to automatically read messages, resolve customer identities, retrieve context, and draft highly accurate replies for the owner to 1-tap approve.

  ## Research Report
  - **Chatwoot Source Code Audit**: Reviewed `https://github.com/chatwoot/chatwoot` (`app/models/*`). Key findings:
    - **Data Models**: The core schema revolves around `Account` (tenant), `Inbox`, `Channel::*`, `Contact`, `Conversation`, and `Message`.
    - **Omnichannel Support**: Handled via polymorphic `Channel::*` models (e.g., `Channel::Whatsapp`, `Channel::WebWidget`).
    - **Identity Resolution**: Handled via `Contact` and `ContactInbox`.
  - **OHC Opportunity**: Implement these core models natively in Rust with strict Row Level Security (RLS) via `tenant_id`. We can bypass the complex agent assignment/routing logic of traditional helpdesks and directly route messages to the AI Agent ("The Ambassador") for drafting, presenting the owner with a unified feed of action-required items.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|Ingress| B(Channel Adapter Layer)
      C[Insta DM Webhook] -->|Ingress| B
      D[Web Widget WebSocket] <-->|Real-time| B
      B --> E[Message Ingestion Pipeline]
      E --> F{Identity Resolution}
      F -->|Lookup/Create| G[(Unified Graph DB - Contacts & Inboxes)]
      F --> H[Conversation Manager]
      H -->|Store| G
      H --> I[Event Bus - msgbus.rs]
      I --> J[The Ambassador Agent]
      J -->|Draft Reply| K[Action Required Queue]
      K --> L[Mobile App Feed]
  ```

  ### Data Model & Invariants
  The following core entities need to be implemented in Rust (with strict RLS `tenant_id` on all tables):
  1.  **`ChannelAdapter`**: Configuration for a specific channel (e.g., WhatsApp API token, Web Widget domain).
  2.  **`Inbox`**: Represents a specific intake point (e.g., "Main WhatsApp", "Support Email"). Linked to a `ChannelAdapter`.
  3.  **`Contact`**: A unique customer. Omnichannel identity resolution merges identifiers (phone, email, social handle) into a single contact.
  4.  **`ContactInbox`**: Maps a `Contact` to an `Inbox` with a channel-specific identifier.
  5.  **`Conversation`**: A thread of messages between a `Contact` and an `Inbox`.
  6.  **`Message`**: Individual messages (text, attachments, agent drafts).

  ### Mobile UX Flow (375px First)
  - The native chat system operates mostly invisibly in the backend.
  - The user-facing component is the **Action Feed**: The owner sees a card: "New WhatsApp message from Carlos. AI has drafted a reply."
  - **1-Tap Action**: The card shows the customer's intent and the drafted response. The owner taps "Approve" (dispatches via Channel Adapter) or "Edit" (opens native keyboard).
  - Web Widget: A lightweight, translucent glass-styled chat widget for the owner's storefront, powered by WebSocket.

  ### AI Agent Integration
  - When a `Message` is created, an event is published to `msgbus.rs`.
  - **The Ambassador** listens, retrieves the `Conversation` history and `Contact` context (orders, preferences), and generates a draft.
  - The draft is saved as a pending `Message` (type: draft) and surfaced to the owner.

  ### Security & Zero Trust
  - All DB queries must include `tenant_id`.
  - Webhook payloads must be signature-verified (e.g., Meta/WhatsApp webhook signatures).
  - Web Widget WebSocket connections must authenticate via short-lived JWTs.

  ## Implementation Prompt
  **User-Facing Outcome**: As an owner, I want all my WhatsApp and Web Widget messages to flow into my single OHC feed. Instead of typing replies, my AI assistant drafts the perfect response based on the customer's history, and I just tap "Approve" to send it instantly.
  **CUJ & Acceptance Criteria**:
  1.  Implement the core database schema (PostgreSQL) and Rust structs/queries for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` with strict `tenant_id` RLS.
  2.  Implement a Webhook ingestion endpoint (e.g., for WhatsApp) that creates a `Contact` (if new), `Conversation`, and `Message`.
  3.  Ensure an event is published to `msgbus.rs` upon message creation.
  4.  Provide Playwright E2E tests: Simulate an incoming webhook, verify the entities are created in the database, and verify the message appears in the UI (mocking the AI draft for now).

  **Priority**: P0 (critical)
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
