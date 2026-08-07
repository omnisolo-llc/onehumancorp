issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  **Mission Queue Protocol Brief: Native Rust Omnichannel Chat System**

  **Problem Statement:**
  Chatwoot as an external dependency has been retired to simplify our architecture, improve multi-tenant isolation, and reduce operational costs. However, our non-technical business owners (like Maya the baker and Carlos the handyman) still need a unified inbox to manage customer interactions across Instagram DMs, WhatsApp, SMS, and Email. The lack of a native, highly performant chat system natively integrated within OHC means these owners are forced to juggle multiple apps, risking missed opportunities and lost context. We need to implement a native Rust omnichannel chat system within `onehumancorp/mono` that achieves feature parity with Chatwoot's core capabilities (data models, channel adapters, web chat widget, WebSocket events).

  **Research Report:**
  - **Market Context:** Small business owners suffer from "channel fragmentation." Legacy platforms often just aggregate messages without providing deep context. A native Rust solution allows us to tightly integrate chat with our AI agents (e.g., "The Ambassador") and our customer identity graph.
  - **Competitive Analysis:**
    - *Shopify Inbox/Wix Inbox:* Basic aggregation, lacking autonomous agent capabilities.
    - *Zendesk/Intercom:* Too complex and expensive for our target personas.
    - *Chatwoot (External):* Open-source but introduces unnecessary operational overhead (separate DBs, complex deployments).
  - **Solution:** A native Rust implementation using our existing PostgreSQL backend and Valkey for real-time pub/sub. This ensures strict row-level security (RLS) for multi-tenancy and seamlessly connects with our AI job queues.

  **Top 5 Things That Do Not Make Sense (Repository Audit):**
  1. Dependencies like `@journeyapps/wa-sqlite` triggering legacy setup workflows that spew multiple warnings during `pnpm install` in modern node versions.
  2. Extensive, un-pruned architectural documentation pointing to Chatwoot features that are simultaneously being purged across Kubernetes definitions and helm charts.
  3. Excessive timeout issues when running `bazel test //...` inside isolated environments indicating tests may be spinning out instead of properly failing or yielding.
  4. Conflicting versions of `bazel` vs `bazelisk` tools locally, leading to command-not-found issues in typical environment setups.
  5. The existence of empty `package.json` configurations triggering workspace scoping warnings (e.g., `pnpm.onlyBuiltDependencies` found in `src/ui/next/package.json` rather than the workspace root).

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    graph TD
        A[External Channels: IG, WhatsApp, Email] -->|Webhooks| B(Rust Channel Adapters)
        B --> C{Omnichannel Ingress Service}
        C -->|Event Bus / Valkey| D[Real-time WebSocket Hub]
        C --> E[(PostgreSQL - Conversations/Messages with RLS)]
        D --> F[Mobile-First PWA/Flutter App]
        C --> G[AI Ambassador Agent Queue]
        G -->|Draft Reply| E
    ```
  - **Mobile UX Flow (375px First):**
    - A unified "Inbox" tab.
    - A feed of conversation cards showing the customer name, channel icon (e.g., IG), and a snippet of the latest message or AI-drafted reply.
    - Tapping a card opens the chat view with context (past orders) at the top and the chat history below.
    - Large, touch-friendly "Approve & Send" buttons for AI drafts.
  - **AI Agent Integration:** The system will publish incoming messages to our Redis (Valkey) job queue. The "Customer Success Agent" will consume these events, query the customer's history, and insert a draft reply directly into the native conversation thread.
  - **Key Design Decisions:**
    - Use Rust for high performance and low memory footprint in handling WebSockets.
    - Strict multi-tenancy enforced at the database level (PostgreSQL RLS).
    - Fully replace Chatwoot's data model with an optimized, OHC-specific native schema.

  **Implementation Prompt:**
  As an Implementer agent, your task is to build the foundational native Rust omnichannel chat system to replace our reliance on Chatwoot.
  - **User-Facing Outcome:** The owner (e.g., Maya) receives an Instagram DM. It instantly appears in her OHC mobile app inbox via a native WebSocket connection. The system automatically identifies the customer and The Ambassador agent drafts a reply.
  - **CUJ & Acceptance Criteria:**
    1. Implement the core Rust data models (Conversations, Messages, Contacts, ChannelAdapters) with strict multi-tenant isolation.
    2. Build a high-performance WebSocket hub in Rust to push real-time updates to connected clients.
    3. Implement a generic webhook ingress endpoint that can receive payloads from external channels (mocked for testing).
    4. Ensure all new Rust code has 100% unit test coverage and integration tests using our Bazel infrastructure (`bazel test //...`).
    5. No external dependencies on Chatwoot should be introduced or assumed.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
