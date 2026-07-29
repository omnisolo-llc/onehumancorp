issue_title: "[Research] Architect Native Rust Omnichannel Inbox to Replace Platform"
issue_description: |
  # Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) receive customer inquiries across unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages and lost sales. Traditional solutions aggregate messages without context. OHC previously relied on Platform for its inbox, but Platform as an external dependency is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust to achieve 100% feature parity with Platform, while deeply integrating with the OHC Agentic workflow (The Ambassador agent).

  # Research Report
  **Findings & Competitive Analysis:**
  - **Platform Source Code Audit**: Benchmarked `https://github.com/platform_old/platform_old`. Platform uses a Rails monolith with models like `account`, `inbox`, `conversation`, `message`, `contact`, and `channel_*` (adapters for email, API, WhatsApp, etc.). It uses WebSockets for real-time updates and heavily relies on PostgreSQL.
  - **The Native Rust Opportunity**: Building this natively in Rust inside `onehumancorp/mono` guarantees strict multi-tenant row-level security (RLS) out-of-the-box, significantly lower latency (critical for 375px mobile responsiveness), and unified observability. It eliminates a massive third-party operational dependency.
  - **Agentic Integration**: Unlike Platform, the native OHC inbox will natively route incoming messages to the "The Ambassador" AI agent, which queries the customer's unified identity graph (purchase history, past bookings) to proactively draft a complete, accurate response.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - Rust)
      B --> C{Tenant Router & Auth}
      C --> D[(PostgreSQL - Unified Graph DB)]
      C --> E[Event Mesh]
      E --> F[The Ambassador Agent]
      E --> G[WebSocket PubSub - Rust]
      G --> H[Mobile App Feed 375px]
      F -->|Draft Reply| I[Action Required Queue]
      I --> H
      H -->|1-Tap Approve| J[Omnichannel Dispatcher - Rust]
      J --> A
  ```

  ### Mobile UX Flow (375px First)
  1. **Omnichannel Feed**: The owner opens the app and sees a unified "Work Triage" feed.
  2. **Message Card**: "1 New Message from Sarah (Insta DM)".
  3. **Contextual View**: Tapping the card opens the conversation. It displays the message history, Sarah's customer profile (past orders, LTV), and a pre-drafted AI response from The Ambassador.
  4. **Action**: The owner taps "Approve & Send", "Edit", or "Dismiss". The response is routed natively through the Rust dispatcher to the correct channel.

  ### Data Model & Invariants
  - `Tenant` (Account)
  - `Inbox` (Channel configuration: email, IG, WA, API)
  - `Contact` (Customer profile)
  - `Conversation` (Thread between Contact and Inbox)
  - `Message` (Individual message, supports attachments)
  - **Invariants**: Strict row-level security (RLS) on `tenant_id` for all tables. All API and WebSocket connections must be validated via SPIFFE/SPIRE identity.

  # Implementation Prompt
  Implement the core native Rust Omnichannel Inbox data model and CRUD APIs to replace Platform.
  - **CUJ**: A small business owner connects an inbox (e.g., API channel), receives a message from a customer, and views the conversation in a unified 375px mobile UI.
  - **Requirements**:
    1. Define the PostgreSQL schemas for `inboxes`, `conversations`, `messages`, and `contacts` with `tenant_id` RLS.
    2. Build the Rust gRPC/REST APIs to create inboxes, start conversations, and send/receive messages.
    3. Implement a basic React/Flutter 375px mobile-first UI for the "Work Triage" feed to display these conversations.
    4. Ensure the system is ready to emit events to The Ambassador agent for auto-drafting replies.
  - **Acceptance Criteria**: The owner can see incoming messages from an API channel in the OHC feed, and all tests (`bazel test //...`) pass. Zero mock data in the UI.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
